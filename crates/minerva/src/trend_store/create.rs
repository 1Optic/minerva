use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use crate::trend_store::{format_duration, TrendStore, TrendStorePart};

use super::{GeneratedTrend, Trend};

#[derive(thiserror::Error, Debug)]
pub enum CreateTrendStoreError {
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("Database issue: {0}")]
    Database(#[from] tokio_postgres::Error),
}

pub async fn create_trend_store<T: GenericClient>(
    client: &mut T,
    trend_store: &TrendStore,
) -> Result<(), CreateTrendStoreError> {
    let get_data_source_id_query = "SELECT id FROM directory.data_source WHERE name = $1";

    let rows = client
        .query(get_data_source_id_query, &[&trend_store.data_source])
        .await?;

    let data_source_id: i32 = if rows.is_empty() {
        create_data_source(client, &trend_store.data_source, "default").await?
    } else {
        rows.first().unwrap().get(0)
    };

    let get_entity_type_id_query = "SELECT id FROM directory.entity_type WHERE name = $1";

    let rows = client
        .query(get_entity_type_id_query, &[&trend_store.entity_type])
        .await?;

    let entity_type_id: i32 = if rows.is_empty() {
        create_entity_type(client, &trend_store.entity_type, "").await?
    } else {
        rows.first().unwrap().get(0)
    };

    let insert_trend_store_query = concat!(
        "INSERT INTO trend_directory.trend_store (",
        "data_source_id, entity_type_id, granularity, partition_size, retention_period) ",
        "VALUES ($1, $2, $3::text::interval, $4::text::interval, $5::text::interval) ",
        "RETURNING id"
    );

    let granularity_str: String = format_duration(trend_store.granularity).to_string();
    let partition_size_str = format_duration(trend_store.partition_size).to_string();
    let retention_period_str = format_duration(trend_store.retention_period).to_string();

    let rows = client
        .query(
            insert_trend_store_query,
            &[
                &data_source_id,
                &entity_type_id,
                &granularity_str,
                &partition_size_str,
                &retention_period_str,
            ],
        )
        .await?;

    if rows.is_empty() {
        return Err(CreateTrendStoreError::Unexpected(
            "No trend store inserted".to_string(),
        ));
    }

    let trend_store_id: i32 = rows.first().unwrap().get(0);

    for trend_store_part in &trend_store.parts {
        create_trend_store_part(client, trend_store_id, trend_store_part).await?;
    }

    Ok(())
}

pub async fn create_data_source<T: GenericClient>(
    client: &mut T,
    name: &str,
    description: &str,
) -> Result<i32, CreateTrendStoreError> {
    let create_data_source_query =
        "INSERT INTO directory.data_source(name, description) VALUES ($1, $2) RETURNING id";

    let rows = client
        .query(create_data_source_query, &[&name, &description])
        .await?;

    Ok(rows.first().unwrap().get(0))
}

pub async fn create_entity_type<T: GenericClient>(
    client: &mut T,
    name: &str,
    description: &str,
) -> Result<i32, CreateTrendStoreError> {
    let create_entity_type_query =
        "INSERT INTO directory.entity_type(name, description) VALUES ($1, $2) RETURNING id";

    let rows = client
        .query(create_entity_type_query, &[&name, &description])
        .await?;

    let id: i32 = rows.first().unwrap().get(0);

    let create_entity_table_query =
        "SELECT entity.create_entity_table(entity_type, NULL) FROM directory.entity_type WHERE name = $1";
    client.execute(create_entity_table_query, &[&name]).await?;

    let create_get_entity_function_query = "SELECT entity.create_get_entity_function(entity_type) FROM directory.entity_type WHERE name = $1";
    client
        .execute(create_get_entity_function_query, &[&name])
        .await?;

    let create_create_entity_function_query = "SELECT entity.create_create_entity_function(entity_type) FROM directory.entity_type WHERE name = $1";
    client
        .execute(create_create_entity_function_query, &[&name])
        .await?;

    let create_create_to_entity_function_query = "SELECT entity.create_to_entity_function(entity_type) FROM directory.entity_type WHERE name = $1";
    client
        .execute(create_create_to_entity_function_query, &[&name])
        .await?;

    Ok(id)
}

pub async fn create_trend_store_part<T: GenericClient>(
    client: &mut T,
    trend_store_id: i32,
    trend_store_part: &TrendStorePart,
) -> Result<(), CreateTrendStoreError> {
    let insert_trend_store_part_query = concat!(
        "INSERT INTO trend_directory.trend_store_part (trend_store_id, name, primary_alias) ",
        "VALUES ($1, $2, $3) ",
        "RETURNING id"
    );

    let rows = client
        .query(
            insert_trend_store_part_query,
            &[
                &trend_store_id,
                &trend_store_part.name,
                &trend_store_part.has_alias_column,
            ],
        )
        .await?;

    let trend_store_part_id: i32 = rows.first().unwrap().get(0);

    for trend in &trend_store_part.trends {
        define_table_trend(client, trend_store_part_id, trend).await?;
    }

    for generated_trend in &trend_store_part.generated_trends {
        define_generated_trend(client, trend_store_part_id, generated_trend).await?;
    }

    create_base_table(client, trend_store_part).await?;
    create_staging_table(client, trend_store_part).await?;

    Ok(())
}

pub async fn define_table_trend<T: GenericClient>(
    client: &mut T,
    trend_store_part_id: i32,
    trend: &Trend,
) -> Result<(), CreateTrendStoreError> {
    let query = concat!(
        "INSERT INTO trend_directory.table_trend(name, data_type, trend_store_part_id, description, time_aggregation, entity_aggregation, extra_data) ",
        "VALUES ($1, $2, $3, $4, $5, $6, $7)"
    );

    client
        .execute(
            query,
            &[
                &trend.name,
                &trend.data_type,
                &trend_store_part_id,
                &trend.description,
                &trend.time_aggregation,
                &trend.entity_aggregation,
                &trend.extra_data,
            ],
        )
        .await?;

    Ok(())
}

pub async fn define_generated_trend<T: GenericClient>(
    client: &mut T,
    trend_store_part_id: i32,
    trend: &GeneratedTrend,
) -> Result<(), CreateTrendStoreError> {
    let query = concat!(
        "INSERT INTO trend_directory.generated_table_trend(trend_store_part_id, name, data_type, expression, extra_data, description) ",
        "VALUES ($1, $2, $3, $4, $5, $6)"
    );

    client
        .execute(
            query,
            &[
                &trend_store_part_id,
                &trend.name,
                &trend.data_type,
                &trend.expression,
                &trend.extra_data,
                &trend.description,
            ],
        )
        .await?;

    Ok(())
}

fn trend_column_spec(trend: &Trend) -> String {
    format!("{} {}", escape_identifier(&trend.name), trend.data_type)
}

fn generated_trend_column_spec(generated_trend: &GeneratedTrend) -> String {
    format!(
        "{} {} GENERATED ALWAYS AS ({}) STORED",
        escape_identifier(&generated_trend.name),
        generated_trend.data_type,
        generated_trend.expression
    )
}

pub async fn create_base_table<T: GenericClient>(
    client: &mut T,
    trend_store_part: &TrendStorePart,
) -> Result<(), CreateTrendStoreError> {
    let base_table_schema = "trend";
    let base_table_name = escape_identifier(&trend_store_part.name);

    let mut column_specs: Vec<String> = trend_store_part
        .base_columns()
        .iter()
        .map(|column| format!("{} {}", escape_identifier(&column.name), column.data_type))
        .collect();

    column_specs.extend(trend_store_part.trends.iter().map(trend_column_spec));

    column_specs.extend(
        trend_store_part
            .generated_trends
            .iter()
            .map(generated_trend_column_spec),
    );

    let columns_part = column_specs.join(",");

    let create_table_query = format!(
        concat!(
            "CREATE TABLE {}.{} (",
            "{}",
            ") PARTITION BY RANGE (\"timestamp\")"
        ),
        base_table_schema, base_table_name, columns_part,
    );

    client.execute(&create_table_query, &[]).await?;

    let alter_table_add_primary_key_query = format!(
        "ALTER TABLE {base_table_schema}.{base_table_name} ADD PRIMARY KEY (entity_id, \"timestamp\")",
    );

    client
        .execute(&alter_table_add_primary_key_query, &[])
        .await?;

    let create_job_id_index_query =
        format!("CREATE INDEX ON {base_table_schema}.{base_table_name} USING btree (job_id)",);

    client.execute(&create_job_id_index_query, &[]).await?;

    let create_timestamp_index_query =
        format!("CREATE INDEX ON {base_table_schema}.{base_table_name} USING btree (timestamp)",);

    client.execute(&create_timestamp_index_query, &[]).await?;

    let create_distributed_table_query = format!(
        "SELECT create_distributed_table('{base_table_schema}.{base_table_name}', 'entity_id')",
    );

    client.execute(&create_distributed_table_query, &[]).await?;

    Ok(())
}

pub async fn create_staging_table<T: GenericClient>(
    client: &mut T,
    trend_store_part: &TrendStorePart,
) -> Result<(), CreateTrendStoreError> {
    let staging_table_schema = "trend";
    let staging_table_name = escape_identifier(&format!("{}_staging", &trend_store_part.name));

    let mut column_specs: Vec<String> = trend_store_part
        .base_columns()
        .iter()
        .map(|column| format!("{} {}", escape_identifier(&column.name), column.data_type))
        .collect();

    column_specs.extend(trend_store_part.trends.iter().map(trend_column_spec));

    let columns_part = column_specs.join(",");

    let create_table_query = format!(
        "CREATE UNLOGGED TABLE {staging_table_schema}.{staging_table_name}({columns_part})",
    );

    client.execute(&create_table_query, &[]).await?;

    let add_primary_key_query = format!(
        "ALTER TABLE ONLY {staging_table_schema}.{staging_table_name} ADD PRIMARY KEY (entity_id, \"timestamp\")",
    );

    client.execute(&add_primary_key_query, &[]).await?;

    Ok(())
}
