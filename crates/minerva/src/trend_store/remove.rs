use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use crate::trend_store::{format_duration, TrendStore};

const BASE_TABLE_SCHEMA: &str = "trend";
const STAGING_TABLE_SCHEMA: &str = "trend";

#[derive(thiserror::Error, Debug)]
pub enum RemoveTrendStoreError {
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("Database issue: {0}")]
    Database(#[from] tokio_postgres::Error),
}

pub async fn remove_trend_store<T: GenericClient>(
    client: &mut T,
    trend_store: &TrendStore,
) -> Result<(), RemoveTrendStoreError> {
    for trend_store_part in &trend_store.parts {
        remove_trend_store_part(client, &trend_store_part.name).await?;
    }

    let insert_trend_store_query = concat!(
        "DELETE FROM trend_directory.trend_store ",
        "USING directory.data_source, directory.entity_type ",
        "WHERE data_source.id = trend_store.data_source_id AND entity_type.id = trend_store.entity_type_id ",
        "AND data_source.name = $1 AND entity_type.name = $2 AND trend_store.granularity = $3::text::interval",
    );

    let granularity_str: String = format_duration(trend_store.granularity).to_string();

    let count = client
        .execute(
            insert_trend_store_query,
            &[
                &trend_store.data_source,
                &trend_store.entity_type,
                &granularity_str,
            ],
        )
        .await?;

    if count == 0 {
        return Err(RemoveTrendStoreError::Unexpected(
            "No trend store deleted".to_string(),
        ));
    }

    Ok(())
}

pub async fn remove_trend_store_part<T: GenericClient>(
    client: &mut T,
    name: &str,
) -> Result<(), RemoveTrendStoreError> {
    drop_staging_table(client, name).await?;
    drop_base_table(client, name).await?;

    let delete_trend_store_part_query = concat!(
        "DELETE FROM trend_directory.trend_store_part ",
        "WHERE name = $1",
    );

    let count = client
        .execute(
            delete_trend_store_part_query,
            &[&name],
        )
        .await?;

    Ok(())
}

pub async fn drop_base_table<T: GenericClient>(
    client: &mut T,
    name: &str,
) -> Result<(), RemoveTrendStoreError> {
    let drop_base_table_query = format!(
        "DROP TABLE {}.{}",
        BASE_TABLE_SCHEMA,
        escape_identifier(name)
    );

    let count = client
        .execute(
            &drop_base_table_query,
            &[],
        )
        .await?;

    Ok(())
}

pub async fn drop_staging_table<T: GenericClient>(
    client: &mut T,
    name: &str,
) -> Result<(), RemoveTrendStoreError> {
    let staging_table_name = format!("{}_staging", name);

    let drop_staging_table_query = format!(
        "DROP TABLE {}.{}",
        STAGING_TABLE_SCHEMA,
        escape_identifier(&staging_table_name),
    );

    let count = client
        .execute(
            &drop_staging_table_query,
            &[],
        )
        .await?;

    Ok(())
}
