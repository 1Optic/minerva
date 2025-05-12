use std::env;
use std::path::PathBuf;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::info;
use rust_decimal::prelude::FromPrimitive;
use tokio_postgres::types::ToSql;
use tokio_postgres::{binary_copy::BinaryCopyInWriter, GenericClient};

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
use minerva::meas_value::{DataType, MeasValue};
use minerva::schema::create_schema;
use minerva::trend_store::{
    create_partitions_for_timestamp, DataPackage, MeasurementStore, TrendStore,
};

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
        data_type: numeric
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
    job_id: i64,
    rows: Vec<Vec<MeasValue>>,
}

#[async_trait]
impl DataPackage for RefinedDataPackage {
    fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn trends(&self) -> &Vec<String> {
        &self.trends
    }

    async fn write(
        &self,
        mut writer: std::pin::Pin<&mut BinaryCopyInWriter>,
        values: &[(usize, DataType)],
        created_timestamp: &DateTime<chrono::Utc>,
    ) -> Result<usize, minerva::error::Error> {
        for (index, entity_id) in self.entity_ids.iter().enumerate() {
            let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                vec![entity_id, &self.timestamp, created_timestamp, &self.job_id];

            let row = self.rows.get(index).unwrap();

            for (column_index, _data_type) in values {
                let v = row.get(*column_index).unwrap();
                sql_values.push(v);
            }

            writer.as_mut().write(&sql_values).await?;
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

        for (row_index, entity_id) in self.entity_ids.iter().enumerate() {
            let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                vec![entity_id, &self.timestamp, &created_timestamp, &self.job_id];

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

#[tokio::test]
async fn store_package() -> Result<(), Box<dyn std::error::Error>> {
    integration_tests::setup();

    let cluster_config = MinervaClusterConfig {
        config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
        ..Default::default()
    };

    let cluster = MinervaCluster::start(&cluster_config).await?;

    let test_database = cluster.create_db().await?;

    info!("Created database '{}'", test_database.name);

    {
        let mut client = test_database.connect().await?;
        create_schema(&mut client).await?;

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
            MeasValue::Numeric(Some(rust_decimal::Decimal::from_f64(43.0).unwrap())),
        ]];

        let package = RefinedDataPackage {
            timestamp,
            trends,
            entity_ids,
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
    }

    Ok(())
}
