use tokio_postgres::GenericClient;

use crate::trend_store::{format_duration, TrendStore};

#[derive(thiserror::Error, Debug)]
pub enum RemoveTrendStoreError {
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("Database issue: {0}")]
    Database(#[from] tokio_postgres::Error),
    #[error("No such data source: '{0}'")]
    NoSuchDataSource(String),
    #[error("No such entity type: '{0}'")]
    NoSuchEntityType(String),
    #[error("No such trend store")]
    NoSuchTrendStore,
}

pub async fn remove_trend_store<T: GenericClient>(
    client: &mut T,
    trend_store: &TrendStore,
) -> Result<(), RemoveTrendStoreError> {
    let get_data_source_id_query = "SELECT id FROM directory.data_source WHERE name = $1";

    let rows = client
        .query(get_data_source_id_query, &[&trend_store.data_source])
        .await?;

    let data_source_id: i32 = rows
        .first()
        .ok_or(RemoveTrendStoreError::NoSuchDataSource(
            trend_store.data_source.clone(),
        ))?
        .get(0);

    let get_entity_type_id_query = "SELECT id FROM directory.entity_type WHERE name = $1";

    let rows = client
        .query(get_entity_type_id_query, &[&trend_store.entity_type])
        .await?;

    let entity_type_id: i32 = rows
        .first()
        .ok_or(RemoveTrendStoreError::NoSuchEntityType(
            trend_store.entity_type.clone(),
        ))?
        .get(0);

    let delete_trend_store_query = concat!(
        "DELETE FROM trend_directory.trend_store ",
        "WHERE data_source_id = $1 AND entity_type_id = $2 AND granularity = $3::text::interval",
    );

    let granularity_str: String = format_duration(trend_store.granularity).to_string();

    let deleted_count = client
        .execute(
            delete_trend_store_query,
            &[&data_source_id, &entity_type_id, &granularity_str],
        )
        .await?;

    match deleted_count {
        0 => Err(RemoveTrendStoreError::NoSuchTrendStore),
        1 => Ok(()),
        _ => Err(RemoveTrendStoreError::Unexpected(format!(
            "Unexpected number of trend stores matched: {deleted_count}"
        ))),
    }
}
