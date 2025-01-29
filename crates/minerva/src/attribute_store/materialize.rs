use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use super::load_attribute_names;

#[derive(thiserror::Error, Debug)]
pub enum AttributeMaterializeError {
    #[error("")]
    Unexpected(String),
}

pub struct AttributeStoreRef 
{
    pub id: i32,
    pub name: String,
}

pub async fn materialize_attribute<T: GenericClient + Send + Sync>(
    client: &T, attribute_store: AttributeStoreRef, view_name: &str
) -> Result<u64, AttributeMaterializeError> {
    let mut attribute_names = load_attribute_names(client, attribute_store.id).await.map_err(AttributeMaterializeError::Unexpected)?;
    let mut columns: Vec<String> = vec!["entity_id", "timestamp"].into_iter().map(String::from).collect();
    columns.append(&mut attribute_names);
    let columns_part = columns.iter().map(|name| escape_identifier(name)).collect::<Vec<String>>().join(",");

    let query = format!("INSERT INTO attribute_history.\"{}\"({}) SELECT {} FROM {}", attribute_store.name, columns_part, columns_part, view_name);

    let record_count = client.execute(&query, &[]).await.map_err(|e|AttributeMaterializeError::Unexpected(format!("Could not stage attribute data: {e}")))?;

    Ok(record_count)
}

