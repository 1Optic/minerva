use std::fmt::Display;

use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

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
    #[error("Could not materialize curr_ptr data: {0}")]
    CurrPtr(String),
    #[error("Could mark attribute store with ID {0} as compacted: {1}")]
    MarkCompacted(i32, String),
}

impl From<CompactError> for Error {
    fn from(value: CompactError) -> Self {
        Error::Runtime(format!("Could not compact attribute data: {}", value).into())
    }
}

pub async fn compact_attribute_store_by_id<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
    limit: Option<usize>,
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

    let create_tmp_table_query = r#"
CREATE TEMP TABLE compact_info (
    id integer,
    first_id integer,
    last_id integer,
    timestamp timestamptz,
    modified timestamptz
) ON COMMIT DROP"#;

    client.execute(create_tmp_table_query, &[]).await.unwrap();

    let insert_count = match limit {
        Some(max_records) => {
            let query = format!(
                r#"
INSERT INTO compact_info(id, first_id, last_id, timestamp, modified)
SELECT
    id, first_id, last_id, timestamp, modified
FROM (
    SELECT
        id,
        first_value(id) OVER (PARTITION BY entity_id, run ORDER BY timestamp ASC) AS first_id,
        first_value(id) OVER (PARTITION BY entity_id, run ORDER BY timestamp DESC) AS last_id,
        "timestamp",
        modified,
        count(*) OVER (PARTITION BY entity_id, run) AS run_length
    FROM (
        SELECT
            id,
            entity_id,
            "timestamp",
            first_appearance,
            modified,
            sum(change) OVER w2 AS run
        FROM (
            SELECT
                id,
                entity_id,
                "timestamp",
                first_appearance,
                modified,
                CASE
                    WHEN hash <> lag(hash) OVER w THEN 1
                    ELSE 0
                END AS change
            FROM attribute_history."{}"
            WINDOW w AS (PARTITION BY entity_id ORDER BY "timestamp")
        ) t
        WINDOW w2 AS (PARTITION BY entity_id ORDER BY "timestamp")
        LIMIT $1
    ) runs
) to_compact WHERE run_length > 1
"#,
                attribute_store_name
            );

            client
                .execute(&query, &[&(max_records as i64)])
                .await
                .unwrap()
        }
        None => {
            let query = format!(
                r#"
INSERT INTO compact_info(id, first_id, last_id, timestamp, modified)
SELECT
    id, first_id, last_id, timestamp, modified
FROM (
    SELECT
        id,
        first_value(id) OVER (PARTITION BY entity_id, run ORDER BY timestamp ASC) AS first_id,
        first_value(id) OVER (PARTITION BY entity_id, run ORDER BY timestamp DESC) AS last_id,
        "timestamp",
        modified,
        count(*) OVER (PARTITION BY entity_id, run) AS run_length
    FROM (
        SELECT
            id,
            entity_id,
            "timestamp",
            first_appearance,
            modified,
            sum(change) OVER w2 AS run
        FROM (
            SELECT
                id,
                entity_id,
                "timestamp",
                first_appearance,
                modified,
                CASE
                    WHEN hash <> lag(hash) OVER w THEN 1
                    ELSE 0
                END AS change
            FROM attribute_history."{}"
            WINDOW w AS (PARTITION BY entity_id ORDER BY "timestamp")
        ) t
        WINDOW w2 AS (PARTITION BY entity_id ORDER BY "timestamp")
    ) runs
) to_compact WHERE run_length > 1
"#,
                attribute_store_name
            );

            client.execute(&query, &[]).await.unwrap()
        }
    };

    println!("Inserted {} records", insert_count);

    let update_history_query = format!(
        r#"
UPDATE attribute_history.{} history
SET modified = compact_info.modified, "end" = compact_info.timestamp
FROM compact_info
WHERE compact_info.first_id = history.id AND compact_info.id = compact_info.last_id"#,
        escape_identifier(attribute_store_name),
    );

    let updated_count = client.execute(&update_history_query, &[]).await.unwrap();

    println!("Updated {} records", updated_count);

    let delete_query = format!(
        r#"
DELETE FROM attribute_history.{} history
USING compact_info
WHERE compact_info.id = history.id
AND compact_info.id <> compact_info.first_id"#,
        escape_identifier(attribute_store_name),
    );

    let delete_count = client.execute(&delete_query, &[]).await.unwrap();

    println!("Deleted {} records", delete_count);

    if updated_count > 0 || delete_count > 0 {
        mark_attribute_store_modified(client, attribute_store_id)
            .await
            .unwrap();
    }

    // Only if compacting is done without limit, we can be sure that everything is compacted
    if limit.is_none() {
        mark_attribute_store_compacted(client, attribute_store_id).await?;
    }

    Ok(CompactResult {
        record_count: updated_count,
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

pub async fn compact_attribute_store_by_name<T: GenericClient + Send + Sync>(
    client: &T,
    name: &str,
    limit: Option<usize>,
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

    compact_attribute_store_by_id(client, attribute_store_id, limit).await
}

async fn mark_attribute_store_compacted<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_id: i32,
) -> Result<(), CompactError> {
    let query = r#"
INSERT INTO attribute_directory.attribute_store_compacted (attribute_store_id, compacted)
SELECT attribute_store_id, modified FROM attribute_directory.attribute_store_modified WHERE attribute_store_id = $1
ON CONFLICT (attribute_store_id) DO UPDATE SET compacted = EXCLUDED.compacted"#;

    client
        .execute(query, &[&attribute_store_id])
        .await
        .map_err(|e| CompactError::MarkCompacted(attribute_store_id, format!("{e}")))?;

    Ok(())
}
