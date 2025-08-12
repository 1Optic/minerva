use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::info;
use rust_decimal::prelude::FromPrimitive;
use tokio_postgres::types::ToSql;
use tokio_postgres::{binary_copy::BinaryCopyInWriter, GenericClient};

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::MinervaClusterConnector;
use minerva::meas_value::{DataType, MeasValue};
use minerva::trend_store::{
    create_partitions_for_timestamp, DataPackage, DataPackageWriteError, MeasurementStore,
    StorePackageError, TrendStore,
};

use crate::common::create_schema_with_retry;

const TREND_STORE_DEFINITION: &str = r"
title: Raw node data
data_source: hub
entity_type: node
granularity: 15m
partition_size: 1d
retention_period: 6 months
parts:
  - name: hub_node_main_15m
    trends:
      - name: outside_temp
        data_type: numeric
      - name: inside_temp
        data_type: numeric
      - name: power_kwh
        data_type: integer
      - name: freq_power
        data_type: numeric
    generated_trends:
      - name: power_Mwh
        data_type: numeric
        description: test
        expression: power_kwh / 1000

";

pub struct RefinedDataPackage {
    timestamp: DateTime<Utc>,
    trends: Vec<String>,
    entity_ids: Vec<i32>,
    entity_names: Option<Vec<String>>,
    job_id: i64,
    rows: Vec<Vec<MeasValue>>,
}

impl RefinedDataPackage {
    fn listed_entity_names(&self) -> Vec<Option<String>> {
        match &self.entity_names {
            Some(list) => list.iter().map(|name| Some(name.to_string())).collect(),
            None => self.entity_ids.iter().map(|_| None).collect(),
        }
    }
}

#[async_trait]
impl DataPackage for RefinedDataPackage {
    fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn trends(&self) -> &[String] {
        &self.trends
    }

    async fn write(
        &self,
        mut writer: std::pin::Pin<&mut BinaryCopyInWriter>,
        values: &[(usize, DataType)],
        created_timestamp: &DateTime<chrono::Utc>,
    ) -> Result<usize, DataPackageWriteError> {
        let entity_names = self.listed_entity_names().clone();
        for (index, entity_id) in self.entity_ids.iter().enumerate() {
            let entity_name = entity_names.get(index).ok_or_else(|| {
                DataPackageWriteError::DataPreparation(format!("No entity name with index {index}"))
            })?;

            let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                vec![entity_id, &self.timestamp, created_timestamp, &self.job_id];
            if let Some(name) = entity_name {
                sql_values.push(name);
            }

            let row = self
                .rows
                .get(index)
                .ok_or_else(|| {
                    DataPackageWriteError::DataPreparation(format!(
                        "No data row with index {index}"
                    ))
                })?
                .clone();

            for (column_index, _data_type) in values {
                let v = row.get(*column_index).ok_or_else(|| {
                    DataPackageWriteError::DataPreparation(format!(
                        "No data column with index {column_index}"
                    ))
                })?;
                sql_values.push(v);
            }

            writer.as_mut().write(&sql_values).await.map_err(|e| {
                let db_error = e.as_db_error();

                match db_error {
                    Some(db_e) => DataPackageWriteError::Generic(format!("dbe: {db_e}")),
                    None => {
                        let text = e.to_string();

                        if text.contains("cannot convert between the Rust type") {
                            DataPackageWriteError::DatatypeMismatch(text)
                        } else {
                            DataPackageWriteError::Generic(text)
                        }
                    }
                }
            })?;
        }

        Ok(self.entity_ids.len())
    }

    async fn insert<C: GenericClient + std::marker::Sync + std::marker::Send>(
        &self,
        client: &mut C,
        query: &str,
        values: &[(usize, DataType)],
        created_timestamp: &DateTime<chrono::Utc>,
    ) -> Result<usize, minerva::error::Error> {
        let mut count: usize = 0;
        let entity_names = self.listed_entity_names().clone();

        for (row_index, entity_id) in self.entity_ids.iter().enumerate() {
            let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                vec![entity_id, &self.timestamp, &created_timestamp, &self.job_id];

            if let Some(name) = entity_names.get(row_index).unwrap() {
                sql_values.push(name);
            }

            let row = self.rows.get(row_index).unwrap();

            for (index, _data_type) in values {
                let v = row.get(*index).unwrap();
                sql_values.push(v);
            }

            client.execute(query, &sql_values).await?;
            count += 1;
        }

        Ok(count)
    }
}

pub async fn store_package(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_database = cluster.create_db().await?;

    info!("Created database '{}'", test_database.name);

    {
        let mut client = test_database.connect().await?;
        create_schema_with_retry(&mut client, 5).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore {
            trend_store: trend_store.clone(),
        };

        add_trend_store.apply(&mut client).await?;

        let timestamp = chrono::DateTime::parse_from_rfc3339("2023-03-25T14:00:00+00:00")
            .unwrap()
            .to_utc();

        create_partitions_for_timestamp(&mut client, timestamp).await?;

        let job_id = 10;

        let trends = vec!["inside_temp".to_string(), "power_kwh".to_string()];

        let trend_store_part = trend_store
            .parts
            .iter()
            .find(|p| p.name == "hub_node_main_15m")
            .unwrap();

        let entity_ids = vec![100];

        let rows = vec![vec![
            MeasValue::Numeric(Some(rust_decimal::Decimal::from_f64(15.0).unwrap())),
            MeasValue::Integer(Some(43)),
        ]];

        let package = RefinedDataPackage {
            timestamp,
            trends: trends.clone(),
            entity_ids: entity_ids.clone(),
            entity_names: None,
            job_id,
            rows,
        };

        trend_store_part
            .store_package(&mut client, &package)
            .await?;

        // Store twice to test the fallback when a unique constraint error is raised
        trend_store_part
            .store_package(&mut client, &package)
            .await?;

        // Create a variant of the trend store part with mismatching data types and check for a
        // corresponding error when trying to store data.

        let mut tsp1 = trend_store_part.clone();
        tsp1.trends.get_mut(1).unwrap().data_type = DataType::Numeric; // inside_temp
        tsp1.trends.get_mut(2).unwrap().data_type = DataType::Numeric; // power_kwh

        let package_mismatching_types = RefinedDataPackage {
            timestamp,
            trends: trends.clone(),
            entity_ids: entity_ids.clone(),
            entity_names: None,
            job_id,
            rows: vec![vec![
                MeasValue::Int8(Some(15)), // inside_temp
                MeasValue::Numeric(Some(rust_decimal::Decimal::from_f64(345.6).unwrap())), // power_kwh
            ]],
        };

        let result = tsp1
            .store_package(&mut client, &package_mismatching_types)
            .await;

        assert_eq!(
            result,
            Err(StorePackageError::DatatypeMismatch(
                "Mismatching data types: [power_kwh(numeric<>integer)]".to_string()
            ))
        );

        // Create another variant of the trend store part with slightly different mismatching data
        // types that should cause a different error internally.

        let mut tsp2 = trend_store_part.clone();
        tsp2.trends.get_mut(1).unwrap().data_type = DataType::Integer; // inside_temp
        tsp2.trends.get_mut(2).unwrap().data_type = DataType::Integer; // power_kwh

        let package_mismatching_types = RefinedDataPackage {
            timestamp,
            trends,
            entity_ids,
            entity_names: None,
            job_id,
            rows: vec![vec![
                MeasValue::Integer(Some(15)),  // inside_temp
                MeasValue::Integer(Some(345)), // power_kwh
            ]],
        };

        let result = tsp2
            .store_package(&mut client, &package_mismatching_types)
            .await;

        assert_eq!(
            result,
            Err(StorePackageError::DatatypeMismatch(
                "Mismatching data types: [inside_temp(integer<>numeric)]".to_string()
            ))
        );
    }

    Ok(())
}
