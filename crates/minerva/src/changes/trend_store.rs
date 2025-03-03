use humantime::format_duration;
use postgres_protocol::escape::escape_identifier;
use serde_json::Value;
use std::fmt;
use tokio_postgres::{Client, GenericClient};

use async_trait::async_trait;

use crate::change::{Change, ChangeResult};
use crate::error::DatabaseError;
use crate::meas_value::DataType;
use crate::trend_store::create::create_trend_store;
use crate::trend_store::{GeneratedTrend, Trend, TrendStore, TrendStorePart};

pub struct RemoveTrends {
    pub trend_store_part: TrendStorePart,
    pub trends: Vec<String>,
}

impl fmt::Display for RemoveTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RemoveTrends({}, {}):\n{}",
            &self.trend_store_part,
            self.trends.len(),
            &self
                .trends
                .iter()
                .map(|t| format!(" - {}\n", &t))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl fmt::Debug for RemoveTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RemoveTrends({}, {})",
            &self.trend_store_part,
            &self
                .trends
                .iter()
                .map(|t| format!("'{}'", &t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[async_trait]
impl Change for RemoveTrends {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = concat!(
            "SELECT trend_directory.remove_table_trend(table_trend) ",
            "FROM trend_directory.table_trend ",
            "JOIN trend_directory.trend_store_part ON trend_store_part.id = table_trend.trend_store_part_id ",
            "WHERE trend_store_part.name = $1 AND table_trend.name = $2",
        );

        for trend_name in &self.trends {
            tx 
                .query_one(query, &[&self.trend_store_part.name, &trend_name])
                .await
                .map_err(|e| {
                    DatabaseError::from_msg(format!(
                        "Error removing trend '{}' from trend store part: {}",
                        &trend_name, e
                    ))
                })?;
        }

        tx.commit().await?;

        Ok(format!(
            "Removed {} trends from trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part.name
        ))
    }
}

////////////
// AddTrends
////////////

pub struct AddTrends {
    pub trend_store_part: TrendStorePart,
    pub trends: Vec<Trend>,
}

impl fmt::Display for AddTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrends({}, {}):\n{}",
            &self.trend_store_part,
            &self.trends.len(),
            &self
                .trends
                .iter()
                .map(|t| format!(" - {}: {}\n", &t.name, &t.data_type))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl fmt::Debug for AddTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrends({}, {})",
            &self.trend_store_part,
            &self
                .trends
                .iter()
                .map(|t| format!("{}", &t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[async_trait]
impl Change for AddTrends {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_table_trends(&mut tx, &self.trend_store_part.name, &self.trends)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!("Error adding trends to trend store part: {e}"))
            })?;

        tx.commit().await?;

        Ok(format!(
            "Added {} trends to trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part.name
        ))
    }
}

async fn create_table_trends<T: GenericClient>(
    client: &mut T,
    trend_store_part_name: &str,
    trends: &[Trend],
) -> Result<(), tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT id FROM trend_directory.trend_store_part WHERE name = $1",
            &[&trend_store_part_name],
        )
        .await?;

    let trend_store_part_id: i32 = rows.first().unwrap().get(0);

    define_table_trends(client, trend_store_part_id, trends).await?;
    initialize_table_trends(client, trend_store_part_name, trends).await?;

    Ok(())
}

async fn initialize_table_trends<T: GenericClient>(
    client: &mut T,
    trend_store_part_name: &str,
    trends: &[Trend],
) -> Result<(), tokio_postgres::Error> {
    let column_specs = trends
        .iter()
        .map(|trend| {
            format!(
                "ADD COLUMN {} {}",
                escape_identifier(&trend.name),
                trend.data_type
            )
        })
        .collect::<Vec<String>>()
        .join(",");

    let alter_table_query = format!(
        "ALTER TABLE {}.{} {}",
        BASE_TABLE_SCHEMA,
        escape_identifier(trend_store_part_name),
        column_specs
    );

    client.execute(&alter_table_query, &[]).await?;

    Ok(())
}

pub struct ModifyTrendDataType {
    pub trend_name: String,
    pub from_type: DataType,
    pub to_type: DataType,
}

impl fmt::Display for ModifyTrendDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trend({}, {}->{})",
            &self.trend_name, &self.from_type, &self.to_type
        )
    }
}

/// A set of trends of a trend store part for which the data type needs to
/// change.
///
/// The change of data types for multiple trends in a trend store part is
/// grouped into one operation for efficiency purposes.
pub struct ModifyTrendDataTypes {
    pub trend_store_part: TrendStorePart,
    pub modifications: Vec<ModifyTrendDataType>,
}

impl fmt::Display for ModifyTrendDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ModifyTrendDataTypes({}, {}):\n{}",
            &self.trend_store_part,
            self.modifications.len(),
            self.modifications
                .iter()
                .map(|m| format!(" - {}: {} -> {}\n", m.trend_name, m.from_type, m.to_type))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

impl fmt::Debug for ModifyTrendDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifications: Vec<String> =
            self.modifications.iter().map(|m| format!("{m}")).collect();

        write!(
            f,
            "ModifyTrendDataTypes({}, {})",
            &self.trend_store_part,
            &modifications.join(", "),
        )
    }
}

#[async_trait]
impl Change for ModifyTrendDataTypes {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let transaction = client
            .transaction()
            .await
            .map_err(|e| DatabaseError::from_msg(format!("could not start transaction: {e}")))?;

        let timeout_query = "SET SESSION statement_timeout = 0";

        let result = transaction.execute(timeout_query, &[]).await;

        if let Err(e) = result {
            return Err(
                DatabaseError::from_msg(format!("Error setting session timeout: {e}")).into(),
            );
        }

        let timeout_query = "SET SESSION lock_timeout = '10min'";

        let result = transaction.execute(timeout_query, &[]).await;

        if let Err(e) = result {
            return Err(DatabaseError::from_msg(format!("Error setting lock timeout: {e}")).into());
        }

        let query = concat!(
            "UPDATE trend_directory.table_trend tt ",
            "SET data_type = $1 ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = tt.trend_store_part_id AND tsp.name = $2 AND tt.name = $3"
        );

        for modification in &self.modifications {
            let result = transaction
                .execute(
                    query,
                    &[
                        &modification.to_type,
                        &self.trend_store_part.name,
                        &modification.trend_name,
                    ],
                )
                .await;

            if let Err(e) = result {
                transaction.rollback().await.unwrap();

                return Err(
                    DatabaseError::from_msg(format!("Error changing data types: {e}")).into(),
                );
            }
        }

        let alter_type_parts: Vec<String> = self
            .modifications
            .iter()
            .map(|m| {
                format!(
                    "ALTER \"{}\" TYPE {} USING CAST(\"{}\" AS {})",
                    &m.trend_name, &m.to_type, &m.trend_name, &m.to_type
                )
            })
            .collect();

        let alter_type_parts_str = alter_type_parts.join(", ");

        let alter_query = format!(
            "ALTER TABLE trend.\"{}\" {}",
            &self.trend_store_part.name, &alter_type_parts_str
        );

        let alter_query_slice: &str = &alter_query;

        if let Err(e) = transaction.execute(alter_query_slice, &[]).await {
            transaction.rollback().await.unwrap();

            return Err(match e.code() {
                Some(code) => DatabaseError::from_msg(format!(
                    "Error changing data types: {} - {}",
                    code.code(),
                    e
                ))
                .into(),
                None => DatabaseError::from_msg(format!("Error changing data types: {e}")).into(),
            });
        }

        if let Err(e) = transaction.commit().await {
            return Err(DatabaseError::from_msg(format!("Error committing changes: {e}")).into());
        }

        Ok(format!(
            "Altered trend data types for trend store part '{}'",
            &self.trend_store_part.name
        ))
    }
}

pub struct ModifyTrendExtraData {
    pub trend_name: String,
    pub trend_store_part_name: String,
    pub from_extra_data: Value,
    pub to_extra_data: Value,
}

impl fmt::Display for ModifyTrendExtraData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trend({}.{}, {}->{})",
            &self.trend_store_part_name,
            &self.trend_name,
            &self.from_extra_data,
            &self.to_extra_data
        )
    }
}

#[async_trait]
impl Change for ModifyTrendExtraData {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = concat!(
            "UPDATE trend_directory.table_trend tt ",
            "SET extra_data = $1 ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = tt.trend_store_part_id AND tsp.name = $2 AND tt.name = $3"
        );

        tx.execute(
            query,
            &[
                &self.to_extra_data,
                &self.trend_store_part_name,
                &self.trend_name,
            ],
        )
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error changing extra_data: {e}")))?;

        tx.commit().await?;

        Ok(format!(
            "Altered extra_data for trend '{}'.'{}'",
            &self.trend_store_part_name, &self.trend_name,
        ))
    }
}

pub struct AddTrendStorePart {
    pub trend_store: TrendStore,
    pub trend_store_part: TrendStorePart,
}

const BASE_TABLE_SCHEMA: &str = "trend";

<<<<<<< HEAD
<<<<<<< HEAD
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

async fn create_base_table<T: GenericClient>(
    client: &mut T,
    trend_store_part: &TrendStorePart,
) -> Result<(), tokio_postgres::Error> {
    let column_spec = std::iter::once("job_id bigint NOT NULL".to_string())
        .chain(trend_store_part.trends.iter().map(trend_column_spec))
        .chain(
            trend_store_part
                .generated_trends
                .iter()
                .map(generated_trend_column_spec),
        )
        .collect::<Vec<String>>()
        .join(", ");

    let create_table_query = format!(
        concat!(
            "CREATE TABLE {}.{} (",
            "entity_id integer NOT NULL, ",
            "\"timestamp\" timestamp with time zone NOT NULL, ",
            "created timestamp with time zone NOT NULL, ",
            "{}",
            ") PARTITION BY RANGE (\"timestamp\")"
        ),
        BASE_TABLE_SCHEMA,
        escape_identifier(&trend_store_part.name),
        column_spec,
    );

    client.execute(&create_table_query, &[]).await?;

    let primary_key_query = format!(
        "ALTER TABLE {}.{} ADD PRIMARY KEY (entity_id, \"timestamp\");",
        BASE_TABLE_SCHEMA,
        escape_identifier(&trend_store_part.name),
    );

    client.execute(&primary_key_query, &[]).await?;

    let create_job_id_index_query = format!(
        "CREATE INDEX ON {}.{} USING btree (job_id)",
        BASE_TABLE_SCHEMA,
        escape_identifier(&trend_store_part.name),
    );

    client.execute(&create_job_id_index_query, &[]).await?;

    let create_timestamp_index_query = format!(
        "CREATE INDEX ON {}.{} USING btree (timestamp)",
        BASE_TABLE_SCHEMA,
        escape_identifier(&trend_store_part.name),
    );

    client.execute(&create_timestamp_index_query, &[]).await?;

    let distribute_table_query = format!(
        "SELECT create_distributed_table('{}.{}', 'entity_id')",
        BASE_TABLE_SCHEMA,
        escape_identifier(&trend_store_part.name),
    );

    client.execute(&distribute_table_query, &[]).await?;

    Ok(())
}

async fn define_table_trends<T: GenericClient>(
    client: &mut T,
    trend_store_part_id: i32,
    trends: &[Trend],
) -> Result<(), tokio_postgres::Error> {
    let define_trend_query = concat!(
        "INSERT INTO trend_directory.table_trend(name, data_type, trend_store_part_id, description, time_aggregation, entity_aggregation, extra_data) ",
        "VALUES ($1, $2, $3, $4, $5, $6, $7)"
    );

    for trend in trends {
        client
            .execute(
                define_trend_query,
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
    }

    Ok(())
}

pub async fn create_trend_store_part<T: GenericClient>(
    client: &mut T,
    trend_store: &TrendStore,
    trend_store_part: &TrendStorePart,
) -> Result<(), tokio_postgres::Error> {
    let query = concat!(
        "INSERT INTO trend_directory.trend_store_part(trend_store_id, name) ",
        "SELECT trend_store.id, $4 ",
        "FROM trend_directory.trend_store ",
        "JOIN directory.data_source ON data_source.id = trend_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = trend_store.entity_type_id ",
        "WHERE data_source.name = $1 AND entity_type.name = $2 AND granularity = $3::text::interval ",
        "RETURNING id;"
    );

    let granularity_str: String = format_duration(trend_store.granularity).to_string();

    let trend_store_part_rows = client
        .query(
            query,
            &[
                &trend_store.data_source,
                &trend_store.entity_type,
                &granularity_str,
                &trend_store_part.name,
            ],
        )
        .await?;

    let trend_store_part_id: i32 = trend_store_part_rows.first().unwrap().get(0);

    define_table_trends(client, trend_store_part_id, &trend_store_part.trends).await?;

    let define_generated_trend_query = concat!(
        "INSERT INTO trend_directory.generated_table_trend(trend_store_part_id, name, data_type, expression, extra_data, description) ",
        "VALUES ($1, $2, $3, $4, $5, $6)"
    );

    for generated_trend in &trend_store_part.generated_trends {
        client
            .execute(
                define_generated_trend_query,
                &[
                    &trend_store_part_id,
                    &generated_trend.name,
                    &generated_trend.data_type,
                    &generated_trend.expression,
                    &generated_trend.extra_data,
                    &generated_trend.description,
                ],
            )
            .await?;
    }

    create_base_table(client, trend_store_part).await?;

    Ok(())
}

#[async_trait]
impl Change for AddTrendStorePart {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_trend_store_part(&mut tx, &self.trend_store, &self.trend_store_part)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error creating trend store part '{}': {}",
                    &self.trend_store_part.name, e
                ))
            })?;

        tx.commit().await?;

        Ok(format!(
            "Added trend store part '{}' to trend store '{}'",
            &self.trend_store_part.name, &self.trend_store
        ))
    }
}

impl fmt::Display for AddTrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrendStorePart({}, {})",
            &self.trend_store, &self.trend_store_part
        )
    }
}

pub struct AddTrendStore {
    pub trend_store: TrendStore,
}

impl fmt::Display for AddTrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrendStore({})\n{}",
            &self.trend_store,
            &self
                .trend_store
                .parts
                .iter()
                .map(|part| format!(" - {}\n", &part.name))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[async_trait]
impl Change for AddTrendStore {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_trend_store(&mut tx, &self.trend_store)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error creating trend store: {e}")))?;

        tx.commit().await?;

        Ok(format!("Added trend store {}", &self.trend_store))
    }
}
