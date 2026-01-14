use async_trait::async_trait;
use chrono::Utc;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;
use console::Style;
use postgres_protocol::escape::escape_identifier;
use rand::distr::{Alphanumeric, SampleString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use similar::{ChangeTag, TextDiff};
use std::fmt::{self, Display};
use thiserror::Error;
use tokio_postgres::{Client, GenericClient};

use crate::change::{Change, ChangeResult, Changed, InformationOption, MinervaObjectRef};
use crate::error::{DatabaseError, RuntimeError};
use crate::interval::parse_interval;
use crate::meas_value::DataType;
use crate::trend_store::create::{
    create_trend_store, create_trend_store_part, remove_trend_store_part,
};
use crate::trend_store::remove::remove_trend_store;
use crate::trend_store::{
    get_trend_store_id, load_trend_store, load_trend_store_part, load_trend_store_ref_for_part,
    Trend, TrendStore, TrendStorePart, TrendStoreRef,
};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct StageTrendsForDeletion {
    pub trend_store_part_name: String,
    pub trends: Vec<String>,
}

impl fmt::Display for StageTrendsForDeletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "StageTrendsForDeletion(TrendStorePart({}), {}):",
            &self.trend_store_part_name,
            self.trends.len()
        )?;

        for t in &self.trends {
            writeln!(f, " - {}", &t)?;
        }

        Ok(())
    }
}

impl fmt::Debug for StageTrendsForDeletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StageTrendsForDeletion(TrendStorePart({}), {})",
            &self.trend_store_part_name,
            &self
                .trends
                .iter()
                .map(|t| format!("'{}'", &t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

struct TrendColumnRename {
    pub trend_store_part_name: String,
    pub trend_name: String,
    pub staging_name: String,
}

impl TrendColumnRename {
    pub async fn update<T: GenericClient>(
        &self,
        client: &mut T,
    ) -> Result<(), tokio_postgres::Error> {
        let alter_query = format!(
            "ALTER TABLE trend.{} RENAME COLUMN {} TO {}",
            escape_identifier(&self.trend_store_part_name),
            escape_identifier(&self.trend_name),
            escape_identifier(&self.staging_name)
        );

        client.execute(&alter_query, &[]).await?;

        let alter_staging_table_query = format!(
            "ALTER TABLE trend.{} RENAME COLUMN {} TO {}",
            escape_identifier(&format!("{}_staging", self.trend_store_part_name)),
            escape_identifier(&self.trend_name),
            escape_identifier(&self.staging_name)
        );

        client.execute(&alter_staging_table_query, &[]).await?;

        let update_query = concat!(
            "UPDATE trend_directory.table_trend tt ",
            "SET staged_for_deletion = now(), deletion_staging_column = $3 ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = tt.trend_store_part_id AND tsp.name = $1 AND tt.name = $2"
        );

        client
            .execute(
                update_query,
                &[
                    &self.trend_store_part_name,
                    &self.trend_name,
                    &self.staging_name,
                ],
            )
            .await?;

        Ok(())
    }
}

pub fn random_name(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

#[async_trait]
#[typetag::serde]
impl Change for StageTrendsForDeletion {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let renamings: Vec<TrendColumnRename> = self
            .trends
            .iter()
            .map(|trend_name| TrendColumnRename {
                trend_store_part_name: self.trend_store_part_name.clone(),
                trend_name: trend_name.clone(),
                staging_name: format!("_{}", random_name(32)),
            })
            .collect();

        for trend_column_rename in &renamings {
            trend_column_rename.update(&mut tx).await.map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error staging trend '{}' for removal in trend store part '{}': {}",
                    &trend_column_rename.trend_name, &self.trend_store_part_name, e
                ))
            })?;
        }

        tx.commit().await?;

        Ok(Box::new(TrendsStagedForDeletion {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends: self.trends.clone(),
        }))
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        vec![Box::new(TrendRemoveValueInformation {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trend_names: self.trends.to_vec(),
        })]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct TrendsStagedForDeletion {
    pub trend_store_part_name: String,
    pub trends: Vec<String>,
}

impl Display for TrendsStagedForDeletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Staged {} trends for deletion in trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for TrendsStagedForDeletion {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RestoreTrendsStagedForDeletion {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends: self.trends.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
struct RestoreTrendsStagedForDeletion {
    pub trend_store_part_name: String,
    pub trends: Vec<String>,
}

#[async_trait]
#[typetag::serde]
impl Change for RestoreTrendsStagedForDeletion {
    async fn apply(&self, _client: &mut Client) -> ChangeResult {
        Err(crate::error::Error::Runtime(RuntimeError {
            msg: "Not implemented".to_string(),
        }))
    }
}

impl Display for RestoreTrendsStagedForDeletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Staged {} trends for deletion in trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part_name
        )
    }
}

/////////////// RemoveTrends ////////////

async fn load_trend<T: GenericClient>(
    client: &mut T,
    trend_store_part_name: &str,
    trend_name: &str,
) -> Result<Trend, String> {
    let trend_query = concat!(
        "SELECT tt.name, tt.data_type, tt.description, tt.entity_aggregation, tt.time_aggregation, tt.extra_data ",
        "FROM trend_directory.table_trend tt ",
        "JOIN trend_directory.trend_store_part tsp ON tsp.id = tt.trend_store_part_id ",
        "WHERE tsp.name = $1 AND tt.name = $2",
    );

    let rows = client
        .query(trend_query, &[&trend_store_part_name, &trend_name])
        .await
        .unwrap();

    if rows.is_empty() {
        return Err("No such trend".to_string());
    }

    let trend_row = rows.first().unwrap();

    let trend_name: &str = trend_row.get(0);
    let trend_data_type: &str = trend_row.get(1);
    let trend_description: &str = trend_row.get(2);
    let trend_entity_aggregation: &str = trend_row.get(3);
    let trend_time_aggregation: &str = trend_row.get(4);
    let trend_extra_data: Value = trend_row.get(5);

    let trend = Trend {
        name: String::from(trend_name),
        data_type: DataType::from(trend_data_type),
        description: String::from(trend_description),
        entity_aggregation: String::from(trend_entity_aggregation),
        time_aggregation: String::from(trend_time_aggregation),
        extra_data: trend_extra_data,
    };

    Ok(trend)
}

#[derive(Error, Debug)]
enum TrendRemoveError {
    #[error("{0}")]
    Database(#[from] tokio_postgres::Error),
    #[error("No such trend")]
    NoSuchTrend,
}

struct TrendRemove {
    pub trend_store_part_name: String,
    pub trend_name: String,
}

impl TrendRemove {
    pub async fn remove<T: GenericClient>(&self, client: &mut T) -> Result<(), TrendRemoveError> {
        let deletion_staging_column_query = concat!(
            "SELECT deletion_staging_column ",
            "FROM trend_directory.table_trend tt ",
            "JOIN trend_directory.trend_store_part tsp ON tsp.id = tt.trend_store_part_id ",
            "WHERE tsp.name = $1 AND tt.name = $2"
        );

        let rows = client
            .query(
                deletion_staging_column_query,
                &[&self.trend_store_part_name, &self.trend_name],
            )
            .await?;

        if rows.is_empty() {
            return Err(TrendRemoveError::NoSuchTrend);
        }

        let deleted_staging_column: Option<String> = rows.first().unwrap().get(0);

        let column_name = deleted_staging_column.unwrap_or(self.trend_name.clone());

        let drop_column_query = format!(
            "ALTER TABLE trend.{} DROP COLUMN {}",
            escape_identifier(&self.trend_store_part_name),
            escape_identifier(&column_name),
        );

        client.execute(&drop_column_query, &[]).await?;

        let drop_staging_table_column_query = format!(
            "ALTER TABLE trend.{} DROP COLUMN {}",
            escape_identifier(&format!("{}_staging", self.trend_store_part_name)),
            escape_identifier(&column_name),
        );

        client
            .execute(&drop_staging_table_column_query, &[])
            .await?;

        let update_query = concat!(
            "UPDATE trend_directory.table_trend tt ",
            "SET deleted = now(), deletion_staging_column = NULL ",
            "FROM trend_directory.trend_store_part tsp ",
            "WHERE tsp.id = tt.trend_store_part_id AND tsp.name = $1 AND tt.name = $2"
        );

        let affected_records = client
            .execute(
                update_query,
                &[&self.trend_store_part_name, &self.trend_name],
            )
            .await?;

        if affected_records == 0 {
            return Err(TrendRemoveError::NoSuchTrend);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveTrends {
    pub trend_store_part_name: String,
    pub trends: Vec<String>,
}

impl fmt::Display for RemoveTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "RemoveTrends(TrendStorePart({}), {}):",
            &self.trend_store_part_name,
            self.trends.len()
        )?;

        for t in &self.trends {
            writeln!(f, " - {}", &t)?;
        }

        Ok(())
    }
}

impl fmt::Debug for RemoveTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RemoveTrends(TrendStorePart({}), {})",
            &self.trend_store_part_name,
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
#[typetag::serde]
impl Change for RemoveTrends {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut trends: Vec<Trend> = Vec::new();
        let mut tx = client.transaction().await?;

        for trend_name in &self.trends {
            let trend: Trend = load_trend(&mut tx, &self.trend_store_part_name, trend_name)
                .await
                .map_err(|e| {
                    DatabaseError::from_msg(format!(
                        "Could not load trend '{}' definition from the database: {}",
                        &trend_name, e
                    ))
                })?;

            trends.push(trend);

            let remove = TrendRemove {
                trend_store_part_name: self.trend_store_part_name.clone(),
                trend_name: trend_name.clone(),
            };

            remove.remove(&mut tx).await.map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error removing trend '{}' from trend store part: {}",
                    &trend_name, e
                ))
            })?;
        }

        tx.commit().await?;

        Ok(Box::new(RemovedTrends {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends,
        }))
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        vec![Box::new(TrendRemoveValueInformation {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trend_names: self.trends.to_vec(),
        })]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct RemovedTrends {
    pub trend_store_part_name: String,
    pub trends: Vec<Trend>,
}

impl Display for RemovedTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Removed {} trends from trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for RemovedTrends {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddTrends {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends: self.trends.clone(),
        }))
    }
}

////////////
// AddTrends
////////////

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddTrends {
    pub trend_store_part_name: String,
    pub trends: Vec<Trend>,
}

impl fmt::Display for AddTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "AddTrends(TrendStorePart({}), {}):",
            &self.trend_store_part_name,
            &self.trends.len()
        )?;

        for t in &self.trends {
            writeln!(f, " - {}: {}", &t.name, &t.data_type)?;
        }

        Ok(())
    }
}

impl fmt::Debug for AddTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrends(TrendStorePart({}), {})",
            &self.trend_store_part_name,
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
#[typetag::serde]
impl Change for AddTrends {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_table_trends(&mut tx, &self.trend_store_part_name, &self.trends)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!("Error adding trends to trend store part: {e}"))
            })?;

        tx.commit().await?;

        Ok(Box::new(AddedTrends {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends: self.trends.iter().map(|t| t.name.clone()).collect(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct AddedTrends {
    pub trend_store_part_name: String,
    pub trends: Vec<String>,
}

impl Display for AddedTrends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Added {} trends to trend store part '{}'",
            &self.trends.len(),
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for AddedTrends {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveTrends {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trends: self.trends.clone(),
        }))
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

    let alter_staging_table_query = format!(
        "ALTER TABLE {}.{} {}",
        BASE_TABLE_SCHEMA,
        escape_identifier(&format!("{}_staging", trend_store_part_name)),
        column_specs
    );

    client.execute(&alter_staging_table_query, &[]).await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddAliasColumn {
    pub trend_store_part_name: String,
}

impl fmt::Display for AddAliasColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "AddAlias(TrendStorePart({})):",
            &self.trend_store_part_name
        )?;

        Ok(())
    }
}

async fn add_alias_column<T: GenericClient>(
    client: &mut T,
    trend_store_part_name: &str,
) -> Result<(), tokio_postgres::Error> {
    let update_query =
        "UPDATE trend_directory.trend_store_part SET primary_alias = true WHERE name = $1";

    client
        .execute(update_query, &[&trend_store_part_name])
        .await?;

    let query = format!(
        "ALTER TABLE trend.{} ADD COLUMN name text",
        escape_identifier(trend_store_part_name)
    );

    client.execute(&query, &[]).await?;

    Ok(())
}

#[async_trait]
#[typetag::serde]
impl Change for AddAliasColumn {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        add_alias_column(&mut tx, &self.trend_store_part_name)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error adding alias column to trend store part: {e}"
                ))
            })?;

        tx.commit().await?;

        Ok(Box::new(AddedAliasColumn {
            trend_store_part_name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedAliasColumn {
    pub trend_store_part_name: String,
}

impl Display for AddedAliasColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Added alias column to trend store part '{}'",
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for AddedAliasColumn {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveAliasColumn {
            trend_store_part_name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveAliasColumn {
    pub trend_store_part_name: String,
}

impl fmt::Display for RemoveAliasColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RemoveAlias({}):", &self.trend_store_part_name)?;

        Ok(())
    }
}

async fn remove_alias_column<T: GenericClient>(
    client: &mut T,
    trend_store_part_name: &str,
) -> Result<(), tokio_postgres::Error> {
    let query = concat!(
        "SELECT trend_directory.remove_name_column(tsp) ",
        "FROM trend_directory.trend_store_part tsp ",
        "WHERE tsp.name = $1"
    );
    client.execute(query, &[&trend_store_part_name]).await?;

    Ok(())
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveAliasColumn {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        remove_alias_column(&mut tx, &self.trend_store_part_name)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error removing alias column to trend store part: {e}"
                ))
            })?;

        tx.commit().await?;

        Ok(Box::new(RemovedAliasColumn {
            trend_store_part_name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedAliasColumn {
    pub trend_store_part_name: String,
}

impl Display for RemovedAliasColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Removed alias column to trend store part '{}'",
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for RemovedAliasColumn {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddAliasColumn {
            trend_store_part_name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
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
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ModifyTrendDataTypes {
    pub trend_store_part_name: String,
    pub modifications: Vec<ModifyTrendDataType>,
    pub total_trend_count: usize,
}

impl fmt::Display for ModifyTrendDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ModifyTrendDataTypes({}, {}/{}):",
            &self.trend_store_part_name,
            self.modifications.len(),
            self.total_trend_count,
        )?;

        for m in &self.modifications {
            writeln!(f, " - {}: {} -> {}", m.trend_name, m.from_type, m.to_type)?;
        }

        Ok(())
    }
}

impl fmt::Debug for ModifyTrendDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifications: Vec<String> =
            self.modifications.iter().map(|m| format!("{m}")).collect();

        write!(
            f,
            "ModifyTrendDataTypes({}, {})",
            &self.trend_store_part_name,
            &modifications.join(", "),
        )
    }
}

#[async_trait]
#[typetag::serde]
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
                        &self.trend_store_part_name,
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
            "ALTER TABLE trend.{} {}",
            escape_identifier(&self.trend_store_part_name),
            &alter_type_parts_str
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

        Ok(Box::new(ModifiedTrendDataTypes {
            trend_store_part_name: self.trend_store_part_name.clone(),
            modifications: self.modifications.clone(),
            total_trend_count: self.total_trend_count,
        }))
    }

    fn existing_object(&self) -> Option<MinervaObjectRef> {
        Some(MinervaObjectRef::TrendStorePart(
            self.trend_store_part_name.clone(),
        ))
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        vec![Box::new(TrendTypeChangeValueInformation {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trend_changes: self.modifications.clone(),
        })]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ModifiedTrendDataTypes {
    pub trend_store_part_name: String,
    pub modifications: Vec<ModifyTrendDataType>,
    pub total_trend_count: usize,
}

impl Display for ModifiedTrendDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Altered trend data types for trend store part '{}'",
            &self.trend_store_part_name
        )
    }
}

#[typetag::serde]
impl Changed for ModifiedTrendDataTypes {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(ModifyTrendDataTypes {
            trend_store_part_name: self.trend_store_part_name.clone(),
            modifications: self
                .modifications
                .iter()
                .map(|m| ModifyTrendDataType {
                    trend_name: m.trend_name.clone(),
                    from_type: m.to_type,
                    to_type: m.from_type,
                })
                .collect(),
            total_trend_count: self.total_trend_count,
        }))
    }
}

pub struct TrendExtraDiff {
    pub from_extra_data: Value,
    pub to_extra_data: Value,
}

#[async_trait]
impl InformationOption for TrendExtraDiff {
    fn name(&self) -> String {
        "Show diff".to_string()
    }

    async fn retrieve(&self, _client: &mut Client) -> Vec<String> {
        let from = serde_json::to_string_pretty(&self.from_extra_data).unwrap();
        let to = serde_json::to_string_pretty(&self.to_extra_data).unwrap();

        let diff = TextDiff::from_lines(&from, &to);

        diff.iter_all_changes()
            .map(|c| {
                let (sign, s) = match c.tag() {
                    ChangeTag::Delete => ("-", Style::new().red().bold()),
                    ChangeTag::Insert => ("+", Style::new().green().bold()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                s.apply_to(format!("{}{}", sign, c.to_string().trim_end()))
                    .to_string()
            })
            .collect()
    }
}

impl Display for TrendExtraDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name())
    }
}

pub struct TrendTypeChangeValueInformation {
    pub trend_store_part_name: String,
    pub trend_changes: Vec<ModifyTrendDataType>,
}

#[async_trait]
impl InformationOption for TrendTypeChangeValueInformation {
    fn name(&self) -> String {
        "Show trend value information".to_string()
    }

    async fn retrieve(&self, client: &mut Client) -> Vec<String> {
        let trend_store_row = client
            .query_one("SELECT granularity::text FROM trend_directory.trend_store ts JOIN trend_directory.trend_store_part tsp on tsp.trend_store_id = ts.id WHERE tsp.name = $1", &[&self.trend_store_part_name])
            .await
            .unwrap();

        let granularity_str: String = trend_store_row.get(0);
        let granularity = parse_interval(&granularity_str).unwrap();

        let expressions: Vec<String> = self
            .trend_changes
            .iter()
            .map(|trend_change| {
                format!(
                    "max({})::numeric",
                    escape_identifier(&trend_change.trend_name)
                )
            })
            .collect();
        let expressions_part: String = expressions.join(", ");
        let query = format!(
            "SELECT {} FROM trend.{} WHERE timestamp > $1",
            expressions_part,
            escape_identifier(&self.trend_store_part_name)
        );

        let timestamp_threshold = Utc::now() - (granularity * 10);

        let row = client
            .query_one(&query, &[&timestamp_threshold])
            .await
            .unwrap();

        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["trend", "from", "to", "max"]);

        for (index, trend_change) in self.trend_changes.iter().enumerate() {
            let max = row.get::<usize, Option<Decimal>>(index);

            let value = match max {
                Some(v) => format!("{v}"),
                None => "-".to_string(),
            };

            table.add_row(vec![
                Cell::new(trend_change.trend_name.clone()),
                Cell::new(trend_change.from_type),
                Cell::new(trend_change.to_type),
                Cell::new(value).set_alignment(CellAlignment::Right),
            ]);
        }

        table.lines().collect()
    }
}

impl Display for TrendTypeChangeValueInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name())
    }
}

pub struct TrendRemoveValueInformation {
    pub trend_store_part_name: String,
    pub trend_names: Vec<String>,
}

#[async_trait]
impl InformationOption for TrendRemoveValueInformation {
    fn name(&self) -> String {
        "Show trend value information".to_string()
    }

    async fn retrieve(&self, client: &mut Client) -> Vec<String> {
        let trend_store_row = client
            .query_one("SELECT granularity::text FROM trend_directory.trend_store ts JOIN trend_directory.trend_store_part tsp on tsp.trend_store_id = ts.id WHERE tsp.name = $1", &[&self.trend_store_part_name])
            .await
            .unwrap();

        let granularity_str: String = trend_store_row.get(0);
        let granularity = parse_interval(&granularity_str).unwrap();

        let expressions: Vec<String> = self
            .trend_names
            .iter()
            .map(|trend_name| format!("max({})::numeric", escape_identifier(trend_name)))
            .collect();
        let expressions_part: String = expressions.join(", ");
        let query = format!(
            "SELECT {} FROM trend.{} WHERE timestamp > $1",
            expressions_part,
            escape_identifier(&self.trend_store_part_name)
        );

        let timestamp_threshold = Utc::now() - (granularity * 10);

        let row = client
            .query_one(&query, &[&timestamp_threshold])
            .await
            .unwrap();

        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL_CONDENSED)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["trend", "max"]);

        for (index, trend_name) in self.trend_names.iter().enumerate() {
            let max = row.get::<usize, Option<Decimal>>(index);

            let value = match max {
                Some(v) => format!("{v}"),
                None => "-".to_string(),
            };

            table.add_row(vec![Cell::new(trend_name.clone()), Cell::new(value)]);
        }

        table.lines().collect()
    }
}

impl Display for TrendRemoveValueInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
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
            "Trend({}.{})",
            &self.trend_store_part_name, &self.trend_name,
        )
    }
}

#[async_trait]
#[typetag::serde]
impl Change for ModifyTrendExtraData {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let original_trend =
            load_trend(&mut tx, &self.trend_store_part_name, &self.trend_name).await?;

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

        Ok(Box::new(ModifiedTrendExtraData {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trend_name: self.trend_name.clone(),
            from_extra_data: original_trend.extra_data.clone(),
            to_extra_data: self.to_extra_data.clone(),
        }))
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        vec![Box::new(TrendExtraDiff {
            from_extra_data: self.from_extra_data.clone(),
            to_extra_data: self.to_extra_data.clone(),
        })]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ModifiedTrendExtraData {
    pub trend_store_part_name: String,
    pub trend_name: String,
    pub from_extra_data: Value,
    pub to_extra_data: Value,
}

impl Display for ModifiedTrendExtraData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Altered extra_data for trend '{}'.'{}'",
            &self.trend_store_part_name, &self.trend_name
        )
    }
}

#[typetag::serde]
impl Changed for ModifiedTrendExtraData {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(ModifyTrendExtraData {
            trend_store_part_name: self.trend_store_part_name.clone(),
            trend_name: self.trend_name.clone(),
            from_extra_data: self.to_extra_data.clone(),
            to_extra_data: self.from_extra_data.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddTrendStorePart {
    pub trend_store: TrendStoreRef,
    pub trend_store_part: TrendStorePart,
}

const BASE_TABLE_SCHEMA: &str = "trend";

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

impl fmt::Display for AddTrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrendStorePart({}, {})",
            &self.trend_store, &self.trend_store_part
        )
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddTrendStorePart {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let trend_store_id = get_trend_store_id(
            &tx,
            &self.trend_store.data_source,
            &self.trend_store.entity_type,
            &self.trend_store.granularity,
        )
        .await?;

        create_trend_store_part(&mut tx, trend_store_id, &self.trend_store_part)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error creating trend store part '{}': {}",
                    &self.trend_store_part.name, e
                ))
            })?;

        tx.commit().await?;

        Ok(Box::new(AddedTrendStorePart {
            trend_store: self.trend_store.clone(),
            trend_store_part_name: self.trend_store_part.name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedTrendStorePart {
    pub trend_store: TrendStoreRef,
    pub trend_store_part_name: String,
}

impl Display for AddedTrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Added trend store part '{}' to trend store '{}'",
            &self.trend_store_part_name, &self.trend_store
        )
    }
}

#[typetag::serde]
impl Changed for AddedTrendStorePart {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveTrendStorePart {
            name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveTrendStorePart {
    pub name: String,
}

impl fmt::Display for RemoveTrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoveTrendStorePart({})", &self.name)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveTrendStorePart {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let trend_store_part = load_trend_store_part(&tx, &self.name)
            .await
            .map_err(|e| RuntimeError::from_msg(format!("{e}")))?;
        let trend_store_ref = load_trend_store_ref_for_part(&tx, &self.name)
            .await
            .map_err(RuntimeError::from_msg)?;

        remove_trend_store_part(&mut tx, &self.name)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error removing trend store part '{}': {}",
                    &self.name, e
                ))
            })?;

        tx.commit().await?;

        Ok(Box::new(RemovedTrendStorePart {
            trend_store: trend_store_ref,
            trend_store_part,
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedTrendStorePart {
    pub trend_store: TrendStoreRef,
    pub trend_store_part: TrendStorePart,
}

impl Display for RemovedTrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Removed trend store part '{}'",
            &self.trend_store_part.name
        )
    }
}

#[typetag::serde]
impl Changed for RemovedTrendStorePart {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddTrendStorePart {
            trend_store: self.trend_store.clone(),
            trend_store_part: self.trend_store_part.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddTrendStore {
    pub trend_store: TrendStore,
}

impl fmt::Display for AddTrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AddTrendStore({})", &self.trend_store)?;

        for part in &self.trend_store.parts {
            writeln!(f, " - {}", &part.name)?;
        }

        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddTrendStore {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_trend_store(&mut tx, &self.trend_store)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error creating trend store: {e}")))?;

        tx.commit().await?;

        Ok(Box::new(AddedTrendStore {
            trend_store: (&self.trend_store).into(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedTrendStore {
    pub trend_store: TrendStoreRef,
}

impl Display for AddedTrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Added trend store {}", &self.trend_store)
    }
}

#[typetag::serde]
impl Changed for AddedTrendStore {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveTrendStore {
            trend_store: self.trend_store.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveTrendStore {
    pub trend_store: TrendStoreRef,
}

impl fmt::Display for RemoveTrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoveTrendStore({})", self.trend_store)?;

        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveTrendStore {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        // Load the trend store we are about to delete to store it for later reverting this change
        let trend_store = load_trend_store(&tx, &self.trend_store)
            .await
            .map_err(|e| {
                RuntimeError::from_msg(format!(
                    "Could not load trend store {}: {e}",
                    &self.trend_store
                ))
            })?;

        remove_trend_store(&mut tx, &self.trend_store)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error removing trend store: {e}")))?;

        tx.commit().await?;

        Ok(Box::new(RemovedTrendStore { trend_store }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedTrendStore {
    pub trend_store: TrendStore,
}

impl Display for RemovedTrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Removed trend store {}", self.trend_store)
    }
}

#[typetag::serde]
impl Changed for RemovedTrendStore {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddTrendStore {
            trend_store: self.trend_store.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateStatistics {
    pub trend_store_part_name: Option<String>,
}

impl fmt::Display for CreateStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.trend_store_part_name {
            Some(tsp) => writeln!(f, "CreateStatitics({})", &tsp)?,
            None => writeln!(f, "CreateAllStatitics()")?,
        };
        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for CreateStatistics {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        match &self.trend_store_part_name {
            Some(tsp) => {
                let query = concat!(
                    "SELECT trend_directory.update_statistics(tt) ",
                    "FROM trend_directory.table_trend tt ",
                    "JOIN trend_directory.trend_store_part tsp ",
                    "ON tt.trend_store_part_id = tsp.id ",
                    "WHERE tsp.name = {}",
                );
                client.execute(query, &[&tsp]).await?;
            }
            None => {
                let query = concat!(
                    "SELECT trend_directory.update_statistics(tt) ",
                    "FROM trend_directory.table_trend tt",
                );
                client.execute(query, &[]).await?;
            }
        }

        Ok(Box::new(CreatedStatistics {
            trend_store_part_name: self.trend_store_part_name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct CreatedStatistics {
    pub trend_store_part_name: Option<String>,
}

impl Display for CreatedStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.trend_store_part_name {
            Some(name) => {
                write!(f, "Created statistics for trend store part {}", &name)
            }
            None => {
                write!(f, "Created statistics for all trend store parts")
            }
        }
    }
}

#[typetag::serde]
impl Changed for CreatedStatistics {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
    }
}
