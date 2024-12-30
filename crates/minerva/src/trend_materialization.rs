use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_yaml;
use std::collections::HashMap;
use std::fmt;
use std::marker::{Send, Sync};
use std::path::Path;
use std::time::Duration;
use tokio_postgres::{Row, Transaction};

use postgres_protocol::escape::escape_identifier;
use thiserror::Error;
use tokio_postgres::{types::ToSql, types::Type, GenericClient};

use humantime::format_duration;

use async_trait::async_trait;

use crate::error::ConfigurationError;

use super::change::{Change, ChangeResult};
use super::error::{DatabaseError, Error, RuntimeError};
use super::interval::parse_interval;

pub const MATERIALIZATION_FUNCTION_SCHEMA: &str = "trend";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrendMaterializationSource {
    pub trend_store_part: String,
    pub mapping_function: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrendViewMaterialization {
    pub target_trend_store_part: String,
    pub enabled: bool,
    #[serde(with = "humantime_serde")]
    pub processing_delay: Duration,
    #[serde(with = "humantime_serde")]
    pub stability_delay: Duration,
    #[serde(with = "humantime_serde")]
    pub reprocessing_period: Duration,
    pub sources: Vec<TrendMaterializationSource>,
    pub view: String,
    pub fingerprint_function: String,
    pub description: Option<Value>,
}

impl TrendViewMaterialization {
    async fn define_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = concat!(
        "SELECT trend_directory.define_view_materialization(",
        "id, $1::text::interval, $2::text::interval, $3::text::interval, $4::text::regclass, $5::jsonb",
        ") ",
        "FROM trend_directory.trend_store_part WHERE name = $6",
        );

        let description_default = serde_json::json!("{}");

        let query_args: &[&(dyn ToSql + Sync)] = &[
            &format_duration(self.processing_delay).to_string(),
            &format_duration(self.stability_delay).to_string(),
            &format_duration(self.reprocessing_period).to_string(),
            &format!("trend.{}", escape_identifier(&self.view_name())),
            &self.description.as_ref().unwrap_or(&description_default),
            &self.target_trend_store_part,
        ];

        match client.query(query, query_args).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error defining view materialization: {e}"
            )))),
        }
    }

    fn view_name(&self) -> String {
        format!("_{}", &self.target_trend_store_part)
    }

    pub async fn drop_view<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            "DROP VIEW IF EXISTS trend.{}",
            &escape_identifier(&self.view_name()),
        );

        match client.execute(query.as_str(), &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error dropping view: {e}"
            )))),
        }
    }

    pub async fn create_view<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            "CREATE VIEW trend.{} AS {}",
            &escape_identifier(&self.view_name()),
            self.view,
        );

        match client.execute(query.as_str(), &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error creating view: {e}"
            )))),
        }
    }

    pub async fn init_view_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let view_ident = format!("trend.{}", &escape_identifier(&self.view_name()));

        let query = concat!(
            "INSERT INTO trend_directory.view_materialization(materialization_id, src_view) ",
            "SELECT m.id, $2::text::regclass ",
            "FROM trend_directory.materialization m ",
            "JOIN trend_directory.trend_store_part dstp ",
            "ON m.dst_trend_store_part_id = dstp.id ",
            "WHERE dstp.name = $1"
        );

        client
            .execute(query, &[&self.target_trend_store_part, &view_ident])
            .await
            .map_err(|e| {
                Error::Database(DatabaseError::from_msg(format!(
                    "Error initializing view materialization: {e}"
                )))
            })?;

        Ok(())
    }

    async fn create<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.create_view(client).await?;
        self.define_materialization(client).await?;
        self.connect_sources(client).await?;
        create_fingerprint_function(
            client,
            &self.target_trend_store_part,
            &self.fingerprint_function,
        )
        .await?;

        Ok(())
    }

    pub fn diff(&self, other: &TrendViewMaterialization) -> Vec<Box<dyn Change + Send>> {
        let mut changes: Vec<Box<dyn Change + Send>> = Vec::new();

        // Comparing a view from the database with a view definition is not
        // really usefull because PostgreSQL rewrites the SQL.
        //if self.view != other.view {
        //    changes.push(Box::new(UpdateView { trend_view_materialization:
        // self.clone() }));
        //}

        if self.enabled != other.enabled
            || self.processing_delay != other.processing_delay
            || self.stability_delay != other.stability_delay
            || self.reprocessing_period != other.reprocessing_period
        {
            changes.push(Box::new(UpdateTrendViewMaterializationAttributes {
                trend_view_materialization: other.clone(),
            }));
        }

        changes
    }

    async fn update<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.create_view(client).await?;
        self.init_view_materialization(client).await?;
        create_fingerprint_function(
            client,
            &self.target_trend_store_part,
            &self.fingerprint_function,
        )
        .await?;
        self.connect_sources(client).await?;
        self.update_attributes(client).await?;

        Ok(())
    }

    async fn connect_sources<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        connect_materialization_sources(client, &self.target_trend_store_part, &self.sources).await
    }

    async fn delete<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.drop_view(client).await?;
        drop_fingerprint_function(client, &self.target_trend_store_part).await?;
        Ok(())
    }

    async fn update_attributes<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            concat!(
                "UPDATE trend_directory.materialization ",
                "SET processing_delay = $1::text::interval, ",
                "stability_delay = $2::text::interval, ",
                "reprocessing_period = $3::text::interval, ",
                "enabled = $4, ",
                "description = '{}'::jsonb ",
                "WHERE materialization::text = $5",
            ),
            &self
                .description
                .as_ref()
                .unwrap_or(&serde_json::json!("{}"))
                .to_string()
        );

        let query_args: &[&(dyn ToSql + Sync)] = &[
            &format_duration(self.processing_delay).to_string(),
            &format_duration(self.stability_delay).to_string(),
            &format_duration(self.reprocessing_period).to_string(),
            &self.enabled,
            &self.target_trend_store_part,
        ];

        match client.execute(&query, query_args).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error updating view materialization attributes: {e}"
            )))),
        }
    }
}

fn fingerprint_function_name(materialization_name: &str) -> String {
    format!("{}_fingerprint", materialization_name)
}

async fn create_fingerprint_function<T: GenericClient + Send + Sync>(
    client: &mut T,
    materialization_name: &str,
    function_body: &str,
) -> Result<(), Error> {
    let query = format!(concat!(
        "CREATE FUNCTION trend.{}(timestamp with time zone) RETURNS trend_directory.fingerprint AS $$\n",
        "{}\n",
        "$$ LANGUAGE sql STABLE\n"
    ), escape_identifier(&fingerprint_function_name(materialization_name)), function_body);

    match client.query(query.as_str(), &[]).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
            "Error creating fingerprint function: {e}"
        )))),
    }
}

async fn drop_fingerprint_function<T: GenericClient + Send + Sync>(
    client: &mut T,
    materialization_name: &str,
) -> Result<(), Error> {
    let query = format!(
        "DROP FUNCTION IF EXISTS trend.{}(timestamp with time zone)",
        escape_identifier(&fingerprint_function_name(materialization_name))
    );

    match client.query(query.as_str(), &[]).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
            "Error dropping fingerprint function: {e}"
        )))),
    }
}

pub struct UpdateTrendViewMaterializationAttributes {
    pub trend_view_materialization: TrendViewMaterialization,
}

#[async_trait]
impl Change for UpdateTrendViewMaterializationAttributes {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        self.trend_view_materialization
            .update_attributes(client)
            .await?;

        Ok("Updated attributes of view materialization".into())
    }
}

impl fmt::Display for UpdateTrendViewMaterializationAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpdateTrendViewMaterializationAttributes({})",
            &self.trend_view_materialization.target_trend_store_part,
        )
    }
}

pub struct UpdateView {
    pub trend_view_materialization: TrendViewMaterialization,
}

#[async_trait]
impl Change for UpdateView {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        self.trend_view_materialization
            .drop_view(client)
            .await
            .unwrap();
        self.trend_view_materialization
            .create_view(client)
            .await
            .unwrap();

        Ok(format!(
            "Updated view {}",
            self.trend_view_materialization.view_name()
        ))
    }
}

impl fmt::Display for UpdateView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpdateView({}, {})",
            &self.trend_view_materialization.target_trend_store_part,
            &self.trend_view_materialization.view_name()
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrendMaterializationFunction {
    pub return_type: String,
    pub src: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrendFunctionMaterialization {
    pub target_trend_store_part: String,
    pub enabled: bool,
    #[serde(with = "humantime_serde")]
    pub processing_delay: Duration,
    #[serde(with = "humantime_serde")]
    pub stability_delay: Duration,
    #[serde(with = "humantime_serde")]
    pub reprocessing_period: Duration,
    pub sources: Vec<TrendMaterializationSource>,
    pub function: TrendMaterializationFunction,
    pub fingerprint_function: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Value>,
}

impl TrendFunctionMaterialization {
    async fn define_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = concat!(
        "SELECT trend_directory.define_function_materialization(",
        "id, $1::text::interval, $2::text::interval, $3::text::interval, $4::text::regprocedure, $5::jsonb",
        ") ",
        "FROM trend_directory.trend_store_part WHERE name = $6",
        );

        let description_default = serde_json::json!("{}");

        let query_args: &[&(dyn ToSql + Sync)] = &[
            &format_duration(self.processing_delay).to_string(),
            &format_duration(self.stability_delay).to_string(),
            &format_duration(self.reprocessing_period).to_string(),
            &format!(
                "trend.{}(timestamp with time zone)",
                escape_identifier(&self.target_trend_store_part)
            ),
            &self.description.as_ref().unwrap_or(&description_default),
            &self.target_trend_store_part,
        ];

        match client.query(query, query_args).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error defining function materialization: {e}"
            )))),
        }
    }

    async fn create_function<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            "CREATE FUNCTION trend.{}(timestamp with time zone) RETURNS {} AS $function$\n{}\n$function$ LANGUAGE {}",
            &escape_identifier(&self.target_trend_store_part),
            &self.function.return_type,
            &self.function.src,
            &self.function.language,
        );

        match client.execute(query.as_str(), &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error creating function: {e}"
            )))),
        }
    }

    async fn init_function_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let function_ident = format!(
            "trend.{}",
            &escape_identifier(&self.target_trend_store_part)
        );

        let query = concat!(
            "INSERT INTO trend_directory.function_materialization(materialization_id, src_function) ",
            "SELECT m.id, $2::text::regproc ",
            "FROM trend_directory.materialization m ",
            "JOIN trend_directory.trend_store_part dstp ",
            "ON m.dst_trend_store_part_id = dstp.id ",
            "WHERE dstp.name = $1"
        );

        client
            .execute(query, &[&self.target_trend_store_part, &function_ident])
            .await
            .map_err(|e| {
                Error::Database(DatabaseError::from_msg(format!(
                    "Error initializing function materialization: {e}"
                )))
            })?;

        Ok(())
    }

    async fn create<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.create_function(client).await?;
        create_fingerprint_function(
            client,
            &self.target_trend_store_part,
            &self.fingerprint_function,
        )
        .await?;
        self.define_materialization(client).await?;
        if self.enabled {
            self.do_enable(client).await?
        };
        self.connect_sources(client).await?;

        Ok(())
    }

    async fn do_enable<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        let query = concat!(
            "UPDATE trend_directory.materialization AS m ",
            "SET enabled = true ",
            "FROM trend_directory.trend_store_part AS dtsp ",
            "WHERE m.dst_trend_store_part_id = dtsp.id ",
            "AND dtsp.name = $1"
        );
        match client.query(query, &[&self.target_trend_store_part]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Unable to enable materialization: {e}"
            )))),
        }
    }

    async fn connect_sources<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        connect_materialization_sources(client, &self.target_trend_store_part, &self.sources).await
    }

    async fn drop_sources<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            concat!(
        "DELETE FROM trend_directory.materialization_trend_store_link tsl ",
        "USING trend_directory.materialization m JOIN trend_directory.trend_store_part dstp ",
        "ON m.dst_trend_store_part_id = dstp.id ",
        "WHERE dstp.name = '{}' AND tsl.materialization_id = m.id"
        ),
            &self.target_trend_store_part
        );
        match client.query(&query, &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error removing old sources: {e}"
            )))),
        }
    }

    pub fn diff(&self, _other: &TrendFunctionMaterialization) -> Vec<Box<dyn Change + Send>> {
        Vec::new()
    }

    async fn update_attributes<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = format!(
            concat!(
                "UPDATE trend_directory.materialization ",
                "SET processing_delay = $1::text::interval, ",
                "stability_delay = $2::text::interval, ",
                "reprocessing_period = $3::text::interval, ",
                "enabled = $4, ",
                "description = '{}'::jsonb ",
                "WHERE materialization::text = $5",
            ),
            &self
                .description
                .as_ref()
                .unwrap_or(&serde_json::json!("{}"))
                .to_string()
        );

        let query_args: &[&(dyn ToSql + Sync)] = &[
            &format_duration(self.processing_delay).to_string(),
            &format_duration(self.stability_delay).to_string(),
            &format_duration(self.reprocessing_period).to_string(),
            &self.enabled,
            &self.target_trend_store_part,
        ];

        match client.execute(&query, query_args).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error updating view materialization attributes: {e}"
            )))),
        }
    }

    async fn drop_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = "DELETE FROM trend_directory.materialization WHERE materialization::text = $1";
        match client
            .execute(query, &[&self.target_trend_store_part])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Database(DatabaseError::from_msg(format!(
                "Error deleting view materialization: {e}"
            )))),
        }
    }

    async fn update<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.create_function(client).await?;
        self.init_function_materialization(client).await?;
        create_fingerprint_function(
            client,
            &self.target_trend_store_part,
            &self.fingerprint_function,
        )
        .await?;
        self.connect_sources(client).await?;
        self.update_attributes(client).await?;

        Ok(())
    }

    async fn delete<T: GenericClient + Send + Sync>(&self, client: &mut T) -> Result<(), Error> {
        self.drop_sources(client).await?;
        self.drop_materialization(client).await?;
        drop_fingerprint_function(client, &self.target_trend_store_part).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TrendMaterialization {
    View(TrendViewMaterialization),
    Function(TrendFunctionMaterialization),
}

impl fmt::Display for TrendMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrendMaterialization::View(view_materialization) => write!(
                f,
                "TrendViewMaterialization('{}')",
                &view_materialization.target_trend_store_part
            ),
            TrendMaterialization::Function(function_materialization) => write!(
                f,
                "TrendFunctionMaterialization('{}')",
                &function_materialization.target_trend_store_part
            ),
        }
    }
}

impl TrendMaterialization {
    pub fn name(&self) -> &str {
        match self {
            TrendMaterialization::View(m) => &m.target_trend_store_part,
            TrendMaterialization::Function(m) => &m.target_trend_store_part,
        }
    }

    pub fn dump(&self) -> Result<String, Error> {
        match self {
            TrendMaterialization::View(m) => serde_yaml::to_string(m).map_err(|e| {
                Error::Runtime(RuntimeError::from_msg(format!(
                    "Could not dump view materialization: {e}"
                )))
            }),
            TrendMaterialization::Function(m) => serde_yaml::to_string(m).map_err(|e| {
                Error::Runtime(RuntimeError::from_msg(format!(
                    "Could not dump function materialization: {e}"
                )))
            }),
        }
    }

    fn fingerprint_function_name(&self) -> String {
        format!("{}_fingerprint", self.name())
    }

    pub async fn drop_sources<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = concat!(
            "DELETE FROM trend_directory.materialization_trend_store_link tsl ",
            "USING trend_directory.materialization m ",
            "JOIN trend_directory.trend_store_part dstp ",
            "ON m.dst_trend_store_part_id = dstp.id ",
            "WHERE tsl.materialization_id = m.id AND dstp.name = $1"
        );

        client.execute(query, &[&self.name()]).await.map_err(|e| {
            Error::Database(DatabaseError::from_msg(format!(
                "Error removing materialization_trend_store_link records: {e}"
            )))
        })?;

        Ok(())
    }

    /// Tear down all implementation details of the materialization
    ///
    /// Keeps the materialization record and any attached materialization state, but removes all
    /// implementation details such as materialization function or view, source trend store part
    /// links. Looks for both function and view materialization implementation so this can be used
    /// to switch between function and view materialization implementation.
    pub async fn teardown<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        let query = concat!(
            "DELETE FROM trend_directory.view_materialization vm ",
            "USING trend_directory.materialization m ",
            "JOIN trend_directory.trend_store_part dstp ",
            "ON m.dst_trend_store_part_id = dstp.id ",
            "WHERE m.id = vm.materialization_id AND dstp.name = $1"
        );

        client.execute(query, &[&self.name()]).await.map_err(|e| {
            Error::Database(DatabaseError::from_msg(format!(
                "Error removing view_materialization record: {e}"
            )))
        })?;

        let query = concat!(
            "DELETE FROM trend_directory.function_materialization fm ",
            "USING trend_directory.materialization m ",
            "JOIN trend_directory.trend_store_part dstp ",
            "ON m.dst_trend_store_part_id = dstp.id ",
            "WHERE m.id = fm.materialization_id AND dstp.name = $1"
        );

        client.execute(query, &[&self.name()]).await.map_err(|e| {
            Error::Database(DatabaseError::from_msg(format!(
                "Error removing function_materialization record: {e}"
            )))
        })?;

        self.drop_sources(client).await?;

        let fingerprint_function_name = self.fingerprint_function_name();

        let query = format!(
            "DROP FUNCTION IF EXISTS trend.{}(timestamp with time zone)",
            escape_identifier(&fingerprint_function_name)
        );

        client.execute(&query, &[]).await.map_err(|e| {
            Error::Database(DatabaseError::from_msg(format!(
                "Error dropping fingerprint function '{fingerprint_function_name}': {e}"
            )))
        })?;

        Ok(())
    }

    pub async fn update<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        self.teardown(client).await?;

        match self {
            TrendMaterialization::View(m) => m.update(client).await,
            TrendMaterialization::Function(m) => m.update(client).await,
        }
    }

    pub async fn create<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        match self {
            TrendMaterialization::View(m) => m.create(client).await,
            TrendMaterialization::Function(m) => m.create(client).await,
        }
    }

    pub async fn delete<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), Error> {
        match self {
            TrendMaterialization::View(m) => m.delete(client).await,
            TrendMaterialization::Function(m) => m.delete(client).await,
        }
    }

    pub fn diff(&self, other: &TrendMaterialization) -> Vec<Box<dyn Change + Send>> {
        match self {
            TrendMaterialization::View(m) => match other {
                TrendMaterialization::View(other_m) => m.diff(other_m),
                TrendMaterialization::Function(_) => panic!("Incompatible materialization types"),
            },
            TrendMaterialization::Function(m) => match other {
                TrendMaterialization::View(_) => panic!("Incompatible materialization types"),
                TrendMaterialization::Function(other_m) => m.diff(other_m),
            },
        }
    }
}

pub fn trend_materialization_from_config(
    path: &std::path::PathBuf,
) -> Result<TrendMaterialization, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        Error::Runtime(RuntimeError::from_msg(format!(
            "could not open definition file: {e}"
        )))
    })?;
    let deserialize_result: Result<TrendMaterialization, serde_yaml::Error> =
        serde_yaml::from_reader(f);

    match deserialize_result {
        Ok(materialization) => Ok(materialization),
        Err(e) => Err(Error::Runtime(RuntimeError::from_msg(format!(
            "could not deserialize materialization: {e}"
        )))),
    }
}

pub fn load_materializations_from(
    minerva_instance_root: &Path,
) -> impl Iterator<Item = TrendMaterialization> {
    let glob_path = format!(
        "{}/materialization/*.yaml",
        minerva_instance_root.to_string_lossy()
    );

    glob(&glob_path)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => match trend_materialization_from_config(&path) {
                Ok(materialization) => Some(materialization),
                Err(e) => {
                    println!("Error loading materialization '{}': {}", &path.display(), e);
                    None
                }
            },
            Err(_) => None,
        })
}

pub fn map_sql_to_plpgsql(src: String) -> String {
    [
        "BEGIN\n".into(),
        "RETURN QUERY EXECUTE $query$\n".into(),
        src,
        "$query$ USING $1;\n".into(),
        "END;\n".into(),
    ]
    .join("")
}

/// Citus does not support the construct with parameterized queries in plain sql functions. The use
/// of plpgsql functions is a work-around for this.
fn coorce_to_plpgsql((lang, src): (String, String)) -> Result<(String, String), Error> {
    match lang.as_str() {
        "sql" => Ok(("plpgsql".into(), map_sql_to_plpgsql(src))),
        "plpgsql" => Ok((lang, src)),
        _ => Err(Error::Runtime(RuntimeError::from_msg(format!(
            "Unexpected language '{lang}'"
        )))),
    }
}

pub async fn load_materialization<T: GenericClient + Send + Sync>(
    conn: &mut T,
    name: &str,
) -> Result<TrendMaterialization, Error> {
    let query = concat!(
        "SELECT m.id, m.processing_delay::text, m.stability_delay::text, m.reprocessing_period::text, m.enabled, m.description, tsp.name, vm.src_view, fm.src_function ",
        "FROM trend_directory.materialization AS m ",
        "JOIN trend_directory.trend_store_part AS tsp ON tsp.id = m.dst_trend_store_part_id ",
        "LEFT JOIN trend_directory.view_materialization AS vm ON vm.materialization_id = m.id ",
        "LEFT JOIN trend_directory.function_materialization AS fm ON fm.materialization_id = m.id ",
        "WHERE m::text = $1",
    );

    let result = conn.query(query, &[&name]).await.map_err(|e| {
        DatabaseError::from_msg(format!("Error loading trend materialization: {e}"))
    })?;

    if result.len() == 0 {
        return Err(Error::Configuration(ConfigurationError::from_msg(format!(
            "No materialization that matches name '{}'",
            name
        ))));
    }

    let row = result.first().unwrap();

    let materialization_id: i32 = row.get(0);
    let processing_delay_str: String = row.get(1);
    let stability_delay_str: String = row.get(2);
    let reprocessing_period_str: String = row.get(3);
    let enabled: bool = row.get(4);
    let description: Option<Value> = row.get(5);
    let target_trend_store_part: String = row.get(6);
    let src_view: Option<String> = row.get(7);
    let src_function: Option<String> = row.get(8);

    let fingerprint_function_name = format!("{}_fingerprint", &target_trend_store_part);
    let (_fingerprint_function_lang, fingerprint_function_def) =
        get_function_def(conn, &fingerprint_function_name)
            .await
            .unwrap_or((
                "failed getting language".into(),
                "failed getting sources".into(),
            ));

    let processing_delay = parse_interval(&processing_delay_str)
        .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of processing_delay: {e}"))))?;

    let reprocessing_period = parse_interval(&reprocessing_period_str)
        .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of reprocessing_period: {e}"))))?;

    let stability_delay = parse_interval(&stability_delay_str)
        .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of stability_delay: {e}"))))?;

    if let Some(view) = src_view {
        let view_def = get_view_def(conn, &view).await.unwrap();
        let sources = load_sources(conn, materialization_id).await?;

        let view_materialization = TrendViewMaterialization {
            target_trend_store_part: target_trend_store_part.clone(),
            enabled,
            fingerprint_function: fingerprint_function_def.clone(),
            processing_delay,
            reprocessing_period,
            sources,
            stability_delay,
            view: view_def,
            description: description.clone(),
        };

        Ok(TrendMaterialization::View(view_materialization))
    } else if src_function.is_some() {
        let (function_lang, function_def) = coorce_to_plpgsql(
            get_function_def(conn, &target_trend_store_part)
                .await
                .unwrap_or((
                    "failed getting language".into(),
                    "failed getting sources".into(),
                )),
        )?;
        let return_type = get_function_return_type(
            conn,
            MATERIALIZATION_FUNCTION_SCHEMA,
            &target_trend_store_part,
        )
        .await
        .unwrap_or("failed getting return type".into());

        let sources = load_sources(conn, materialization_id).await?;

        let function_materialization = TrendFunctionMaterialization {
            target_trend_store_part: target_trend_store_part.clone(),
            enabled,
            fingerprint_function: fingerprint_function_def.clone(),
            processing_delay,
            reprocessing_period,
            sources,
            stability_delay,
            function: TrendMaterializationFunction {
                return_type,
                src: function_def,
                language: function_lang,
            },
            description: description.clone(),
        };

        Ok(TrendMaterialization::Function(function_materialization))
    } else {
        Err(Error::Runtime(RuntimeError::from_msg("Unexpected configuration where materialization is not a view and not a function materialization".to_string())))
    }
}

#[derive(Error, Debug)]
pub enum LoadTrendMaterializationError {
    #[error("no such view '{0}' could be loaded")]
    NoSuchView(String),
    #[error("could not parse value from database: {0}")]
    ParseError(String),
    #[error("could not load materialization sources: {0}")]
    Sources(String),
    #[error("could not load materialization function: {0}")]
    MaterializationFunction(String),
    #[error("invalid configuration: {0}")]
    Configuration(String),
    #[error("unexpected issue loading materialization: {0}")]
    Unexpected(String),
    #[error("could not find matching materialization: {0}")]
    NotFound(String),
}

async fn trend_materialization_from_row<T: GenericClient + Send + Sync>(
    client: &mut T,
    row: &Row,
) -> Result<TrendMaterialization, LoadTrendMaterializationError> {
    let materialization_id: i32 = row.get(0);
    let processing_delay_str: String = row.get(1);
    let stability_delay_str: String = row.get(2);
    let reprocessing_period_str: String = row.get(3);
    let enabled: bool = row.get(4);
    let description: Option<Value> = row.get(5);
    let target_trend_store_part: String = row.get(6);
    let src_view: Option<String> = row.get(7);
    let src_function: Option<String> = row.get(8);

    let fingerprint_function_name = format!("{}_fingerprint", &target_trend_store_part);
    let (_fingerprint_function_lang, fingerprint_function_def) =
        get_function_def(client, &fingerprint_function_name)
            .await
            .unwrap_or((
                "failed getting language".into(),
                "failed getting sources".into(),
            ));

    let processing_delay = parse_interval(&processing_delay_str)
        .map_err(|e| LoadTrendMaterializationError::ParseError(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of processing_delay: {e}")))?;

    let reprocessing_period = parse_interval(&reprocessing_period_str)
        .map_err(|e| LoadTrendMaterializationError::ParseError(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of reprocessing_period: {e}")))?;

    let stability_delay = parse_interval(&stability_delay_str)
        .map_err(|e| LoadTrendMaterializationError::ParseError(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of stability_delay: {e}")))?;

    if let Some(view) = src_view {
        let view_def = get_view_def(client, &view)
            .await
            .ok_or(LoadTrendMaterializationError::NoSuchView(view))?;
        let sources = load_sources(client, materialization_id)
            .await
            .map_err(|e| LoadTrendMaterializationError::Sources(format!("{e}")))?;

        let view_materialization = TrendViewMaterialization {
            target_trend_store_part: target_trend_store_part.clone(),
            enabled,
            fingerprint_function: fingerprint_function_def.clone(),
            processing_delay,
            reprocessing_period,
            sources,
            stability_delay,
            view: view_def,
            description: description.clone(),
        };

        let trend_materialization = TrendMaterialization::View(view_materialization);

        return Ok(trend_materialization);
    }

    if src_function.is_some() {
        let (function_lang, function_def) = coorce_to_plpgsql(
            get_function_def(client, &target_trend_store_part)
                .await
                .unwrap_or((
                    "failed getting language".into(),
                    "failed getting sources".into(),
                )),
        )
        .map_err(|e| {
            LoadTrendMaterializationError::MaterializationFunction(format!(
                "could not load function definition: {e}"
            ))
        })?;

        let return_type = get_function_return_type(
            client,
            MATERIALIZATION_FUNCTION_SCHEMA,
            &target_trend_store_part,
        )
        .await
        .unwrap_or("failed getting return type".into());

        let sources = load_sources(client, materialization_id)
            .await
            .map_err(|e| LoadTrendMaterializationError::Sources(format!("{e}")))?;

        let function_materialization = TrendFunctionMaterialization {
            target_trend_store_part: target_trend_store_part.clone(),
            enabled,
            fingerprint_function: fingerprint_function_def.clone(),
            processing_delay,
            reprocessing_period,
            sources,
            stability_delay,
            function: TrendMaterializationFunction {
                return_type,
                src: function_def,
                language: function_lang,
            },
            description: description.clone(),
        };

        let trend_materialization = TrendMaterialization::Function(function_materialization);

        return Ok(trend_materialization);
    }

    Err(LoadTrendMaterializationError::Configuration(
        "No function or view materialization could be loaded".to_string(),
    ))
}

pub async fn load_trend_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    name: &str,
) -> Result<TrendMaterialization, LoadTrendMaterializationError> {
    let query = concat!(
        "SELECT m.id, m.processing_delay::text, m.stability_delay::text, m.reprocessing_period::text, m.enabled, m.description, tsp.name, vm.src_view, fm.src_function ",
        "FROM trend_directory.materialization AS m ",
        "JOIN trend_directory.trend_store_part AS tsp ON tsp.id = m.dst_trend_store_part_id ",
        "LEFT JOIN trend_directory.view_materialization AS vm ON vm.materialization_id = m.id ",
        "LEFT JOIN trend_directory.function_materialization AS fm ON fm.materialization_id = m.id ",
        "WHERE m::text = $1",
    );

    let rows = client.query(query, &[&name]).await.map_err(|e| {
        LoadTrendMaterializationError::Unexpected(format!(
            "Error loading trend materialization: {e}"
        ))
    })?;

    if rows.is_empty() {
        return Err(LoadTrendMaterializationError::NotFound(format!(
            "No trend materialization found matching name '{name}'"
        )));
    }

    let row = rows.first().unwrap();

    trend_materialization_from_row(client, row).await
}

pub async fn load_materializations<T: GenericClient + Send + Sync>(
    conn: &mut T,
) -> Result<Vec<TrendMaterialization>, Error> {
    let mut trend_materializations: Vec<TrendMaterialization> = Vec::new();

    let query = concat!(
        "SELECT m.id, m.processing_delay::text, m.stability_delay::text, m.reprocessing_period::text, m.enabled, m.description, tsp.name, vm.src_view, fm.src_function ",
        "FROM trend_directory.materialization AS m ",
        "JOIN trend_directory.trend_store_part AS tsp ON tsp.id = m.dst_trend_store_part_id ",
        "LEFT JOIN trend_directory.view_materialization AS vm ON vm.materialization_id = m.id ",
        "LEFT JOIN trend_directory.function_materialization AS fm ON fm.materialization_id = m.id ",
    );

    let result = conn.query(query, &[]).await.map_err(|e| {
        DatabaseError::from_msg(format!("Error loading trend materializations: {e}"))
    })?;

    for row in result {
        let materialization_id: i32 = row.get(0);
        let processing_delay_str: String = row.get(1);
        let stability_delay_str: String = row.get(2);
        let reprocessing_period_str: String = row.get(3);
        let enabled: bool = row.get(4);
        let description: Option<Value> = row.get(5);
        let target_trend_store_part: String = row.get(6);
        let src_view: Option<String> = row.get(7);
        let src_function: Option<String> = row.get(8);

        let fingerprint_function_name = format!("{}_fingerprint", &target_trend_store_part);
        let (_fingerprint_function_lang, fingerprint_function_def) =
            get_function_def(conn, &fingerprint_function_name)
                .await
                .unwrap_or((
                    "failed getting language".into(),
                    "failed getting sources".into(),
                ));

        let processing_delay = parse_interval(&processing_delay_str)
            .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of processing_delay: {e}"))))?;

        let reprocessing_period = parse_interval(&reprocessing_period_str)
            .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of reprocessing_period: {e}"))))?;

        let stability_delay = parse_interval(&stability_delay_str)
            .map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("Could not load materialization '{target_trend_store_part}' due to failure in parsing of stability_delay: {e}"))))?;

        if let Some(view) = src_view {
            let view_def = get_view_def(conn, &view).await.unwrap();
            let sources = load_sources(conn, materialization_id).await?;

            let view_materialization = TrendViewMaterialization {
                target_trend_store_part: target_trend_store_part.clone(),
                enabled,
                fingerprint_function: fingerprint_function_def.clone(),
                processing_delay,
                reprocessing_period,
                sources,
                stability_delay,
                view: view_def,
                description: description.clone(),
            };

            let trend_materialization = TrendMaterialization::View(view_materialization);

            trend_materializations.push(trend_materialization);
        }

        if src_function.is_some() {
            let (function_lang, function_def) = coorce_to_plpgsql(
                get_function_def(conn, &target_trend_store_part)
                    .await
                    .unwrap_or((
                        "failed getting language".into(),
                        "failed getting sources".into(),
                    )),
            )?;
            let return_type = get_function_return_type(
                conn,
                MATERIALIZATION_FUNCTION_SCHEMA,
                &target_trend_store_part,
            )
            .await
            .unwrap_or("failed getting return type".into());

            let sources = load_sources(conn, materialization_id).await?;

            let function_materialization = TrendFunctionMaterialization {
                target_trend_store_part: target_trend_store_part.clone(),
                enabled,
                fingerprint_function: fingerprint_function_def.clone(),
                processing_delay,
                reprocessing_period,
                sources,
                stability_delay,
                function: TrendMaterializationFunction {
                    return_type,
                    src: function_def,
                    language: function_lang,
                },
                description: description.clone(),
            };

            let trend_materialization = TrendMaterialization::Function(function_materialization);

            trend_materializations.push(trend_materialization);
        }
    }

    Ok(trend_materializations)
}

async fn load_sources<T: GenericClient + Send + Sync>(
    conn: &mut T,
    materialization_id: i32,
) -> Result<Vec<TrendMaterializationSource>, Error> {
    let mut sources: Vec<TrendMaterializationSource> = Vec::new();

    let query = concat!(
        "SELECT tsp.name, mtsl.timestamp_mapping_func::regproc::text ",
        "FROM trend_directory.materialization_trend_store_link mtsl ",
        "JOIN trend_directory.trend_store_part tsp ON tsp.id = mtsl.trend_store_part_id ",
        "WHERE mtsl.materialization_id = $1"
    );

    let result = conn
        .query(query, &[&materialization_id])
        .await
        .map_err(|e| {
            DatabaseError::from_msg(format!("Error loading trend materializations: {e}"))
        })?;

    for row in result {
        let trend_store_part: String = row.get(0);
        let mapping_function: String = row.get(1);

        sources.push(TrendMaterializationSource {
            trend_store_part,
            mapping_function,
        });
    }

    Ok(sources)
}

// Load the body of a function by specifying it's full name
pub async fn get_view_def<T: GenericClient + Send + Sync>(
    client: &mut T,
    view: &str,
) -> Option<String> {
    let query = format!(concat!("SELECT pg_get_viewdef('{}'::regclass::oid);"), view);

    match client.query_one(query.as_str(), &[]).await {
        Ok(row) => row.get(0),
        Err(_) => None,
    }
}

pub async fn get_function_def<T: GenericClient + Send + Sync>(
    client: &mut T,
    function: &str,
) -> Option<(String, String)> {
    let query = concat!(
        "SELECT lanname, prosrc ",
        "FROM pg_proc ",
        "JOIN pg_language ON pg_language.oid = prolang ",
        "WHERE proname = $1"
    );

    match client.query_one(query, &[&function]).await {
        Ok(row) => Some((row.get(0), row.get(1))),
        Err(_) => None,
    }
}

pub struct ResultColumn {
    pub name: String,
    pub data_type: String,
}

pub async fn get_view_result_columns<T: GenericClient + Send + Sync>(
    client: &mut T,
    view_schema: &str,
    view_name: &str,
) -> Result<Vec<ResultColumn>, String> {
    let query = concat!(
        "SELECT attname, format_type(atttypid, null) ",
        "FROM pg_class ",
        "JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid ",
        "JOIN pg_attribute ON pg_attribute.attrelid = pg_class.oid ",
        "WHERE relkind = 'v' AND nspname = $1 AND relname = $2 AND attnum >= 0"
    );

    let columns: Vec<ResultColumn> = client
        .query(query, &[&view_schema, &view_name])
        .await
        .map(|rows| {
            rows.iter()
                .map(|row| ResultColumn {
                    name: row.get(0),
                    data_type: row.get(1),
                })
                .collect()
        })
        .map_err(|e| format!("could not retrieve result columns for view: {e}"))?;

    Ok(columns)
}

pub async fn get_function_result_columns<T: GenericClient + Send + Sync>(
    client: &mut T,
    function_schema: &str,
    function_name: &str,
) -> Result<Vec<ResultColumn>, String> {
    let query = concat!(
        "SELECT unnest(proargnames[2:]), format_type(unnest(proallargtypes[2:]), null) ",
        "FROM pg_proc ",
        "JOIN pg_namespace ON pg_proc.pronamespace = pg_namespace.oid ",
        "WHERE nspname = $1 AND proname = $2"
    );

    let columns: Vec<ResultColumn> = client
        .query(query, &[&function_schema, &function_name])
        .await
        .map(|rows| {
            rows.iter()
                .map(|row| ResultColumn {
                    name: row.get(0),
                    data_type: row.get(1),
                })
                .collect()
        })
        .map_err(|e| format!("could not retrieve result columns for function: {e}"))?;

    Ok(columns)
}

pub async fn get_function_return_type<T: GenericClient + Send + Sync>(
    client: &mut T,
    function_schema: &str,
    function_name: &str,
) -> Option<String> {
    let columns: Vec<ResultColumn> =
        get_function_result_columns(client, function_schema, function_name)
            .await
            .unwrap();

    let columns_part = columns
        .iter()
        .map(|column| format!("    \"{}\" {}", column.name, column.data_type))
        .collect::<Vec<String>>()
        .join(",\n");

    Some(format!("TABLE (\n{}\n)\n", columns_part))
}

pub struct AddTrendMaterialization {
    pub trend_materialization: TrendMaterialization,
}

impl fmt::Display for AddTrendMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddTrendMaterialization({})",
            &self.trend_materialization
        )
    }
}

#[async_trait]
impl Change for AddTrendMaterialization {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        match self.trend_materialization.create(client).await {
            Ok(_) => Ok(format!(
                "Added trend materialization '{}'",
                &self.trend_materialization
            )),
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "Error adding trend materialization '{}': {}",
                    &self.trend_materialization, e
                ),
            })),
        }
    }
}

impl From<TrendMaterialization> for AddTrendMaterialization {
    fn from(trend_materialization: TrendMaterialization) -> Self {
        AddTrendMaterialization {
            trend_materialization,
        }
    }
}

pub struct RemoveTrendMaterialization {
    pub materialization: TrendMaterialization,
}

impl fmt::Display for RemoveTrendMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RemoveTrendMaterialization({})",
            &self.materialization.name()
        )
    }
}

#[async_trait]
impl Change for RemoveTrendMaterialization {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        self.materialization
            .delete(client)
            .await
            .map(|_| {
                format!(
                    "Removed trend materialization '{}'",
                    &self.materialization.name(),
                )
            })
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!(
                        "Error removing trend materialization '{}': {}",
                        &self.materialization.name(),
                        e
                    ),
                })
            })
    }
}

pub struct UpdateTrendMaterialization {
    pub trend_materialization: TrendMaterialization,
}

impl fmt::Display for UpdateTrendMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpdateTrendMaterialization({})",
            &self.trend_materialization
        )
    }
}

#[async_trait]
impl Change for UpdateTrendMaterialization {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        match self.trend_materialization.update(client).await {
            Ok(_) => Ok(format!(
                "Updated trend materialization '{}'",
                &self.trend_materialization
            )),
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "Error updating trend materialization '{}': {}",
                    &self.trend_materialization, e
                ),
            })),
        }
    }
}

#[derive(Error, Debug)]
pub enum PopulateSourceFingerprintError {
    #[error("Could not load materialization sources: {0}")]
    SourcesLoading(tokio_postgres::Error),
    #[error("No sources found for materialization")]
    NoSources,
    #[error("Could not update fingerprints: {0}")]
    FingerprintUpdating(tokio_postgres::Error),
}

pub async fn populate_source_fingerprint<T: GenericClient + Send + Sync>(
    client: &mut T,
    materialization: &str,
) -> Result<(), PopulateSourceFingerprintError> {
    let sources_query = concat!(
        "SELECT mtsl.timestamp_mapping_func::regproc::text, tsp.name ",
        "FROM trend_directory.materialization m ",
        "JOIN trend_directory.materialization_trend_store_link mtsl ON mtsl.materialization_id = m.id ",
        "JOIN trend_directory.trend_store_part tsp ON tsp.id = mtsl.trend_store_part_id ",
        "WHERE m::text = $1"
    );

    let sources: Vec<(String, String)> = client
        .query(sources_query, &[&materialization])
        .await
        .map_err(PopulateSourceFingerprintError::SourcesLoading)?
        .iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();

    if sources.is_empty() {
        return Err(PopulateSourceFingerprintError::NoSources);
    }

    let mut ctes: Vec<String> = Vec::new();
    let mut query_parts: Vec<String> = Vec::new();

    for (index, (mapping_func, source_name)) in sources.iter().enumerate() {
        let query = format!(
            "select {}(timestamp) AS timestamp from trend.{} group by timestamp",
            mapping_func,
            escape_identifier(source_name),
        );

        let cte_name = format!("source_{}", index + 1);

        let cte = format!("{} AS ({})", cte_name, query);

        ctes.push(cte);

        if index == 0 {
            query_parts.push("SELECT trend_directory.update_source_fingerprint(m.id, source_1.timestamp) FROM source_1".to_string());
        } else {
            query_parts.push(format!(
                "JOIN {cte_name} ON source_1.timestamp = {cte_name}.timestamp"
            ));
        }
    }

    let query = format!(
        "WITH {} {}, trend_directory.materialization m WHERE m::text = $1",
        ctes.join(","),
        query_parts.join(" ")
    );

    client
        .execute(&query, &[&materialization])
        .await
        .map_err(PopulateSourceFingerprintError::FingerprintUpdating)?;

    Ok(())
}

pub async fn reset_source_fingerprint<T: GenericClient + Send + Sync>(
    client: &mut T,
    materialization: &str,
) -> Result<(), String> {
    let query = format!(
        concat!(
            "UPDATE trend_directory.materialization_state nms ",
            "SET source_fingerprint = (trend.\"{}_fingerprint\"(ms.timestamp)).body ",
            "FROM trend_directory.materialization_state ms ",
            "JOIN trend_directory.materialization m ON ms.materialization_id = m.id ",
            "WHERE m::text = $1 AND nms.materialization_id = m.id AND nms.timestamp = ms.timestamp"
        ),
        &materialization
    );

    client
        .execute(&query, &[&materialization])
        .await
        .map_err(|e| format!("Error loading trend materializations: {e}"))?;

    Ok(())
}

async fn get_trend_store_part_id<T: GenericClient + Send + Sync>(
    client: &mut T,
    name: &str,
) -> Result<i32, Error> {
    let source_query = "SELECT id FROM trend_directory.trend_store_part WHERE name = $1";

    let rows = client.query(source_query, &[&name]).await?;

    if rows.is_empty() {
        return Err(Error::Database(DatabaseError::from_msg(format!(
            "Materialization source '{}' does not exist",
            name
        ))));
    }

    let trend_store_part_id: i32 = rows[0].get(0);

    Ok(trend_store_part_id)
}

async fn connect_materialization_sources<T: GenericClient + Send + Sync>(
    client: &mut T,
    target_trend_store_part_name: &str,
    sources: &[TrendMaterializationSource],
) -> Result<(), Error> {
    let query = concat!(
        "INSERT INTO trend_directory.materialization_trend_store_link(materialization_id, trend_store_part_id, timestamp_mapping_func) ",
        "SELECT m.id, $3, $1::regprocedure ",
        "FROM trend_directory.materialization m JOIN trend_directory.trend_store_part dstp ",
        "ON m.dst_trend_store_part_id = dstp.id ",
        "WHERE dstp.name = $2"
    );

    let statement = client
        .prepare_typed(query, &[Type::TEXT, Type::TEXT, Type::INT4])
        .await?;

    for source in sources {
        let source_trend_store_part_id: i32 =
            get_trend_store_part_id(client, &source.trend_store_part).await?;

        let mapping_function = format!("{}(timestamptz)", &source.mapping_function);

        let insert_count = client
            .execute(
                &statement,
                &[
                    &mapping_function,
                    &target_trend_store_part_name,
                    &source_trend_store_part_id,
                ],
            )
            .await
            .map_err(|e| {
                Error::Database(DatabaseError::from_msg(format!(
                    "Error connecting sources: {e}"
                )))
            })?;

        if insert_count == 0 {
            return Err(Error::Runtime(RuntimeError::from_msg(format!(
                "Unexpectedly no link was created for source '{}'",
                source.trend_store_part
            ))));
        }
    }

    Ok(())
}

pub async fn remove_trend_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    name: &str,
) -> Result<(), String> {
    let query =
        concat!("DELETE FROM trend_directory.materialization WHERE materialization::text = $1");

    let deleted = client.execute(query, &[&name]).await.unwrap();

    if deleted == 1 {
        Ok(())
    } else if deleted == 0 {
        Err("No materializations deleted".to_string())
    } else {
        Err(format!("More than 1 materialization deleted ({})", deleted))
    }
}

pub async fn check_trend_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    trend_materialization: &TrendMaterialization,
) -> Result<Vec<String>, String> {
    match trend_materialization {
        TrendMaterialization::View(ref view_materialization) => {
            check_view_materialization(client, view_materialization).await
        }
        TrendMaterialization::Function(ref function_materialization) => {
            check_function_materialization(client, function_materialization).await
        }
    }
}

pub async fn check_view_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    view_materialization: &TrendViewMaterialization,
) -> Result<Vec<String>, String> {
    let mut report = Vec::new();

    let view_name = format!("_{}", view_materialization.target_trend_store_part);

    let view_result_columns: Vec<ResultColumn> =
        get_view_result_columns(client, MATERIALIZATION_FUNCTION_SCHEMA, &view_name)
            .await
            .unwrap()
            .into_iter()
            .filter(|c| c.name != "entity_id" && c.name != "timestamp")
            .collect();

    let view_result_columns_map: HashMap<String, String> = view_result_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    let trend_store_part_columns =
        get_trend_store_part_columns(client, &view_materialization.target_trend_store_part)
            .await
            .unwrap();

    let trend_store_part_columns_map: HashMap<String, String> = trend_store_part_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    for view_result_column in view_result_columns {
        let trend_data_type = trend_store_part_columns_map.get(&view_result_column.name);

        match trend_data_type {
            None => {
                report.push(format!(
                    "Column '{}'({}) is returned from view but has no matching trend",
                    view_result_column.name, view_result_column.data_type
                ));
            }
            Some(data_type) => {
                if !data_type.eq(&view_result_column.data_type) {
                    report.push(format!(
                        "Column '{}'({}) returned from view differs in type: '{}' != '{}' ",
                        view_result_column.name,
                        view_result_column.data_type,
                        view_result_column.data_type,
                        data_type
                    ));
                }
            }
        }
    }

    for trend_store_part_column in trend_store_part_columns {
        let view_result_column_data_type =
            view_result_columns_map.get(&trend_store_part_column.name);

        if view_result_column_data_type.is_none() {
            report.push(format!(
                "Column '{}'({}) is defined as trend in trend store part '{}' but is not returned from view",
                trend_store_part_column.name, trend_store_part_column.data_type, view_materialization.target_trend_store_part
            ));
        }
    }

    Ok(report)
}

pub async fn check_function_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    function_materialization: &TrendFunctionMaterialization,
) -> Result<Vec<String>, String> {
    let mut report: Vec<String> = Vec::new();
    let function_result_columns: Vec<ResultColumn> = get_function_result_columns(
        client,
        MATERIALIZATION_FUNCTION_SCHEMA,
        &function_materialization.target_trend_store_part,
    )
    .await
    .unwrap()
    .into_iter()
    .filter(|c| c.name != "entity_id" && c.name != "timestamp")
    .collect();

    let function_result_columns_map: HashMap<String, String> = function_result_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    let trend_store_part_columns =
        get_trend_store_part_columns(client, &function_materialization.target_trend_store_part)
            .await
            .unwrap();

    let trend_store_part_columns_map: HashMap<String, String> = trend_store_part_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    for function_result_column in function_result_columns {
        let trend_data_type = trend_store_part_columns_map.get(&function_result_column.name);

        match trend_data_type {
            None => {
                report.push(format!(
                    "Column '{}'({}) is returned from function but has no matching trend",
                    function_result_column.name, function_result_column.data_type
                ));
            }
            Some(data_type) => {
                if !data_type.eq(&function_result_column.data_type) {
                    report.push(format!(
                        "Column '{}'({}) returned from function differs in type: '{}' != '{}' ",
                        function_result_column.name,
                        function_result_column.data_type,
                        function_result_column.data_type,
                        data_type
                    ));
                }
            }
        }
    }

    for trend_store_part_column in trend_store_part_columns {
        let function_result_column_data_type =
            function_result_columns_map.get(&trend_store_part_column.name);

        if function_result_column_data_type.is_none() {
            report.push(format!(
                "Column '{}'({}) is defined as trend in trend store part '{}' but is not returned from function",
                trend_store_part_column.name, trend_store_part_column.data_type, function_materialization.target_trend_store_part
            ));
        }
    }

    Ok(report)
}

pub struct TrendStorePartColumn {
    pub name: String,
    pub data_type: String,
}

pub async fn get_trend_store_part_columns<T: GenericClient + Send + Sync>(
    client: &mut T,
    trend_store_part_name: &str,
) -> Result<Vec<TrendStorePartColumn>, String> {
    let query = concat!(
        "SELECT tt.name, tt.data_type ",
        "FROM trend_directory.trend_store_part tsp ",
        "JOIN trend_directory.table_trend tt ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = $1"
    );

    let columns: Vec<TrendStorePartColumn> = client
        .query(query, &[&trend_store_part_name])
        .await
        .map(|rows| {
            rows.iter()
                .map(|row| TrendStorePartColumn {
                    name: row.get(0),
                    data_type: row.get(1),
                })
                .collect()
        })
        .map_err(|e| format!("could not retrieve columns for trend store part: {e}"))?;

    Ok(columns)
}
