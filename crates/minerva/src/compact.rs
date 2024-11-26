use std::fmt::Display;

use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use crate::attribute_store::load_attributes;
use crate::error::Error;

pub struct CompactResult {
    pub record_count: u64,
    pub attribute_store_name: String,
    pub attribute_store_id: i32,
}

impl Display for CompactResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Compacted {} records for '{}'({})",
            self.record_count, self.attribute_store_name, self.attribute_store_id
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompactError {
    #[error("Could not find attribute store with Id {0}")]
    NoSuchAttributeStoreId(i32),
    #[error("Could not find attribute store with name '{0}'")]
    NoSuchAttributeStoreName(String),
    #[error("Unexpected error during compacting: {0}")]
    Unexpected(String),
}

impl From<CompactError> for Error {
    fn from(value: CompactError) -> Self {
        Error::Runtime(format!("Could not compact attribute data: {}", value).into())
    }
}

async fn get_last_compacted_id<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
) -> Result<i32, CompactError> {
    let query = "SELECT compacted FROM attribute_directory.attribute_store_compacted WHERE attribute_store_id = $1";

    let rows = client
        .query(query, &[&attribute_store_id])
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        Ok(0)
    } else {
        let row = rows.first().unwrap();

        let last_compacted_id: i32 = row.get(0);

        Ok(last_compacted_id)
    }
}

async fn get_max_history_id<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_history_table: &str,
) -> Result<i32, CompactError> {
    let query = format!(
        "SELECT MAX(id) FROM attribute_history.{}",
        escape_identifier(attribute_history_table)
    );

    let rows = client
        .query(&query, &[])
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        Ok(0)
    } else {
        let row = rows.first().unwrap();

        let max_history_id: Option<i32> = row.get(0);

        Ok(max_history_id.unwrap_or(0))
    }
}

pub async fn compact_attribute_store_by_id<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
    max_compact_count: i32,
) -> Result<CompactResult, CompactError> {
    let rows = client
        .query(
            "SELECT attribute_store::text FROM attribute_directory.attribute_store WHERE id = $1",
            &[&attribute_store_id],
        )
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        return Err(CompactError::NoSuchAttributeStoreId(attribute_store_id));
    }

    let row = rows.first().unwrap();

    let attribute_store_name = row.get(0);

    let last_compacted_id = get_last_compacted_id(client, attribute_store_id).await?;

    println!("Last compacted Id: {}", last_compacted_id);

    let max_history_id = get_max_history_id(client, attribute_store_name).await?;

    println!("Max history Id: {}", max_history_id);

    let max_to_compact = std::cmp::min(last_compacted_id + max_compact_count, max_history_id);

    println!("Max to compact: {}", max_to_compact);

    let attributes = load_attributes(client, attribute_store_id).await;

    let compacted_view_name = format!("{}_compacted", attribute_store_name);
    let compacted_tmp_table_name = format!("{}_compacted_tmp", attribute_store_name);

    let truncate_tmp_table_query = format!(
        "TRUNCATE attribute_history.{}",
        escape_identifier(&compacted_tmp_table_name)
    );

    client
        .execute(&truncate_tmp_table_query, &[])
        .await
        .unwrap();

    let default_columns = [
        "id",
        "entity_id",
        "timestamp",
        "\"end\"",
        "first_appearance",
        "modified",
    ];

    let extended_default_columns = [
        "compacted.id",
        "compacted.entity_id",
        "timestamp",
        "\"end\"",
        "first_appearance",
        "modified",
    ];

    let columns_part = default_columns.join(", ")
        + ", "
        + &attributes
            .iter()
            .map(|attribute| escape_identifier(&attribute.name))
            .collect::<Vec<String>>()
            .join(", ");
    let extended_columns_part = extended_default_columns.join(", ")
        + ", "
        + &attributes
            .iter()
            .map(|attribute| escape_identifier(&attribute.name))
            .collect::<Vec<String>>()
            .join(", ");

    let query = format!(
        r#" 
        WITH to_compact AS (
            SELECT entity_id, MIN(id) AS first_id 
            FROM attribute_history.{}
            WHERE id > $1 AND id <= $2 GROUP BY entity_id
        )
        INSERT INTO attribute_history.{}({}) 
        SELECT {} FROM attribute_history.{} AS compacted
        JOIN to_compact ON compacted.entity_id = to_compact.entity_id"#,
        escape_identifier(attribute_store_name),
        escape_identifier(&compacted_tmp_table_name),
        columns_part,
        extended_columns_part,
        escape_identifier(&compacted_view_name),
    );

    println!(
        "Loading data into temporary table '{}'",
        compacted_tmp_table_name
    );

    let insert_count = client
        .execute(&query, &[&last_compacted_id, &max_to_compact])
        .await
        .unwrap();

    println!("Inserted {} records", insert_count);

    let update_end_query = format!(
        r#"UPDATE attribute_history.{} SET "end" = "timestamp" WHERE id > $1 AND "end" IS NULL"#,
        escape_identifier(attribute_store_name)
    );

    let update_count = client
        .execute(&update_end_query, &[&last_compacted_id])
        .await
        .unwrap();

    println!("Updated {} records", update_count);

    let delete_query = format!(
        r#"
        DELETE FROM attribute_history.{} history
        USING attribute_history.{} tmp
        WHERE history.entity_id = tmp.entity_id
        AND history.timestamp >= tmp.timestamp
        AND history.timestamp <= tmp."end""#,
        escape_identifier(attribute_store_name),
        escape_identifier(&compacted_tmp_table_name),
    );

    let delete_count = client.execute(&delete_query, &[]).await.unwrap();

    println!("Deleted {} records", delete_count);

    let insert_compacted_query = format!(
        r#"
        INSERT INTO attribute_history.{}({}) 
        SELECT {} 
        FROM attribute_history.{}"#,
        escape_identifier(attribute_store_name),
        columns_part,
        columns_part,
        escape_identifier(&compacted_tmp_table_name),
    );

    let insert_compacted_count = client.execute(&insert_compacted_query, &[]).await.unwrap();

    mark_attribute_store_modified(client, attribute_store_id)
        .await
        .unwrap();
    mark_attribute_store_compacted(client, attribute_store_id, max_to_compact)
        .await
        .unwrap();

    Ok(CompactResult {
        record_count: insert_compacted_count,
        attribute_store_id,
        attribute_store_name: attribute_store_name.to_string(),
    })
}

async fn mark_attribute_store_modified<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
) -> Result<(), ()> {
    let query = r#"
INSERT INTO attribute_directory.attribute_store_modified (attribute_store_id, modified)
VALUES ($1, now())
ON CONFLICT (attribute_store_id) DO UPDATE
SET modified = EXCLUDED.modified
RETURNING attribute_store_modified"#;

    client.execute(query, &[&attribute_store_id]).await.unwrap();

    Ok(())
}

async fn mark_attribute_store_compacted<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
    last_compacted: i32,
) -> Result<(), ()> {
    let query = r#"
INSERT INTO attribute_directory.attribute_store_compacted (attribute_store_id, compacted)
VALUES ($1, $2)
ON CONFLICT (attribute_store_id) DO UPDATE
SET compacted = EXCLUDED.compacted"#;

    client
        .execute(query, &[&attribute_store_id, &last_compacted])
        .await
        .unwrap();

    Ok(())
}

pub async fn compact_attribute_store_by_name<T: GenericClient + Send + Sync>(
    client: &T,
    name: &str,
    max_compact_count: i32,
) -> Result<CompactResult, CompactError> {
    let query =
        "SELECT id FROM attribute_directory.attribute_store WHERE attribute_store::text = $1";

    let rows = client
        .query(query, &[&name])
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        return Err(CompactError::NoSuchAttributeStoreName(name.to_string()));
    }

    let row = rows.first().unwrap();

    let attribute_store_id: i32 = row.get(0);

    compact_attribute_store_by_id(client, attribute_store_id, max_compact_count).await
}
