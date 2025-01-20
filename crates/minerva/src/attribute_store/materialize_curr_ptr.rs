use std::fmt::Display;

use tokio_postgres::GenericClient;

use crate::error::Error;

pub struct MaterializeCurrPtrResult {
    pub record_count: u64,
    pub materialized_view: bool,
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
    let query = "SELECT ast::text FROM attribute_directory.attribute_store WHERE id = $1";

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

    let materialized_view = if has_materialized_view(client, attribute_store_name).await? {
        refresh_materialized_view(client, attribute_store_name).await?;

        true
    } else {
        false
    };

    Ok(MaterializeCurrPtrResult {
        record_count: record_count.try_into().unwrap(),
        materialized_view,
        attribute_store_name: attribute_store_name.to_string(),
        attribute_store_id,
    })
}

pub async fn has_materialized_view<T: GenericClient + Send + Sync>(
    client: &T,
    attribute_store_name: &str,
) -> Result<bool, MaterializeCurrPtrError> {
    let query = concat!(
        "SELECT relname ",
        "FROM pg_class c ",
        "JOIN pg_namespace ns ON ns.oid = c.relnamespace ",
        "WHERE ns.nspname = 'attribute' AND c.relname = $1 AND c.relkind = 'm'"
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
        Ok(false)
    } else {
        Ok(true)
    }
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
