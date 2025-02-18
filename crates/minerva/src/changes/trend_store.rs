use humantime::format_duration;
use postgres_protocol::escape::escape_identifier;
use serde_json::Value;
use std::fmt;
use tokio_postgres::{Client, Transaction};
use log::{error, info};

use async_trait::async_trait;

use crate::change::{Change, ChangeResult};
use crate::error::DatabaseError;
use crate::meas_value::DataType;
use crate::trend_store::create::create_trend_store;
use crate::trend_store::{Trend, TrendStore, TrendStorePart};

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
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        let query = concat!(
            "SELECT trend_directory.remove_table_trend(table_trend) ",
            "FROM trend_directory.table_trend ",
            "JOIN trend_directory.trend_store_part ON trend_store_part.id = table_trend.trend_store_part_id ",
            "WHERE trend_store_part.name = $1 AND table_trend.name = $2",
        );

        for trend_name in &self.trends {
            client
                .query_one(query, &[&self.trend_store_part.name, &trend_name])
                .await
                .map_err(|e| {
                    DatabaseError::from_msg(format!(
                        "Error removing trend '{}' from trend store part: {}",
                        &trend_name, e
                    ))
                })?;
        }

        Ok(format!(
            "Removed {} trends from trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part.name
        ))
    }

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
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
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        let query = concat!(
            "SELECT trend_directory.create_table_trends(trend_store_part, $1) ",
            "FROM trend_directory.trend_store_part WHERE name = $2",
        );

        client
            .query_one(query, &[&self.trends, &self.trend_store_part.name])
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!("Error adding trends to trend store part: {e}"))
            })?;

        Ok(format!(
            "Added {} trends to trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part.name
        ))
    }

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
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
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
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

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
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
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        let query = concat!(
            "UPDATE trend_directory.table_trend tt ",
            "SET extra_data = $1 ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = tt.trend_store_part_id AND tsp.name = $2 AND tt.name = $3"
        );

        let result = client
            .execute(
                query,
                &[
                    &self.to_extra_data,
                    &self.trend_store_part_name,
                    &self.trend_name,
                ],
            )
            .await;

        if let Err(e) = result {
            return Err(DatabaseError::from_msg(format!("Error changing extra_data: {e}")).into());
        }

        Ok(format!(
            "Altered extra_data for trend '{}'.'{}'",
            &self.trend_store_part_name, &self.trend_name,
        ))
    }

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}

pub struct AddTrendStorePart {
    pub trend_store: TrendStore,
    pub trend_store_part: TrendStorePart,
}

const BASE_TABLE_SCHEMA: &str = "trend";

async fn trend_column_spec(trend: Trend) -> String {
    format!("{} {}", escape_identifier(trend.name), trend.data_type)
}

async fn generated_trend_column_spec(trend: Trend) -> String {
    format!("{} {} GENERATED ALWAYS AS ({}) STORED", escape_identifier(generated_trend.name), generated_trend.data_type, generated_trend.expression)
}

async fn create_base_table<T: GenericClient>(client: &mut T, trend_store_part: &TrendStorePart) -> Result<(), tokio_postgres::Error> {
    let default_column_specs = vec!(
        "job_id bigint NOT NULL".to_string()
    );

    let trend_column_specs = trend_store_part
        .trends
        .iter()
        .map(trend_column_spec)
        .collect();

    let generated_trend_column_specs = trend_store_part
        .generated_trends
        .iter()
        .map(generated_trend_column_spec)
        .collect();

    let column_spec = "";//array_to_string(ARRAY['job_id bigint NOT NULL'] || trend_directory.column_specs($1), ',')
    let query = format!(
        concat!(
            "CREATE TABLE {}.{} (",
            "entity_id integer NOT NULL, ",
            "\"timestamp\" timestamp with time zone NOT NULL, ",
            "created timestamp with time zone NOT NULL, ",
            "{}"
            ") PARTITION BY RANGE (\"timestamp\")"
        ),
        BASE_TABLE_SCHEMA,
        base_table_name(trend_store_part),
        column_spec,
    );

    format(
        'ALTER TABLE %I.%I ADD PRIMARY KEY (entity_id, "timestamp");',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'CREATE INDEX ON %I.%I USING btree (job_id)',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'CREATE INDEX ON %I.%I USING btree (timestamp);',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    ),
    format(
        'SELECT create_distributed_table(''%I.%I'', ''entity_id'')',
        trend_directory.base_table_schema(),
        trend_directory.base_table_name($1)
    )
}

async fn create_trend_store_part<T: GenericClient>(client: &mut T, trend_store: &TrendStore, trend_store_part: &TrendStorePart) -> Result<(), tokio_postgres::Error> {
    let query = concat!(
        "INSERT INTO trend_directory.trend_store_part(trend_store_id, name) ",
        "SELECT trend_store.id, $4 ",
        "FROM trend_directory.trend_store ",
        "JOIN directory.data_source ON data_source.id = trend_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = trend_store.entity_type_id ",
        "WHERE data_source.name = $1 AND entity_type.name = $2 AND granularity = $3::text::interval ",
        "RETURNING *;"
    );
    
    let granularity_str: String = format_duration(trend_store.granularity).to_string();

    client.query(query, &[&trend_store.data_source, &trend_store.entity_type, &granularity_str, &trend_store_part.name]).await?;

    Ok(())
}

#[async_trait]
impl Change for AddTrendStorePart {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        create_trend_store_part(client, &self.trend_store, &self.trend_store_part)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error creating trend store part '{}': {}",
                    &self.trend_store_part.name, e
                ))
            })?;

        Ok(format!(
            "Added trend store part '{}' to trend store '{}'",
            &self.trend_store_part.name, &self.trend_store
        ))
    }

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
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
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        create_trend_store(client, &self.trend_store)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error creating trend store: {e}")))?;

        Ok(format!("Added trend store {}", &self.trend_store))
    }

    async fn client_apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;
        let result = self.apply(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}
