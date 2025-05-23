use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use log::info;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
use minerva::schema::create_schema;
use minerva::trend_store::{create_partitions_for_timestamp, TrendStore};

const TEST_CSV_DATA: &str = r"
node,timestamp,outside_temp,inside_temp,power_kwh,freq_power
hillside14,2023-03-25T14:00:00Z,14.4,32.4,55.8,212.4
hillside15,2023-03-25T14:00:00Z,14.5,32.5,55.9,212.5
";

const TEST_CSV_DATA_UPDATE_PARTIAL: &str = r"
node,timestamp,power_kwh,freq_power
hillside15,2023-03-25T14:00:00Z,55.9,200.0
";

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

#[tokio::test]
async fn load_data() -> Result<(), Box<dyn std::error::Error>> {
    integration_tests::setup();

    let cluster_config = MinervaClusterConfig {
        config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
        ..Default::default()
    };

    let cluster = MinervaCluster::start(&cluster_config).await?;

    let data_source_name = "hub";

    let test_database = cluster.create_db().await?;

    info!("Created database '{}'", test_database.name);

    {
        let mut client = test_database.connect().await?;
        create_schema(&mut client).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore { trend_store };

        add_trend_store.apply(&mut client).await?;

        let timestamp = chrono::DateTime::parse_from_rfc3339("2023-03-25T14:00:00+00:00").unwrap();
        create_partitions_for_timestamp(&mut client, timestamp.into()).await?;
    }

    let log_level = std::env::var("RUST_LOG").unwrap_or("error".to_string());

    let mut cmd = Command::cargo_bin("minerva")?;
    cmd.env("RUST_LOG", log_level)
        .env("PGUSER", "postgres")
        .env("PGHOST", cluster.controller_host.to_string())
        .env("PGPORT", cluster.controller_port.to_string())
        .env("PGSSLMODE", "disable")
        .env("PGDATABASE", &test_database.name);

    let mut csv_file = tempfile::tempfile().unwrap();

    csv_file.write_all(TEST_CSV_DATA.as_bytes()).unwrap();

    let instance_root_path = std::fs::canonicalize("../../examples/tiny_instance_v1").unwrap();

    let file_path: PathBuf = [instance_root_path, "sample-data/sample.csv".into()]
        .iter()
        .collect();

    cmd.arg("load-data")
        .arg("--data-source")
        .arg(data_source_name)
        .arg(&file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Finished processing"));

    Ok(())
}

#[tokio::test]
async fn load_data_twice() -> Result<(), Box<dyn std::error::Error>> {
    integration_tests::setup();

    let data_source_name = "hub";

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

        let add_trend_store = AddTrendStore { trend_store };

        add_trend_store.apply(&mut client).await?;
        let timestamp = chrono::DateTime::parse_from_rfc3339("2023-03-25T14:00:00+00:00").unwrap();

        create_partitions_for_timestamp(&mut client, timestamp.into()).await?;
    }

    let log_level = std::env::var("RUST_LOG").unwrap_or("error".to_string());

    let mut cmd = Command::cargo_bin("minerva")?;
    cmd.env("RUST_LOG", log_level.clone())
        .env("PGUSER", "postgres")
        .env("PGHOST", cluster.controller_host.to_string())
        .env("PGPORT", cluster.controller_port.to_string())
        .env("PGSSLMODE", "disable")
        .env("PGDATABASE", &test_database.name);

    let mut csv_file = tempfile::NamedTempFile::new().unwrap();
    csv_file.write_all(TEST_CSV_DATA.as_bytes()).unwrap();

    cmd.arg("load-data")
        .arg("--data-source")
        .arg(data_source_name)
        .arg(csv_file.path());

    let output = cmd.output().unwrap();

    println!("o >> {}", String::from_utf8(output.stdout).unwrap());
    println!("e >> {}", String::from_utf8(output.stderr).unwrap());

    let mut cmd = Command::cargo_bin("minerva")?;
    cmd.env("RUST_LOG", log_level)
        .env("PGUSER", "postgres")
        .env("PGHOST", cluster.controller_host.to_string())
        .env("PGPORT", cluster.controller_port.to_string())
        .env("PGSSLMODE", "disable")
        .env("PGDATABASE", &test_database.name);

    let mut csv_file = tempfile::NamedTempFile::new().unwrap();
    csv_file
        .write_all(TEST_CSV_DATA_UPDATE_PARTIAL.as_bytes())
        .unwrap();

    cmd.arg("load-data")
        .arg("--data-source")
        .arg(data_source_name)
        .arg(csv_file.path());

    let output = cmd.output().unwrap();

    println!("o >> {}", String::from_utf8(output.stdout).unwrap());
    println!("e >> {}", String::from_utf8(output.stderr).unwrap());

    {
        let client = test_database.connect().await?;

        let query = concat!(
            "SELECT freq_power ",
            "FROM trend.hub_node_main_15m t ",
            "JOIN entity.node e ON e.id = t.entity_id ",
            "WHERE e.name = $1 AND t.timestamp = $2::text::timestamptz"
        );

        let row = client
            .query_one(query, &[&"hillside14", &"2023-03-25T14:00:00Z"])
            .await
            .expect("Exactly one record for hillside14");

        let expected_value: Decimal = dec!(212.4);
        let value: Decimal = row.get(0);

        assert_eq!(value, expected_value);

        let row = client
            .query_one(query, &[&"hillside15", &"2023-03-25T14:00:00Z"])
            .await
            .expect("Exactly one record for hillside15");

        let expected_value: Decimal = dec!(200.0);
        let value: Decimal = row.get(0);

        assert_eq!(value, expected_value);
    }

    Ok(())
}
