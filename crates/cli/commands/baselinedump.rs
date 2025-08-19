use std::env;
use std::path::PathBuf;

use log::info;

use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use testcontainers::core::ExecCommand;

use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
use minerva::error::{Error, RuntimeError};
use minerva::schema::migrate;

use super::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct BaselineDumpOpt {}

#[derive(Serialize, Deserialize)]
struct ClusterConfig {
    image_name: String,
    image_tag: String,
    path: String,
}

#[async_trait]
impl Cmd for BaselineDumpOpt {
    async fn run(&self) -> CmdResult {
        env_logger::init();

        info!("Starting containers");
        let config_file = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/postgresql.conf"));

        let cluster_config = MinervaClusterConfig {
            config_file,
            worker_count: 1,
            print_stdout: false,
            print_stderr: false,
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        info!("Started containers");

        let test_database = cluster.create_db().await.map_err(|e| {
            Error::Runtime(RuntimeError::from_msg(format!(
                "Could not create database: {e}"
            )))
        })?;

        info!("Connecting to controller");
        {
            info!("Creating Minerva schema");
            let mut client = test_database.connect().await?;

            let mut env = test_database.get_env();

            env.push(("PGSSLMODE".to_string(), "disable".to_string()));

            let query = format!("SET citus.shard_count = {};", cluster.size());

            client.execute(&query, &[]).await?;

            let query = "SET citus.multi_shard_modify_mode TO 'sequential'";
            client.execute(query, &[]).await?;
            migrate(&mut client).await?;
            info!("Created Minerva schema");
        }

        let mut cmd_args = vec![
            "pg_dump",
            "-U",
            "postgres",
            "-d",
            &test_database.name,
            "--schema-only",
        ];

        let exclude_schemas = vec![
            "trend_partition",
            "trend",
            "attribute",
            "attribute_history",
            "attribute_staging",
            "attribute_base",
            "trigger_rule",
            "entity",
            "relation",
            "relation_def",
            "alias",
            "alias_def",
            "virtual_entity",
            "cached",
            "cached_def",
            "handover",
            "handover_directory",
            "external",
            "export",
            "staging",
            "dimension",
            "dimension_def",
        ];

        for schema in exclude_schemas {
            cmd_args.push("--exclude-schema");
            cmd_args.push(schema);
        }

        let cmd = ExecCommand::new(cmd_args);

        let mut result = cluster.controller_container.exec(cmd).await.unwrap();

        let stdoutbytes = result.stdout_to_vec().await.unwrap();

        println!("{}", String::from_utf8(stdoutbytes).unwrap());

        Ok(())
    }
}
