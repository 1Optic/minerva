use std::fmt::Display;
use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use super::{load_attribute_names, materialize_curr_ptr::{materialize_curr_ptr_by_name, MaterializeCurrPtrError, MaterializeCurrPtrResult}};

#[derive(thiserror::Error, Debug)]
pub enum AttributeMaterializeError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Could not materialize curr ptr data: {0}")]
    CurrPtrMaterialization(#[from] MaterializeCurrPtrError),
}

#[derive(thiserror::Error, Debug)]
pub enum AttributeStoreRefError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("No attribute store matching name '{0}'")]
    NoAttributeStoreWithName(String),
}

pub struct AttributeStoreRef 
{
    pub id: i32,
    pub name: String,
}

impl Display for AttributeStoreRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'({})", self.name, self.id)
    }
}

impl AttributeStoreRef {
    pub async fn from_name<T: GenericClient + Send + Sync>(client: &T, name: &str) -> Result<AttributeStoreRef, AttributeStoreRefError> {
        let query = "SELECT id FROM attribute_directory.attribute_store ast WHERE ast::text = $1";
        let rows = client
            .query(query, &[&name])
            .await
            .map_err(|e| AttributeStoreRefError::Unexpected(format!("{e}")))?;

        if rows.is_empty() {
            return Err(AttributeStoreRefError::NoAttributeStoreWithName(
                name.to_string(),
            ));
        }

        let row = rows.first().unwrap();

        let attribute_store_id: i32 = row.get(0);

        let attribute_store_ref = AttributeStoreRef {
            id: attribute_store_id,
            name: name.to_string(),
        };

        Ok(attribute_store_ref)
    }
}

pub struct MaterializeAttributeResult {
    pub materialized_record_count: u64,
    pub materialized_curr_ptr: MaterializeCurrPtrResult,
}

pub async fn materialize_attribute<T: GenericClient + Send + Sync>(
    client: &T, attribute_store: &AttributeStoreRef, view_name: &str
) -> Result<MaterializeAttributeResult, AttributeMaterializeError> {
    let mut attribute_names = load_attribute_names(client, attribute_store.id).await.map_err(AttributeMaterializeError::Unexpected)?;
    let mut columns: Vec<String> = vec!["entity_id", "timestamp"].into_iter().map(String::from).collect();
    columns.append(&mut attribute_names);
    let columns_part = columns.iter().map(|name| escape_identifier(name)).collect::<Vec<String>>().join(",");

    let query = format!("TRUNCATE TABLE attribute_staging.\"{}\"", attribute_store.name);

    client.execute(&query, &[]).await.map_err(|e|AttributeMaterializeError::Unexpected(format!("Could not truncate staging table: {e}")))?;

    let query = format!("INSERT INTO attribute_staging.\"{}\"({}) SELECT {} FROM attribute.\"{}\"", attribute_store.name, columns_part, columns_part, view_name);

    let staged_record_count = client.execute(&query, &[]).await.map_err(|e|AttributeMaterializeError::Unexpected(format!("Could not materialize attribute data: {e}")))?;
    let query = format!("INSERT INTO attribute_history.\"{}\"({}) SELECT {} FROM attribute_staging.\"{}\"", attribute_store.name, columns_part, columns_part, attribute_store.name);

    let record_count = client.execute(&query, &[]).await.map_err(|e|AttributeMaterializeError::Unexpected(format!("Could not materialize attribute data: {e}")))?;

    let query = "SELECT attribute_directory.mark_modified(ast.id) FROM attribute_directory.attribute_store ast WHERE ast::text = $1";

    client.execute(query, &[&attribute_store.name]).await.map_err(|e|AttributeMaterializeError::Unexpected(format!("Could not mark attribute store modified: {e}")))?;

    let curr_ptr_result = materialize_curr_ptr_by_name(client, &attribute_store.name).await?;

    Ok(MaterializeAttributeResult { materialized_record_count: record_count, materialized_curr_ptr: curr_ptr_result })
}

