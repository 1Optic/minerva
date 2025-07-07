use futures_util::pin_mut;
use humantime::format_duration;
use log::debug;
use postgres_protocol::escape::escape_identifier;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::fmt;
use std::iter::zip;
use std::path::PathBuf;
use std::time::Duration;
use tokio_postgres::types::ToSql;
use tokio_postgres::{binary_copy::BinaryCopyInWriter, Client, GenericClient, Row, Transaction};

use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

use async_trait::async_trait;

use crate::changes::trend_store::{
    AddAliasColumn, AddTrendStorePart, AddTrends, ModifyTrendDataType, ModifyTrendDataTypes,
    ModifyTrendExtraData, RemoveAliasColumn, RemoveTrends,
};
use crate::entity::EntityMapping;
use crate::meas_value::{
    DataType, MeasValue, INT8_NONE_VALUE, INTEGER_NONE_VALUE, NUMERIC_NONE_VALUE, TEXT_NONE_VALUE,
};

use super::change::Change;
use super::error::{ConfigurationError, DatabaseError, Error, RuntimeError};
use super::interval::parse_interval;

pub mod create;

type PostgresName = String;

#[derive(thiserror::Error, Debug)]
pub enum RawMeasurementStoreError {
    #[error("{0}")]
    NamesToEntityIds(String),
    #[error("{0}")]
    ExtractSubPackage(String),
    #[error("{0}")]
    Storage(String),
    #[error("{0}")]
    MarkModified(String),
}

#[async_trait]
pub trait RawMeasurementStore {
    async fn store_raw<E: EntityMapping + Sync>(
        &self,
        client: &mut Client,
        entity_mapping: &E,
        job_id: i64,
        trends: &[String],
        data_package: &[(String, DateTime<chrono::Utc>, Vec<String>)],
        null_value: String,
    ) -> Result<(), RawMeasurementStoreError>;
}

#[derive(thiserror::Error, Debug)]
pub enum MeasurementStoreError {
    #[error("database issue: {0}")]
    Database(String),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum StorePackageError {
    #[error("database connection issue: {0}")]
    DatabaseConnection(String),
    #[error("unexpected database issue: {0}")]
    Database(String),
    #[error("data type mismatch: {0}")]
    DatatypeMismatch(String),
}

#[async_trait]
pub trait MeasurementStore {
    async fn store(
        &self,
        client: &mut Client,
        job_id: i64,
        trends: &[String],
        data_package: &[ValueRow],
    ) -> Result<(), MeasurementStoreError>;

    async fn store_package<U>(
        &self,
        client: &mut Client,
        data_package: &U,
    ) -> Result<(), StorePackageError>
    where
        U: DataPackage + std::marker::Sync;

    async fn mark_modified<T: GenericClient + std::marker::Sync>(
        &self,
        client: &T,
        timestamp: &DateTime<chrono::Utc>,
    ) -> Result<(), MeasurementStoreError>;
}

pub struct DeleteTrendStoreError {
    original: String,
    kind: DeleteTrendStoreErrorKind,
}

impl DeleteTrendStoreError {
    fn database_error(e: tokio_postgres::Error) -> DeleteTrendStoreError {
        DeleteTrendStoreError {
            original: format!("{e}"),
            kind: DeleteTrendStoreErrorKind::DatabaseError,
        }
    }
}

impl From<tokio_postgres::Error> for DeleteTrendStoreError {
    fn from(e: tokio_postgres::Error) -> DeleteTrendStoreError {
        DeleteTrendStoreError::database_error(e)
    }
}

impl fmt::Display for DeleteTrendStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            DeleteTrendStoreErrorKind::DatabaseError => {
                write!(f, "database error: {}", self.original)
            }
            DeleteTrendStoreErrorKind::NoSuchTrendStore => {
                write!(f, "no such trend: {}", self.original)
            }
        }
    }
}

enum DeleteTrendStoreErrorKind {
    NoSuchTrendStore,
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSql, PartialEq, Eq)]
#[postgres(name = "trend_descr")]
pub struct Trend {
    pub name: PostgresName,
    pub data_type: DataType,
    #[serde(default = "default_empty_string")]
    pub description: String,
    #[serde(default = "default_time_aggregation")]
    pub time_aggregation: String,
    #[serde(default = "default_entity_aggregation")]
    pub entity_aggregation: String,
    #[serde(default = "default_extra_data")]
    pub extra_data: Value,
}

fn default_time_aggregation() -> String {
    String::from("SUM")
}

fn default_entity_aggregation() -> String {
    String::from("SUM")
}

fn default_extra_data() -> Value {
    json!("{}")
}

impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Trend({}, {})", &self.name, &self.data_type)
    }
}

impl Trend {
    #[must_use]
    pub fn sql_type(&self) -> Type {
        match self.data_type {
            DataType::Int2 => Type::INT2,
            DataType::Integer => Type::INT4,
            DataType::Int8 => Type::INT8,
            DataType::Numeric => Type::NUMERIC,
            DataType::Real => Type::FLOAT4,
            DataType::Double => Type::FLOAT8,
            DataType::Timestamp => Type::TIMESTAMPTZ,
            _ => Type::TEXT,
        }
    }

    #[must_use]
    pub fn none_value(&self) -> MeasValue {
        match self.data_type {
            DataType::Int2 => MeasValue::Int2(None),
            DataType::Integer => MeasValue::Integer(None),
            DataType::Numeric => MeasValue::Numeric(None),
            DataType::Int8 => MeasValue::Int8(None),
            DataType::Real => MeasValue::Real(None),
            DataType::Double => MeasValue::Double(None),
            DataType::Timestamp => MeasValue::Timestamp(DateTime::default()),
            _ => MeasValue::Text(String::new()),
        }
    }

    pub fn meas_value_from_str(&self, value: &str, null_value: &str) -> Result<MeasValue, Error> {
        match self.data_type {
            DataType::Integer => {
                if value == null_value {
                    Ok(MeasValue::Integer(None))
                } else {
                    Ok(MeasValue::Integer(Some(i32::from_str(value).map_err(
                        |e| {
                            Error::Runtime(RuntimeError {
                                msg: format!(
                                    "Could not parse integer measurement value '{value}': {e}"
                                ),
                            })
                        },
                    )?)))
                }
            }
            DataType::Numeric => {
                if value == null_value {
                    Ok(MeasValue::Numeric(None))
                } else {
                    Ok(MeasValue::Numeric(Some(Decimal::from_str(value).map_err(
                        |e| {
                            Error::Runtime(RuntimeError {
                                msg: format!(
                                    "Could not parse numeric measurement value '{value}': {e}"
                                ),
                            })
                        },
                    )?)))
                }
            }
            DataType::Int8 => {
                if value == null_value {
                    Ok(MeasValue::Int8(None))
                } else {
                    Ok(MeasValue::Int8(Some(i64::from_str(value).map_err(
                        |e| {
                            Error::Runtime(RuntimeError {
                                msg: format!(
                                    "Could not parse bigint measurement value '{value}': {e}"
                                ),
                            })
                        },
                    )?)))
                }
            }
            DataType::Real => {
                if value == null_value {
                    Ok(MeasValue::Real(None))
                } else {
                    Ok(MeasValue::Real(Some(f32::from_str(value).map_err(|e| Error::Runtime(RuntimeError { msg: format!("Could not parse floating point measurement value '{value}': {e}") }))?)))
                }
            }
            DataType::Double => {
                if value == null_value {
                    Ok(MeasValue::Double(None))
                } else {
                    Ok(MeasValue::Double(Some(f64::from_str(value).map_err(|e| Error::Runtime(RuntimeError { msg: format!("Could not parse floating point measurement value '{value}': {e}") }))?)))
                }
            }
            _ => Ok(MeasValue::Text(String::new())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSql, PartialEq, Eq)]
#[postgres(name = "generated_trend_descr")]
pub struct GeneratedTrend {
    pub name: PostgresName,
    pub data_type: String,

    #[serde(default = "default_empty_string")]
    pub description: String,
    pub expression: String,

    #[serde(default = "default_extra_data")]
    pub extra_data: Value,
}

fn default_empty_string() -> String {
    String::new()
}

#[derive(Default, Debug)]
pub struct TrendStorePartDiffOptions {
    pub ignore_trend_extra_data: bool,
    pub ignore_trend_data_type: bool,
    pub ignore_deletions: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSql, PartialEq, Eq)]
#[postgres(name = "trend_store_part_descr")]
pub struct TrendStorePart {
    pub name: PostgresName,
    #[serde(default = "default_boolean")]
    pub has_alias_column: bool,
    pub trends: Vec<Trend>,
    #[serde(default = "default_generated_trends")]
    pub generated_trends: Vec<GeneratedTrend>,
}

impl PartialOrd for TrendStorePart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrendStorePart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

fn default_generated_trends() -> Vec<GeneratedTrend> {
    Vec::new()
}

fn default_boolean() -> bool {
    false
}

fn insert_query(trend_store_part: &TrendStorePart, trends: &[&Trend]) -> String {
    let update_part = trends
        .iter()
        .map(|trend| {
            format!(
                "{} = excluded.{}",
                escape_identifier(&trend.name),
                escape_identifier(&trend.name)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let trend_names_part = trends
        .iter()
        .map(|t| escape_identifier(&t.name))
        .collect::<Vec<_>>()
        .join(", ");

    let values_placeholders = (1..(trends.len() + 5))
        .map(|i| format!("${i}"))
        .collect::<Vec<_>>()
        .join(", ");

    let alias_part = match trend_store_part.has_alias_column {
        true => "name, ",
        false => "",
    };

    let insert_query = format!(
        "INSERT INTO trend.{}(entity_id, timestamp, created, job_id, {}{}) VALUES ({}) ON CONFLICT (entity_id, timestamp) DO UPDATE SET {}",
        escape_identifier(&trend_store_part.name),
        alias_part,
        &trend_names_part,
        &values_placeholders,
        update_part,
    );

    insert_query
}

fn copy_from_query(trend_store_part: &TrendStorePart, trends: &[&Trend]) -> String {
    let trend_names_part = trends
        .iter()
        .map(|t| escape_identifier(&t.name))
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "COPY trend.{}(entity_id, timestamp, created, job_id, {}{}) FROM STDIN BINARY",
        escape_identifier(&trend_store_part.name),
        match trend_store_part.has_alias_column {
            true => "name, ",
            false => "",
        },
        &trend_names_part
    );

    query
}

struct ValueExtractor<'a> {
    pub trend: &'a Trend,
    pub value_index: usize,
}

impl ValueExtractor<'_> {
    fn extract(&self, values: &[String], null_value: &str) -> Result<MeasValue, Error> {
        values
            .get(self.value_index)
            .map(|v| self.trend.meas_value_from_str(v, null_value))
            .ok_or(Error::Runtime(RuntimeError::from(format!(
                "Could not find value at index {}",
                self.value_index
            ))))?
    }
}

struct SubPackageExtractor<'a> {
    pub trend_store_part: &'a TrendStorePart,
    pub null_value: String,
    pub value_extractors: Vec<ValueExtractor<'a>>,
}

impl<'a> SubPackageExtractor<'a> {
    fn new(trend_store_part: &'a TrendStorePart, null_value: String) -> SubPackageExtractor<'a> {
        SubPackageExtractor {
            trend_store_part,
            null_value,
            value_extractors: Vec::new(),
        }
    }

    fn trend_names(&self) -> Vec<String> {
        self.value_extractors
            .iter()
            .map(|e| e.trend.name.clone())
            .collect()
    }

    fn extract_sub_package<'b>(
        &self,
        entity_ids: &Vec<i32>,
        data_package: &'b [(String, DateTime<Utc>, Vec<String>)],
        aliases: &Vec<Option<String>>,
    ) -> Result<(Vec<ValueRow>, Vec<&'b DateTime<Utc>>), Error> {
        let mut sub_package = Vec::new();
        let mut timestamps: HashSet<&DateTime<Utc>> = HashSet::new();

        for ((entity_id, (_entity, timestamp, values)), alias) in
            zip(zip(entity_ids, data_package), aliases)
        {
            let meas_values: Result<Vec<MeasValue>, Error> = self
                .value_extractors
                .iter()
                .map(|value_extractor| value_extractor.extract(values, &self.null_value))
                .collect();

            timestamps.insert(timestamp);

            sub_package.push(ValueRow {
                entity_id: *entity_id,
                timestamp: *timestamp,
                values: meas_values?,
                alias: alias.as_ref().map(|v| v.to_string()),
            });
        }

        Ok((sub_package, timestamps.into_iter().collect()))
    }
}

#[async_trait]
impl RawMeasurementStore for TrendStore {
    async fn store_raw<E: EntityMapping + Sync>(
        &self,
        client: &mut Client,
        entity_mapping: &E,
        job_id: i64,
        trend_names: &[String],
        records: &[(String, DateTime<chrono::Utc>, Vec<String>)],
        null_value: String,
    ) -> Result<(), RawMeasurementStoreError> {
        let alias_column = entity_mapping
            .uses_alias_column(&self.entity_type, client)
            .await
            .map_err(|e| RawMeasurementStoreError::NamesToEntityIds(e.to_string()))?;
        let entity_ids: Vec<i32> = entity_mapping
            .names_to_entity_ids(
                client,
                &self.entity_type,
                &records
                    .iter()
                    .map(|(entity_name, _timestamp, _values)| entity_name.clone())
                    .collect::<Vec<String>>(),
            )
            .await
            .map_err(|e| RawMeasurementStoreError::NamesToEntityIds(e.to_string()))?;

        let aliases: Vec<Option<String>> = if alias_column {
            entity_mapping
                .names_to_aliases(
                    client,
                    &self.entity_type,
                    &records
                        .iter()
                        .map(|(entity_name, _timestamp, _values)| entity_name.clone())
                        .collect::<Vec<String>>(),
                )
                .await
                .map_err(|e| Error::Runtime(RuntimeError::from_msg(e.to_string())))
                .map_err(|e| RawMeasurementStoreError::NamesToEntityIds(e.to_string()))?
        } else {
            entity_ids.iter().map(|_| None).collect()
        };

        let mut extractors: HashMap<&str, SubPackageExtractor> = HashMap::new();

        for (value_index, trend_name) in trend_names.iter().enumerate() {
            for trend_store_part in &self.parts {
                for trend in &trend_store_part.trends {
                    if trend.name == *trend_name {
                        let extractor =
                            extractors.entry(&trend_store_part.name).or_insert_with(|| {
                                SubPackageExtractor::new(trend_store_part, null_value.clone())
                            });

                        extractor
                            .value_extractors
                            .push(ValueExtractor { trend, value_index });
                    }
                }
            }
        }

        for extractor in extractors.values() {
            let (sub_data_package, timestamps) = extractor
                .extract_sub_package(&entity_ids, records, &aliases)
                .map_err(|e| RawMeasurementStoreError::ExtractSubPackage(e.to_string()))?;

            extractor
                .trend_store_part
                .store(client, job_id, &extractor.trend_names(), &sub_data_package)
                .await
                .map_err(|e| {
                    RawMeasurementStoreError::Storage(format!("Error storing data package: {e}"))
                })?;

            for timestamp in timestamps {
                extractor
                    .trend_store_part
                    .mark_modified(client, timestamp)
                    .await
                    .map_err(|e| RawMeasurementStoreError::MarkModified(e.to_string()))?;
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum StoreCopyFromError {
    #[error("{0}")]
    DataMismatch(String),
    #[error("{0}")]
    UniqueViolation(String),
    #[error("{0}")]
    Write(String),
    #[error("{0}")]
    Generic(String),
}

#[async_trait]
impl MeasurementStore for TrendStorePart {
    async fn store(
        &self,
        client: &mut Client,
        job_id: i64,
        trends: &[String],
        data_package: &[ValueRow],
    ) -> Result<(), MeasurementStoreError> {
        if trends.is_empty() {
            return Ok(());
        };

        match self
            .store_copy_from(client, job_id, trends, data_package)
            .await
        {
            Ok(_) => Ok(()),
            Err(TrendStorePartStorageError::UniqueViolation(_)) => {
                self.store_insert(client, job_id, trends, data_package)
                    .await
                    .map_err(|e| MeasurementStoreError::Database(format!("{e}")))?;

                Ok(())
            }
            Err(TrendStorePartStorageError::Database(m)) => Err(MeasurementStoreError::Database(m)),
        }
    }

    async fn store_package<U>(
        &self,
        client: &mut Client,
        data_package: &U,
    ) -> Result<(), StorePackageError>
    where
        U: DataPackage + std::marker::Sync,
    {
        debug!("Copying package");
        match self.store_copy_from_package(client, data_package).await {
            Ok(()) => Ok(()),
            Err(e) => match e {
                StoreCopyFromError::UniqueViolation(_) => self
                    .store_insert_package(client, data_package)
                    .await
                    .map_err(|e| StorePackageError::Database(e.to_string())),
                StoreCopyFromError::DataMismatch(e) => {
                    let data_type_mismatches = self
                        .verify_data_types(client, &data_package.trends())
                        .await
                        .unwrap();

                    if !data_type_mismatches.is_empty() {
                        Err(StorePackageError::DatatypeMismatch(format!(
                            "Mismatching data types: [{}]",
                            data_type_mismatches.join(", ")
                        )))
                    } else {
                        Err(StorePackageError::Database(e.to_string()))
                    }
                }
                _ => Err(StorePackageError::Database(e.to_string())),
            },
        }?;

        debug!("Package copied!");

        self.mark_modified(client, data_package.timestamp())
            .await
            .map_err(|e| {
                StorePackageError::Database(format!(
                    "Could not mark timestamp modified '{}' - '{}': {e}",
                    self.name,
                    data_package.timestamp(),
                ))
            })
    }

    async fn mark_modified<T: GenericClient + std::marker::Sync>(
        &self,
        client: &T,
        timestamp: &DateTime<chrono::Utc>,
    ) -> Result<(), MeasurementStoreError> {
        let query = concat!(
            "SELECT trend_directory.mark_modified(id, $2) ",
            "FROM trend_directory.trend_store_part ",
            "WHERE name = $1"
        );

        client
            .execute(query, &[&self.name, &timestamp])
            .await
            .map_err(|e| {
                MeasurementStoreError::Database(format!("Error marking timestamp as modified: {e}"))
            })?;

        Ok(())
    }
}

struct ValueMapper<'a> {
    index: Option<usize>,
    data_type: &'a DataType,
}

impl<'a> ValueMapper<'a> {
    fn map_value_from(&self, values: &'a [MeasValue]) -> &'a (dyn ToSql + Sync) {
        match self.index {
            Some(index) => {
                match values.get(index) {
                    Some(v) => v,
                    None => {
                        // This should not be possible
                        match self.data_type {
                            DataType::Integer => &(*INTEGER_NONE_VALUE),
                            DataType::Int8 => &(*INT8_NONE_VALUE),
                            DataType::Numeric => &(*NUMERIC_NONE_VALUE),
                            _ => &(*TEXT_NONE_VALUE),
                        }
                    }
                }
            }
            None => match self.data_type {
                DataType::Integer => &(*INTEGER_NONE_VALUE),
                DataType::Int8 => &(*INT8_NONE_VALUE),
                DataType::Numeric => &(*NUMERIC_NONE_VALUE),
                _ => &(*TEXT_NONE_VALUE),
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DataPackageWriteError {
    #[error("Could not write package: {0}")]
    Generic(String),
    #[error("Could not prepare data: {0}")]
    DataPreparation(String),
    #[error("Data type mismatch: {0}")]
    DatatypeMismatch(String),
}

#[async_trait]
pub trait DataPackage {
    fn timestamp(&self) -> &DateTime<Utc>;
    fn trends(&self) -> Vec<String>;

    async fn write(
        &self,
        writer: std::pin::Pin<&mut BinaryCopyInWriter>,
        values: &[(usize, DataType)],
        created_timestamp: &DateTime<chrono::Utc>,
    ) -> Result<usize, DataPackageWriteError>;

    async fn insert<C: GenericClient + std::marker::Sync + std::marker::Send>(
        &self,
        client: &mut C,
        query: &str,
        values: &[(usize, DataType)],
        created_timestamp: &DateTime<chrono::Utc>,
    ) -> Result<usize, Error>;
}

pub struct ValueRow {
    pub entity_id: i32,
    pub alias: Option<String>,
    pub timestamp: DateTime<chrono::Utc>,
    pub values: Vec<MeasValue>,
}

#[derive(thiserror::Error, Debug)]
pub enum TrendStorePartStorageError {
    #[error("database issue: {0}")]
    Database(String),
    #[error("Unique violation: {0}")]
    UniqueViolation(String),
}

pub struct Column {
    pub name: String,
    pub data_type: String,
}

impl TrendStorePart {
    pub fn base_columns(&self) -> Vec<Column> {
        let mut columns = vec![
            Column {
                name: "entity_id".to_string(),
                data_type: "integer".to_string(),
            },
            Column {
                name: "timestamp".to_string(),
                data_type: "timestamp with time zone".to_string(),
            },
        ];

        if self.has_alias_column {
            columns.push(Column {
                name: "name".to_string(),
                data_type: "text".to_string(),
            });
        }

        columns.extend([
            Column {
                name: "created".to_string(),
                data_type: "timestamp with time zone".to_string(),
            },
            Column {
                name: "job_id".to_string(),
                data_type: "bigint".to_string(),
            },
        ]);

        columns
    }

    pub async fn store_copy_from<'a, I>(
        &self,
        client: &mut Client,
        job_id: i64,
        trends: &[String],
        data_rows: I,
    ) -> Result<u64, TrendStorePartStorageError>
    where
        I: IntoIterator<Item = &'a ValueRow>,
    {
        // List of indexes of matched trends to extract corresponding values
        let mut matched_trend_indexes: Vec<Option<usize>> = Vec::new();

        let mut matched_trends: Vec<&Trend> = Vec::new();

        let mut value_types: Vec<Type> =
            vec![Type::INT4, Type::TIMESTAMPTZ, Type::TIMESTAMPTZ, Type::INT8];

        // Filter trends that match the trend store parts trends and add corresponding types
        for t in &self.trends {
            let index = trends.iter().position(|trend_name| trend_name == &t.name);

            if index.is_some() {
                value_types.push(t.sql_type());
                matched_trends.push(t);

                matched_trend_indexes.push(index);
            }
        }

        let index_trend_map: Vec<ValueMapper> = matched_trend_indexes
            .iter()
            .zip(matched_trends.iter())
            .map(|(index, trend)| ValueMapper {
                index: *index,
                data_type: &trend.data_type,
            })
            .collect();

        if matched_trends.is_empty() {
            return Ok(0);
        }

        let query = copy_from_query(self, &matched_trends);

        let copy_in_sink = client.copy_in(&query).await.map_err(|e| {
            TrendStorePartStorageError::Database(format!("Error starting COPY command: {e}"))
        })?;

        let binary_copy_writer = BinaryCopyInWriter::new(copy_in_sink, &value_types);
        pin_mut!(binary_copy_writer);

        // We cannot use the database now() function for COPY FROM queries, so the 'created'
        // timestamp for the trend data records is generated here.
        let created_timestamp = Utc::now();

        for value_row in data_rows {
            let mut values: Vec<&(dyn ToSql + Sync)> = vec![
                &value_row.entity_id,
                &value_row.timestamp,
                &created_timestamp,
                &job_id,
            ];

            if let Some(alias) = &value_row.alias {
                values.push(alias)
            };

            values.extend(
                index_trend_map
                    .iter()
                    .map(|value_mapper| value_mapper.map_value_from(&value_row.values)),
            );

            binary_copy_writer
                .as_mut()
                .write(&values)
                .await
                .map_err(|e| {
                    TrendStorePartStorageError::Database(format!("Error writing row: {e}"))
                })?;
        }

        let row_count = binary_copy_writer.finish().await.map_err(|e| {
            // For some reason, the error code returned by e.code() is XX000, or INTERNAL_ERROR.
            // The string representation of the error does contain the 'duplicate key' violation
            // indication.
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                TrendStorePartStorageError::UniqueViolation(format!(
                    "Could not load data using COPY command: {e}"
                ))
            } else {
                TrendStorePartStorageError::Database(format!(
                    "Could not load data using COPY command: {e}"
                ))
            }
        })?;

        Ok(row_count)
    }

    async fn verify_data_types(
        &self,
        client: &mut Client,
        trends: &[String],
    ) -> Result<Vec<String>, StoreCopyFromError> {
        let mut mismatches: Vec<String> = Vec::new();

        let trend_store_part = load_trend_store_part(client, &self.name).await;

        for trend_name in trends {
            let own_trend_result = self.trends.iter().find(|t| t.name.eq(trend_name)).unwrap();

            let loaded_trend_result = trend_store_part
                .trends
                .iter()
                .find(|t| t.name.eq(trend_name));

            if let Some(trend) = loaded_trend_result {
                if trend.data_type != own_trend_result.data_type {
                    mismatches.push(format!(
                        "{}({}<>{})",
                        trend_name, own_trend_result.data_type, trend.data_type
                    ));
                }
            }
        }

        Ok(mismatches)
    }

    async fn store_copy_from_package<U>(
        &self,
        client: &mut Client,
        data_package: &U,
    ) -> Result<(), StoreCopyFromError>
    where
        U: DataPackage,
    {
        // List of indexes of matched trends to extract corresponding values
        let mut matched_trend_indexes: Vec<usize> = Vec::new();

        let mut matched_trends: Vec<&Trend> = Vec::new();

        let mut value_types: Vec<Type> =
            vec![Type::INT4, Type::TIMESTAMPTZ, Type::TIMESTAMPTZ, Type::INT8];

        if self.has_alias_column {
            value_types.push(Type::TEXT);
        }

        // Filter trends that match the trend store parts trends and add corresponding types
        for t in &self.trends {
            let index = data_package
                .trends()
                .iter()
                .position(|trend_name| trend_name == &t.name);

            if let Some(i) = index {
                value_types.push(t.sql_type());
                matched_trends.push(t);

                matched_trend_indexes.push(i);
            }
        }

        let index_trend_map: Vec<(usize, DataType)> = matched_trend_indexes
            .iter()
            .zip(matched_trends.iter())
            .map(|(index, trend)| (*index, trend.data_type))
            .collect();

        if matched_trends.is_empty() {
            return Ok(());
        }

        let query = copy_from_query(self, &matched_trends);

        let copy_in_sink = client.copy_in(&query).await.map_err(|e| {
            StoreCopyFromError::Generic(format!("Error starting COPY command: {e}"))
        })?;

        let binary_copy_writer = BinaryCopyInWriter::new(copy_in_sink, &value_types);
        pin_mut!(binary_copy_writer);

        // We cannot use the database now() function for COPY FROM queries, so the 'created'
        // timestamp for the trend data records is generated here.
        let created_timestamp = Utc::now();

        data_package
            .write(
                binary_copy_writer.as_mut(),
                &index_trend_map,
                &created_timestamp,
            )
            .await
            .map_err(|e| {
                StoreCopyFromError::Write(format!("Could not write package for COPY command: {e}"))
            })?;

        binary_copy_writer.finish().await.map_err(|e| {
            let error_text = e.to_string();

            // For some reason, the error code returned by e.code() is XX000, or INTERNAL_ERROR.
            // The string representation of the error does contain the 'duplicate key' violation
            // indication.
            if error_text.contains("duplicate key value violates unique constraint") {
                StoreCopyFromError::UniqueViolation(error_text)
            } else if error_text.contains("insufficient data left in message")
                || error_text.contains("incorrect binary data format")
            {
                StoreCopyFromError::DataMismatch(error_text)
            } else {
                StoreCopyFromError::Generic(error_text)
            }
        })?;

        Ok(())
    }

    async fn store_insert<T: GenericClient>(
        &self,
        tx: &mut T,
        job_id: i64,
        trends: &[String],
        data_package: &[ValueRow],
    ) -> Result<(), Error> {
        // List of indexes of matched trends to extract corresponding values
        let mut matched_trend_indexes: Vec<Option<usize>> = Vec::new();
        let mut matched_trends: Vec<&Trend> = Vec::new();

        // Filter trends that match the trend store parts trends
        for t in &self.trends {
            let index = trends.iter().position(|trend_name| trend_name == &t.name);

            if index.is_some() {
                matched_trend_indexes.push(index);
                matched_trends.push(t);
            }
        }

        let created_timestamp = Utc::now();

        let query = insert_query(self, &matched_trends);

        for value_row in data_package {
            let mut values: Vec<&(dyn ToSql + Sync)> = vec![
                &value_row.entity_id,
                &value_row.timestamp,
                &created_timestamp,
                &job_id,
            ];

            if let Some(alias) = &value_row.alias {
                values.push(alias)
            };

            for (i, t) in matched_trend_indexes.iter().zip(matched_trends.iter()) {
                match i {
                    Some(index) => {
                        match value_row.values.get(*index) {
                            Some(v) => values.push(v),
                            None => {
                                // This should not be possible
                                return Err(Error::Runtime(RuntimeError::from_msg(format!(
                                    "Expected value not found at index {index}"
                                ))));
                            }
                        };
                    }
                    None => {
                        match t.data_type {
                            DataType::Integer => values.push(&(*INTEGER_NONE_VALUE)),
                            DataType::Int8 => values.push(&(*INT8_NONE_VALUE)),
                            DataType::Numeric => values.push(&(*NUMERIC_NONE_VALUE)),
                            _ => values.push(&(*TEXT_NONE_VALUE)),
                        };
                    }
                }
            }

            tx.execute(&query, &values).await?;
        }

        Ok(())
    }

    async fn store_insert_package<U, T>(
        &self,
        client: &mut T,
        data_package: &U,
    ) -> Result<(), Error>
    where
        U: DataPackage,
        T: GenericClient,
    {
        // List of indexes of matched trends to extract corresponding values
        let mut matched_trend_indexes: Vec<Option<usize>> = Vec::new();
        let mut matched_trends: Vec<&Trend> = Vec::new();
        let mut values: Vec<(usize, DataType)> = Vec::new();

        // Filter trends that match the trend store parts trends
        for t in &self.trends {
            let index = data_package
                .trends()
                .iter()
                .position(|trend_name| trend_name == &t.name);

            if let Some(i) = index {
                matched_trend_indexes.push(index);
                matched_trends.push(t);
                values.push((i, t.data_type));
            }
        }

        let name_trend = Trend {
            name: "name".to_string(),
            data_type: DataType::Text,
            description: "".to_string(),
            time_aggregation: default_time_aggregation(),
            entity_aggregation: default_entity_aggregation(),
            extra_data: default_extra_data(),
        };

        let index = data_package
            .trends()
            .iter()
            .position(|trend_name| trend_name == "name");

        if let Some(i) = index {
            matched_trend_indexes.push(index);
            matched_trends.push(&name_trend);
            values.push((i, DataType::Text));
        }

        let created_timestamp = Utc::now();

        let mut tx = client.transaction().await?;

        let query = insert_query(self, &matched_trends);

        data_package
            .insert(&mut tx, &query, &values, &created_timestamp)
            .await?;

        tx.commit().await.map_err(|e| {
            Error::Database(DatabaseError::from_msg(format!(
                "Could commit data load using COPY command: {e}"
            )))
        })?;

        Ok(())
    }

    #[must_use]
    pub fn diff(
        &self,
        other: &TrendStorePart,
        options: TrendStorePartDiffOptions,
    ) -> Vec<Box<dyn Change + Send>> {
        let mut changes: Vec<Box<dyn Change + Send>> = Vec::new();

        let mut new_trends: Vec<Trend> = Vec::new();
        let mut removed_trends: Vec<String> = Vec::new();
        let mut alter_trend_data_types: Vec<ModifyTrendDataType> = Vec::new();

        for other_trend in &other.trends {
            match self
                .trends
                .iter()
                .find(|my_trend| my_trend.name == other_trend.name)
            {
                Some(my_trend) => {
                    // The trend already exists, check for changes
                    if !options.ignore_trend_data_type
                        && my_trend.data_type != other_trend.data_type
                    {
                        alter_trend_data_types.push(ModifyTrendDataType {
                            trend_name: my_trend.name.clone(),
                            from_type: my_trend.data_type,
                            to_type: other_trend.data_type,
                        });
                    }

                    if !options.ignore_trend_extra_data
                        && my_trend.extra_data != other_trend.extra_data
                    {
                        changes.push(Box::new(ModifyTrendExtraData {
                            trend_store_part_name: self.name.clone(),
                            trend_name: my_trend.name.clone(),
                            from_extra_data: my_trend.extra_data.clone(),
                            to_extra_data: other_trend.extra_data.clone(),
                        }));
                    }
                }
                None => {
                    new_trends.push(other_trend.clone());
                }
            }
        }

        if !new_trends.is_empty() {
            changes.push(Box::new(AddTrends {
                trend_store_part: self.clone(),
                trends: new_trends,
            }));
        }

        for my_trend in &self.trends {
            match other
                .trends
                .iter()
                .find(|other_trend| other_trend.name == my_trend.name)
            {
                Some(_) => {
                    // Ok, the trend still exists
                }
                None => {
                    removed_trends.push(my_trend.name.clone());
                }
            }
        }

        if !self.has_alias_column && other.has_alias_column {
            changes.push(Box::new(AddAliasColumn {
                trend_store_part: self.clone(),
            }))
        }

        if self.has_alias_column && !other.has_alias_column && !options.ignore_deletions {
            changes.push(Box::new(RemoveAliasColumn {
                trend_store_part: self.clone(),
            }))
        }

        if !options.ignore_deletions && !removed_trends.is_empty() {
            changes.push(Box::new(RemoveTrends {
                trend_store_part: self.clone(),
                trends: removed_trends,
            }));
        }

        if !alter_trend_data_types.is_empty() {
            changes.push(Box::new(ModifyTrendDataTypes {
                trend_store_part_name: self.name.clone(),
                modifications: alter_trend_data_types,
            }));
        }

        changes
    }
}

impl fmt::Display for TrendStorePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrendStorePart({})", &self.name)
    }
}

#[derive(Default)]
pub struct TrendStoreDiffOptions {
    pub ignore_trend_extra_data: bool,
    pub ignore_trend_data_type: bool,
    pub ignore_deletions: bool,
}

impl TrendStoreDiffOptions {
    #[must_use]
    pub fn part_diff_options(&self) -> TrendStorePartDiffOptions {
        TrendStorePartDiffOptions {
            ignore_trend_extra_data: self.ignore_trend_extra_data,
            ignore_trend_data_type: self.ignore_trend_data_type,
            ignore_deletions: self.ignore_deletions,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrendStore {
    pub title: Option<String>,
    pub data_source: String,
    pub entity_type: String,
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    #[serde(with = "humantime_serde")]
    pub partition_size: Duration,
    #[serde(with = "humantime_serde", default = "default_retention_period")]
    pub retention_period: Duration,
    pub parts: Vec<TrendStorePart>,
}

fn default_retention_period() -> Duration {
    Duration::from_secs(86400 * 30)
}

impl TrendStore {
    #[must_use]
    pub fn diff(
        &self,
        other: &TrendStore,
        options: TrendStoreDiffOptions,
    ) -> Vec<Box<dyn Change + Send>> {
        let mut changes: Vec<Box<dyn Change + Send>> = Vec::new();

        for other_part in &other.parts {
            match self
                .parts
                .iter()
                .find(|my_part| my_part.name == other_part.name)
            {
                Some(my_part) => {
                    changes.append(&mut my_part.diff(other_part, options.part_diff_options()));
                }
                None => {
                    changes.push(Box::new(AddTrendStorePart {
                        trend_store: self.clone(),
                        trend_store_part: other_part.clone(),
                    }));
                }
            }
        }

        changes
    }

    pub fn dump(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(|e| {
            Error::Runtime(RuntimeError::from_msg(format!(
                "Could not serialize trend store to yaml: {e}"
            )))
        })
    }
}

impl fmt::Display for TrendStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TrendStore({}, {}, {})",
            &self.data_source,
            &self.entity_type,
            &humantime::format_duration(self.granularity).to_string()
        )
    }
}

pub async fn list_trend_stores(
    conn: &mut Client,
) -> Result<Vec<(i32, String, String, String)>, String> {
    let query = concat!(
        "SELECT ts.id, ds.name, et.name, ts.granularity::text ",
        "FROM trend_directory.trend_store ts ",
        "JOIN directory.data_source ds ON ds.id = ts.data_source_id ",
        "JOIN directory.entity_type et ON et.id = ts.entity_type_id"
    );

    let result = conn.query(query, &[]).await.unwrap();

    let trend_stores = result
        .into_iter()
        .map(|row: Row| {
            (
                row.get::<usize, i32>(0),
                row.get::<usize, String>(1),
                row.get::<usize, String>(2),
                row.get::<usize, String>(3),
            )
        })
        .collect();

    Ok(trend_stores)
}

pub async fn delete_trend_store(conn: &mut Client, id: i32) -> Result<(), DeleteTrendStoreError> {
    let query = "SELECT trend_directory.delete_trend_store($1)";

    let deleted = conn.execute(query, &[&id]).await?;

    if deleted == 0 {
        Err(DeleteTrendStoreError {
            kind: DeleteTrendStoreErrorKind::NoSuchTrendStore,
            original: String::from("No trend store matches"),
        })
    } else {
        Ok(())
    }
}

pub async fn get_trend_store_id<T: GenericClient>(
    conn: &T,
    data_source: &str,
    entity_type: &str,
    granularity: &Duration,
) -> Result<i32, tokio_postgres::Error> {
    let query = concat!(
        "SELECT trend_store.id ",
        "FROM trend_directory.trend_store ",
        "JOIN directory.data_source ON data_source.id = trend_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = trend_store.entity_type_id ",
        "WHERE data_source.name = $1 AND entity_type.name = $2 AND granularity = $3::text::interval"
    );

    let granularity_str: String = format_duration(*granularity).to_string();

    let result = conn
        .query_one(query, &[&data_source, &entity_type, &granularity_str])
        .await?;

    let trend_store_id = result.get::<usize, i32>(0);

    Ok(trend_store_id)
}

pub async fn load_trend_store<T: GenericClient>(
    conn: &T,
    data_source: &str,
    entity_type: &str,
    granularity: &Duration,
) -> Result<TrendStore, Error> {
    let query = concat!(
        "SELECT trend_store.id, partition_size::text, retention_period::text ",
        "FROM trend_directory.trend_store ",
        "JOIN directory.data_source ON data_source.id = trend_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = trend_store.entity_type_id ",
        "WHERE data_source.name = $1 AND entity_type.name = $2 AND granularity = $3::text::interval"
    );

    let granularity_str: String = format_duration(*granularity).to_string();

    let result = conn
        .query_one(query, &[&data_source, &entity_type, &granularity_str])
        .await?;

    let parts = load_trend_store_parts(conn, result.get::<usize, i32>(0)).await;

    let partition_size_str = result.get::<usize, String>(1);
    let partition_size = parse_interval(&partition_size_str).unwrap();
    let retention_period_str = result.get::<usize, String>(2);
    let retention_period = parse_interval(&retention_period_str).unwrap();

    Ok(TrendStore {
        title: None,
        data_source: String::from(data_source),
        entity_type: String::from(entity_type),
        granularity: *granularity,
        partition_size,
        retention_period,
        parts,
    })
}

async fn load_trend_store_part<T: GenericClient>(conn: &T, name: &str) -> TrendStorePart {
    let trend_store_part_query = "SELECT id FROM trend_directory.trend_store_part WHERE name = $1";

    let trend_store_part_result = conn.query(trend_store_part_query, &[&name]).await.unwrap();

    let trend_store_part_row = trend_store_part_result.first().unwrap();

    let trend_store_part_id: i32 = trend_store_part_row.get(0);

    let trend_query = concat!(
        "SELECT name, data_type, description, entity_aggregation, time_aggregation, extra_data ",
        "FROM trend_directory.table_trend ",
        "WHERE trend_store_part_id = $1",
    );

    let trend_result = conn
        .query(trend_query, &[&trend_store_part_id])
        .await
        .unwrap();

    let mut trends = Vec::new();

    for trend_row in trend_result {
        let trend_name: &str = trend_row.get(0);
        let trend_data_type: &str = trend_row.get(1);
        let trend_description: &str = trend_row.get(2);
        let trend_entity_aggregation: &str = trend_row.get(3);
        let trend_time_aggregation: &str = trend_row.get(4);
        let trend_extra_data: Value = trend_row.get(5);

        trends.push(Trend {
            name: String::from(trend_name),
            data_type: DataType::from(trend_data_type),
            description: String::from(trend_description),
            entity_aggregation: String::from(trend_entity_aggregation),
            time_aggregation: String::from(trend_time_aggregation),
            extra_data: trend_extra_data,
        });
    }

    TrendStorePart {
        name: String::from(name),
        trends,
        generated_trends: Vec::new(),
        has_alias_column: false,
    }
}

async fn load_trend_store_parts<T: GenericClient>(
    conn: &T,
    trend_store_id: i32,
) -> Vec<TrendStorePart> {
    let trend_store_part_query =
        "SELECT id, name, primary_alias FROM trend_directory.trend_store_part WHERE trend_store_id = $1";

    let trend_store_part_result = conn
        .query(trend_store_part_query, &[&trend_store_id])
        .await
        .unwrap();

    let mut parts: Vec<TrendStorePart> = Vec::new();

    for trend_store_part_row in trend_store_part_result {
        let trend_store_part_id: i32 = trend_store_part_row.get(0);
        let trend_store_part_name: &str = trend_store_part_row.get(1);
        let has_alias_column: bool = trend_store_part_row.get(2);

        let trend_query = concat!(
            "SELECT name, data_type, description, entity_aggregation, time_aggregation, extra_data ",
            "FROM trend_directory.table_trend ",
            "WHERE trend_store_part_id = $1",
        );

        let trend_result = conn
            .query(trend_query, &[&trend_store_part_id])
            .await
            .unwrap();

        let mut trends = Vec::new();

        for trend_row in trend_result {
            let trend_name: &str = trend_row.get(0);
            let trend_data_type: &str = trend_row.get(1);
            let trend_description: &str = trend_row.get(2);
            let trend_entity_aggregation: &str = trend_row.get(3);
            let trend_time_aggregation: &str = trend_row.get(4);
            let trend_extra_data: Value = trend_row.get(5);

            trends.push(Trend {
                name: String::from(trend_name),
                data_type: DataType::from(trend_data_type),
                description: String::from(trend_description),
                entity_aggregation: String::from(trend_entity_aggregation),
                time_aggregation: String::from(trend_time_aggregation),
                extra_data: trend_extra_data,
            });
        }

        parts.push(TrendStorePart {
            name: String::from(trend_store_part_name),
            trends,
            generated_trends: Vec::new(),
            has_alias_column,
        });
    }

    parts
}

pub async fn load_trend_stores(conn: &mut Client) -> Result<Vec<TrendStore>, Error> {
    let mut trend_stores: Vec<TrendStore> = Vec::new();

    let query = concat!(
        "SELECT trend_store.id, data_source.name, entity_type.name, granularity::text, partition_size::text, retention_period::text ",
        "FROM trend_directory.trend_store ",
        "JOIN directory.data_source ON data_source.id = trend_store.data_source_id ",
        "JOIN directory.entity_type ON entity_type.id = trend_store.entity_type_id"
    );

    let result = conn.query(query, &[]).await.unwrap();

    for row in result {
        let trend_store_id: i32 = row.get(0);
        let data_source: &str = row.get(1);
        let entity_type: &str = row.get(2);
        let granularity_str: String = row.get(3);
        let partition_size_str: String = row.get(4);
        let retention_period_str: String = row.get(5);
        let parts = load_trend_store_parts(conn, trend_store_id).await;

        // Hack for humankind parsing compatibility with PostgreSQL interval
        // representation
        let granularity = parse_interval(&granularity_str).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Error parsing granularity '{}': {}",
                &granularity_str, e
            ))
        })?;

        let partition_size = parse_interval(&partition_size_str).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Error parsing partition size '{}': {}",
                &partition_size_str, e
            ))
        })?;

        let retention_period = parse_interval(&retention_period_str).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Error parsing retention period '{}': {}",
                &retention_period_str, e
            ))
        })?;

        trend_stores.push(TrendStore {
            title: None,
            data_source: String::from(data_source),
            entity_type: String::from(entity_type),
            granularity,
            partition_size,
            retention_period,
            parts,
        });
    }

    Ok(trend_stores)
}

pub fn load_trend_store_from_file(path: &PathBuf) -> Result<TrendStore, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open trend store definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    if path.extension() == Some(std::ffi::OsStr::new("yaml")) {
        let trend_store: TrendStore = serde_yaml::from_reader(f).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not read trend store definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(trend_store)
    } else if path.extension() == Some(std::ffi::OsStr::new("json")) {
        let trend_store: TrendStore = serde_json::from_reader(f).map_err(|e| {
            RuntimeError::from(format!(
                "Could not read trend store definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(trend_store)
    } else {
        return Err(ConfigurationError::from_msg(format!(
            "Unsupported trend store definition format '{}'",
            path.extension().unwrap().to_string_lossy()
        ))
        .into());
    }
}

/// Create partitions for the full retention period of all trend stores.
pub async fn create_partitions<T: GenericClient>(
    client: &mut T,
    ahead_interval: Option<Duration>,
) -> Result<(), Error> {
    let ahead_interval = match ahead_interval {
        Some(i) => i,
        None => humantime::parse_duration("3days").unwrap(),
    };

    let query = concat!("SELECT id FROM trend_directory.trend_store");

    let result = client
        .query(query, &[])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error loading trend store Ids: {e}")))?;

    for row in result {
        let trend_store_id: i32 = row.get(0);

        create_partitions_for_trend_store(client, trend_store_id, ahead_interval).await?;
    }

    Ok(())
}

pub async fn create_partitions_for_timestamp<T: GenericClient>(
    client: &mut T,
    timestamp: DateTime<Utc>,
) -> Result<(), Error> {
    let query = concat!("SELECT id FROM trend_directory.trend_store");

    let result = client
        .query(query, &[])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error loading trend store Ids: {e}")))?;

    for row in result {
        let trend_store_id: i32 = row.get(0);

        create_partitions_for_trend_store_and_timestamp(client, trend_store_id, timestamp).await?;
    }

    Ok(())
}

pub async fn create_partitions_for_trend_store<T: GenericClient>(
    client: &mut T,
    trend_store_id: i32,
    ahead_interval: Duration,
) -> Result<(), Error> {
    let query = concat!(
        "WITH partition_indexes AS (",
        "SELECT trend_directory.timestamp_to_index(partition_size, t) AS i, p.id AS part_id, p.name AS part_name ",
        "FROM trend_directory.trend_store ",
        "JOIN trend_directory.trend_store_part p ON p.trend_store_id = trend_store.id ",
        "JOIN generate_series(now() - partition_size - trend_store.retention_period, now() + partition_size + $2::text::interval, partition_size) t ON true ",
        "WHERE trend_store.id = $1",
        ") ",
        "SELECT partition_indexes.part_id, partition_indexes.part_name, partition_indexes.i FROM partition_indexes ",
        "LEFT JOIN trend_directory.partition ON partition.index = i AND partition.trend_store_part_id = partition_indexes.part_id ",
        "WHERE partition.id IS NULL",
    );

    let ahead_interval_str = humantime::format_duration(ahead_interval).to_string();

    let result = client
        .query(query, &[&trend_store_id, &ahead_interval_str])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error loading trend store Ids: {e}")))?;

    if !result.is_empty() {
        println!(
            "Creating {} partitions for trend store {}",
            result.len(),
            &trend_store_id
        );
    }

    for row in result {
        let trend_store_part_id: i32 = row.get(0);
        let part_name: String = row.get(1);
        let partition_index: i32 = row.get(2);

        let transaction = client.transaction().await?;

        match create_partition_for_trend_store_part(
            &transaction,
            trend_store_part_id,
            partition_index,
        )
        .await
        {
            Ok(partition_name) => {
                transaction.commit().await?;
                println!(
                    "Created partition for '{}': '{}'",
                    &part_name, &partition_name
                );
            }
            Err(e) => {
                transaction.rollback().await?;
                println!("Error creating partition '{part_name}': {e}");
            }
        }
    }

    Ok(())
}

pub async fn create_partitions_for_trend_store_and_timestamp<T: GenericClient>(
    client: &mut T,
    trend_store_id: i32,
    timestamp: DateTime<Utc>,
) -> Result<(), Error> {
    debug!("Creating partitions for trend store {}", &trend_store_id);

    let query = concat!(
        "WITH partition_indexes AS (",
        "SELECT trend_directory.timestamp_to_index(partition_size, $2) AS i, p.id AS part_id, p.name AS part_name ",
        "FROM trend_directory.trend_store ",
        "JOIN trend_directory.trend_store_part p ON p.trend_store_id = trend_store.id ",
        "WHERE trend_store.id = $1",
        ") ",
        "SELECT partition_indexes.part_id, partition_indexes.part_name, partition_indexes.i FROM partition_indexes ",
        "LEFT JOIN trend_directory.partition ON partition.index = i AND partition.trend_store_part_id = partition_indexes.part_id ",
        "WHERE partition.id IS NULL",
    );

    let result = client
        .query(query, &[&trend_store_id, &timestamp])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error loading trend store Ids: {e}")))?;

    for row in result {
        let trend_store_part_id: i32 = row.get(0);
        let part_name: String = row.get(1);
        let partition_index: i32 = row.get(2);

        let transaction = client.transaction().await?;

        let partition_name = create_partition_for_trend_store_part(
            &transaction,
            trend_store_part_id,
            partition_index,
        )
        .await?;

        transaction.commit().await?;

        debug!(
            "Created partition for '{}': '{}'",
            &part_name, &partition_name
        );
    }

    Ok(())
}

pub async fn columnarize_partitions<T: GenericClient>(client: &mut T) -> Result<(), Error> {
    debug!("Columnarizing partitions");
    let query = concat!(
        "SELECT trend_directory.convert_to_columnar(p) ",
        "FROM trend_directory.partition p ",
        "WHERE trend_directory.needs_columnar_store(p)",
    );

    client
        .query(query, &[])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error columnarizing partitions: {e}")))?;

    Ok(())
}

async fn create_partition_for_trend_store_part(
    client: &Transaction<'_>,
    trend_store_part_id: i32,
    partition_index: i32,
) -> Result<String, Error> {
    let query = concat!(
        "SELECT p.name, (trend_directory.create_partition(p, $2::integer)).name ",
        "FROM trend_directory.trend_store_part p ",
        "WHERE p.id = $1",
    );

    let result = client
        .query_one(query, &[&trend_store_part_id, &partition_index])
        .await
        .map_err(|e| DatabaseError::from_msg(format!("Error creating partition: {e}")))?;

    let partition_name = result.get(1);

    Ok(partition_name)
}

pub struct TrendStat {
    pub name: String,
    pub max_value: Option<String>,
    pub min_value: Option<String>,
}

pub struct AnalyzeResult {
    pub trend_stats: Vec<TrendStat>,
}

pub async fn analyze_trend_store_part(
    client: &mut Client,
    name: &str,
) -> Result<AnalyzeResult, Error> {
    let query = "SELECT tt.name FROM trend_directory.trend_store_part tsp JOIN trend_directory.table_trend tt ON tt.trend_store_part_id = tsp.id WHERE tsp.name = $1";

    let result = client.query(query, &[&name]).await.map_err(|e| {
        DatabaseError::from_msg(format!(
            "Could read trends for trend store part '{name}': {e}"
        ))
    })?;

    let trend_names: Vec<String> = result.iter().map(|row| row.get(0)).collect();

    let max_expressions: Vec<String> = trend_names
        .iter()
        .map(|name| format!("max(\"{name}\")::text"))
        .collect();

    let max_expressions_part = max_expressions.join(", ");

    let query = format!(
        "SELECT {} FROM trend.\"{}\" p ",
        &max_expressions_part, name
    );

    let row = client.query_one(&query, &[]).await.map_err(|e| {
        DatabaseError::from_msg(format!("Could not analyze trend store part '{name}': {e}"))
    })?;

    let trend_stats = trend_names
        .iter()
        .enumerate()
        .map(|(i, name)| TrendStat {
            name: name.clone(),
            max_value: row.get(i),
            min_value: None,
        })
        .collect();

    let result = AnalyzeResult { trend_stats };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_trend_with_defaults() {
        let trend_def = concat!(
            "{",
            "  \"name\": \"Foo\",",
            "  \"data_type\": \"integer\"",
            "}",
        );

        let trend: Trend = serde_json::from_str(trend_def).unwrap();

        assert_eq!(trend.name, "Foo");
        assert_eq!(trend.data_type, DataType::Integer);
    }

    #[test]
    fn serialize_trend() {
        let trend: Trend = Trend {
            name: "MaxPower".to_string(),
            data_type: DataType::Int8,
            description: "The maximum received power in the past measurement period".to_string(),
            entity_aggregation: "SUM".to_string(),
            time_aggregation: "SUM".to_string(),
            extra_data: json!("{}"),
        };

        let trend_def: String = serde_json::to_string(&trend).unwrap();
        let expected_trend_def = "{\"name\":\"MaxPower\",\"data_type\":\"bigint\",\"description\":\"The maximum received power in the past measurement period\",\"time_aggregation\":\"SUM\",\"entity_aggregation\":\"SUM\",\"extra_data\":\"{}\"}";

        assert_eq!(trend_def, expected_trend_def);
    }

    #[test]
    fn convert_integer_to_bigint_value() {
        let integer_value: MeasValue = MeasValue::Integer(Some(42));
        let transformed_value: MeasValue = integer_value.to_value_of(DataType::Int8).unwrap();

        match transformed_value {
            MeasValue::Int8(v) => assert_eq!(v.unwrap(), 42),
            _ => unreachable!(),
        }
    }

    #[test]
    fn convert_bigint_to_numeric_value() {
        let integer_value: MeasValue = MeasValue::Int8(Some(42));
        let transformed_value: MeasValue = integer_value.to_value_of(DataType::Numeric).unwrap();

        match transformed_value {
            MeasValue::Numeric(v) => assert_eq!(v.unwrap(), Decimal::from_i32(42).unwrap()),
            _ => unreachable!(),
        }
    }
}
