use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use log::info;

use async_trait::async_trait;
use clap::Parser;

use tokio::signal;

use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
use minerva::error::Error;
use minerva::instance::MinervaInstance;
use minerva::schema::create_schema;
use minerva::trend_store::create_partitions;

use super::common::{Cmd, CmdResult, ENV_MINERVA_INSTANCE_ROOT};

#[derive(Debug, Parser, PartialEq)]
pub struct StartOpt {
    #[arg(long = "create-partitions", help = "create partitions")]
    create_partitions: bool,
    #[arg(long = "node-count", help = "number of worker nodes")]
    node_count: Option<u8>,
    #[arg(
        long = "with-definition",
        help = "Minerva instance definition root directory"
    )]
    instance_root: Option<PathBuf>,
}

#[async_trait]
impl Cmd for StartOpt {
    async fn run(&self) -> CmdResult {
        env_logger::init();
        info!("Starting containers");
        let node_count = self.node_count.unwrap_or(3);

        let config_file = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/postgresql.conf"));

        let cluster_config = MinervaClusterConfig {
            config_file,
            worker_count: node_count,
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let test_database = cluster.create_db().await?;

        info!("Connecting to controller");
        {
            info!("Creating Minerva schema");
            let mut client = test_database.connect().await?;

            let mut env = test_database.get_env();

            env.push(("PGSSLMODE".to_string(), "disable".to_string()));

            let query = format!("SET citus.shard_count = {};", cluster.size());

            client.execute(&query, &[]).await?;
            create_schema(&mut client).await?;
            info!("Created Minerva schema");

            let minerva_instance_root_option: Option<PathBuf> = match &self.instance_root {
                Some(root) => Some(root.clone()),
                None => match env::var(ENV_MINERVA_INSTANCE_ROOT) {
                    Ok(v) => Some(PathBuf::from(v)),
                    Err(_) => None,
                },
            };

            if let Some(minerva_instance_root) = minerva_instance_root_option {
                println!(
                    "Initializing from '{}'",
                    minerva_instance_root.to_string_lossy()
                );

                let minerva_instance = MinervaInstance::load_from(&minerva_instance_root);

                env.push((
                    "MINERVA_INSTANCE_ROOT".to_string(),
                    minerva_instance_root.to_string_lossy().to_string(),
                ));

                minerva_instance.initialize(&mut client, &env).await?;

                if self.create_partitions {
                    create_partitions(&mut client, None).await?;
                }

                println!("Initialized");
            }

            let env_file_path = String::from("cluster.env");
            write_env_file(&env_file_path, &env);
        }

        println!("Minerva cluster is running (press CTRL-C to stop)");
        println!("Connect to the cluster on port {}", cluster.controller_port);
        println!("");
        println!(
            "  psql -h localhost -p {} -d {} -U postgres",
            cluster.controller_port, test_database.name
        );
        println!("");
        println!("or:");
        println!("");
        println!(
            "  PGHOST=localhost PGPORT={} PGDATABASE={} PGUSER=postgres PGSSLMODE=disable minerva",
            cluster.controller_port, test_database.name
        );

        signal::ctrl_c().await.map_err(|e| {
            Error::Runtime(format!("Could not start waiting for Ctrl-C: {e}").into())
        })?;

        Ok(())
    }
}

fn write_env_file(file_path: &str, env: &[(String, String)]) {
    let env_file = File::create(file_path).expect("Could not create env file");

    let mut env_buf_writer = BufWriter::new(env_file);

    for (name, value) in env {
        env_buf_writer
            .write_fmt(format_args!("{}={}\n", name, value))
            .unwrap();
    }
}
