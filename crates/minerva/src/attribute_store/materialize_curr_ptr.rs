use std::fmt::Display;

use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use crate::error::Error;

pub struct MaterializeCurrPtrResult {
    pub record_count: u64,
    pub updated_curr_data: String,
    pub attribute_store_name: String,
    pub attribute_store_id: i32,
}

impl Display for MaterializeCurrPtrResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Materialized {} curr-ptr records for '{}'({})",
            self.record_count, self.attribute_store_name, self.attribute_store_id
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MaterializeCurrPtrError {
    #[error("Unexpected error during curr-ptr materialization: {0}")]
    Unexpected(String),
    #[error("No attribute store matching Id {0}")]
    NoAttributeStoreWithId(i32),
    #[error("No attribute store matching name '{0}'")]
    NoAttributeStoreWithName(String),
}

impl From<MaterializeCurrPtrError> for Error {
    fn from(value: MaterializeCurrPtrError) -> Self {
        Error::Runtime(format!("Could not compact attribute data: {}", value).into())
    }
}

pub async fn materialize_curr_ptr<T: GenericClient + Send + Sync>(
    client: &T,
    id: i32,
) -> Result<MaterializeCurrPtrResult, MaterializeCurrPtrError> {
    let query = "SELECT ast::text FROM attribute_directory.attribute_store ast WHERE id = $1";

    let rows = client
        .query(query, &[&id])
        .await
        .map_err(|e| MaterializeCurrPtrError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        return Err(MaterializeCurrPtrError::NoAttributeStoreWithId(id));
    }

    let row = rows.first().unwrap();

    let attribute_store_name: String = row.get(0);

    materialize_curr_ptr_by_name(client, &attribute_store_name).await
}

pub async fn materialize_curr_ptr_by_name<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_name: &str,
) -> Result<MaterializeCurrPtrResult, MaterializeCurrPtrError> {
    let query = "SELECT id FROM attribute_directory.attribute_store ast WHERE ast::text = $1";

    let rows = client
        .query(query, &[&attribute_store_name])
        .await
        .map_err(|e| MaterializeCurrPtrError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        return Err(MaterializeCurrPtrError::NoAttributeStoreWithName(
            attribute_store_name.to_string(),
        ));
    }

    let row = rows.first().unwrap();

    let attribute_store_id: i32 = row.get(0);

    let query = "SELECT attribute_directory.materialize_curr_ptr(ast) FROM attribute_directory.attribute_store ast WHERE ast::text = $1";

    let row = client
        .query_one(query, &[&attribute_store_name])
        .await
        .map_err(|e| MaterializeCurrPtrError::Unexpected(format!("{e}")))?;

    let record_count: i32 = row.get(0);

    let updated_curr_data = match curr_data_storage_method(client, attribute_store_name).await? {
        CurrDataStorageMethod::View => {
            // Nothing to do for a view
            "view".to_string()
        }
        CurrDataStorageMethod::Table => {
            update_curr_table(client, attribute_store_name).await?;
            "table".to_string()
        }
        CurrDataStorageMethod::MaterializedView => {
            refresh_materialized_view(client, attribute_store_name).await?;
            "materialized view".to_string()
        }
    };

    Ok(MaterializeCurrPtrResult {
        record_count: record_count.try_into().unwrap(),
        updated_curr_data,
        attribute_store_name: attribute_store_name.to_string(),
        attribute_store_id,
    })
}

pub enum CurrDataStorageMethod {
    View,
    MaterializedView,
    Table,
}

pub async fn curr_data_storage_method<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_name: &str,
) -> Result<CurrDataStorageMethod, MaterializeCurrPtrError> {
    let query = concat!(
        "SELECT relkind ",
        "FROM pg_class c ",
        "JOIN pg_namespace ns ON ns.oid = c.relnamespace ",
        "WHERE ns.nspname = 'attribute' AND c.relname = $1"
    );

    let rows = client
        .query(query, &[&attribute_store_name])
        .await
        .map_err(|e| {
            MaterializeCurrPtrError::Unexpected(format!(
                "Could not determine existence of materialized view: {e}"
            ))
        })?;

    if rows.is_empty() {
        Err(MaterializeCurrPtrError::Unexpected(
            "Could not determine current data storage method: No meta data found".to_string(),
        ))
    } else {
        let row = rows.first().unwrap();

        let kind: i8 = row.get(0);
        let kind_char = kind as u8 as char;

        match kind_char {
            'm' => Ok(CurrDataStorageMethod::MaterializedView),
            'r' => Ok(CurrDataStorageMethod::Table),
            'v' => Ok(CurrDataStorageMethod::View),
            _ => Err(MaterializeCurrPtrError::Unexpected(format!(
                "Unexpected relation type: '{}'",
                kind_char
            ))),
        }
    }
}

pub async fn update_curr_table<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_name: &str,
) -> Result<u64, MaterializeCurrPtrError> {
    let query = format!("TRUNCATE TABLE attribute.\"{}\"", attribute_store_name);

    let _count = client.execute(&query, &[]).await.map_err(|e| {
        MaterializeCurrPtrError::Unexpected(format!(
            "Could not truncate curr-data table: {e}"
        ))
    })?;

    let query = concat!(
        "SELECT attribute.name ",
        "FROM attribute_directory.attribute ",
        "JOIN attribute_directory.attribute_store ast ON ast.id = attribute.attribute_store_id ",
        "WHERE ast::text = $1",
    );

    let rows = client
        .query(query, &[&attribute_store_name])
        .await
        .map_err(|e| {
            MaterializeCurrPtrError::Unexpected(format!("Could not query attributes: {e}"))
        })?;

    let mut columns: Vec<String> = vec![
        "id",
        "first_appearance",
        "modified",
        "hash",
        "entity_id",
        "end",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let mut attribute_names: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
    columns.append(&mut attribute_names);

    let dest_cols_part = columns.iter().map(|col_name| escape_identifier(col_name)).collect::<Vec<String>>().join(",");
    let src_cols_part = columns.iter().map(|col_name| format!("history.{}", escape_identifier(col_name))).collect::<Vec<String>>().join(",");

    let query = format!("INSERT INTO attribute.\"{}\"({}) SELECT {} FROM attribute_history.\"{}\" history JOIN attribute_history.\"{}_curr_ptr\" curr_ptr ON history.id = curr_ptr.id", attribute_store_name, dest_cols_part, src_cols_part, attribute_store_name, attribute_store_name);

    let record_count = client.execute(&query, &[]).await.map_err(|e| {
        MaterializeCurrPtrError::Unexpected(format!(
            "Could not load data into curr-data table: {e}"
        ))
    })?;

    Ok(record_count)
}

pub async fn refresh_materialized_view<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_name: &str,
) -> Result<(), MaterializeCurrPtrError> {
    let query = format!("REFRESH MATERIALIZED VIEW attribute.\"{attribute_store_name}\"");

    client.execute(&query, &[]).await.map_err(|e| {
        MaterializeCurrPtrError::Unexpected(format!(
            "Could not refresh materialized view attribute.\"{attribute_store_name}\": {e}"
        ))
    })?;

    Ok(())
}
