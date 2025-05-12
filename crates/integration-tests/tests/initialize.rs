use std::path::PathBuf;
use std::process::Command;

use log::debug;

use assert_cmd::prelude::*;
use predicates::prelude::*;

use minerva::cluster::{MinervaCluster, MinervaClusterConfig};

#[tokio::test]
async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    integration_tests::setup();

    let cluster_config = MinervaClusterConfig {
        config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
        ..Default::default()
    };

    let cluster = MinervaCluster::start(&cluster_config).await?;

    debug!("Containers started");

    let test_database = cluster.create_db().await?;

    debug!("Created database '{}'", test_database.name);

    let mut cmd = Command::cargo_bin("minerva")?;
    cmd.env("PGUSER", "postgres")
        .env("PGHOST", cluster.controller_host.to_string())
        .env("PGPORT", cluster.controller_port.to_string())
        .env("PGSSLMODE", "disable")
        .env("PGDATABASE", &test_database.name);

    let instance_root_path = std::fs::canonicalize("../../examples/tiny_instance_v1").unwrap();

    cmd.arg("initialize")
        .arg("--create-schema")
        .arg(&instance_root_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created trigger"));

    let client = test_database.connect().await?;

    let row = client
        .query_one("SELECT count(*) FROM trend_directory.trend_store", &[])
        .await?;

    let trend_store_count: i64 = row.get(0);

    assert_eq!(trend_store_count, 6);

    let row = client
        .query_one("SELECT count(*) FROM trigger.rule", &[])
        .await?;

    let trigger_count: i64 = row.get(0);

    assert_eq!(trigger_count, 4);

    Ok(())
}
