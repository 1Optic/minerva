pub const DEFAULT_CITUS_IMAGE: &str = "citusdata/citus";
pub const DEFAULT_CITUS_TAG: &str = "12.0";

use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::path::{Path, PathBuf};

use log::{debug, error};

use rand::distributions::{Alphanumeric, DistString};

use tokio::io::AsyncBufReadExt;
use tokio::time::{sleep, Duration};
use tokio_postgres::config::Config;
use tokio_postgres::{Client, NoTls};

use testcontainers::core::{ContainerPort, ContainerRequest, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

use crate::database::{connect_to_db, create_database, drop_database};
use crate::error::Error;

pub fn generate_name(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

pub fn create_citus_container(
    image_name: &str,
    image_tag: &str,
    name: &str,
    exposed_port: Option<u16>,
    config_file: &Path,
) -> ContainerRequest<GenericImage> {
    let image = GenericImage::new(image_name, image_tag).with_wait_for(WaitFor::message_on_stdout(
        "PostgreSQL init process complete; ready for start up.",
    ));

    let image = match exposed_port {
        Some(port) => image.with_exposed_port(ContainerPort::Tcp(port)),
        None => image,
    };

    image
        .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
        .with_container_name(name)
        .with_mount(Mount::bind_mount(
            config_file.to_string_lossy(),
            "/etc/postgresql/postgresql.conf",
        ))
        .with_cmd(vec!["-c", "config-file=/etc/postgresql/postgresql.conf"])
}

pub fn get_available_port(ip_addr: Ipv4Addr) -> Option<u16> {
    (1000..50000).find(|port| port_available(SocketAddr::V4(SocketAddrV4::new(ip_addr, *port))))
}

fn port_available(addr: SocketAddr) -> bool {
    TcpListener::bind(addr).is_ok()
}

pub async fn add_worker(client: &mut Client, host: IpAddr, port: u16) -> Result<(), String> {
    let _count = client
        .execute(
            "SELECT citus_add_node($1, $2)",
            &[&(&host.to_string()), &(port as i32)],
        )
        .await
        .map_err(|e| format!("Could not add worker node: {e}"))?;

    Ok(())
}

pub fn print_stdout<
    I: tokio::io::AsyncBufRead + std::marker::Unpin + std::marker::Send + 'static,
>(
    prefix: String,
    mut reader: I,
) {
    tokio::spawn(async move {
        let mut buffer = String::new();
        loop {
            let result = reader.read_line(&mut buffer).await;

            if let Ok(0) = result {
                break;
            };

            print!("{prefix} - {buffer}");

            buffer.clear();
        }
    });
}

pub async fn connect_db(host: url::Host, port: u16) -> Client {
    let mut config = Config::new();

    let config = config
        .host(host.to_string().as_str())
        .port(port)
        .user("postgres")
        .password("password");

    debug!("Connecting to database host '{}' port '{}'", host, port);

    let (client, connection) = loop {
        let conn_result = config.connect(NoTls).await;

        match conn_result {
            Ok(ok_result) => break ok_result,
            Err(e) => {
                debug!("Error connecting: {e}, retrying");
            }
        }

        sleep(Duration::from_millis(100)).await;
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error {e}");
        }
    });

    debug!("Connected to database host '{}' port '{}'", host, port);

    client
}

pub struct TestDatabase {
    pub name: String,
    connect_config: Config,
}

impl TestDatabase {
    pub async fn drop_database(&self, client: &mut Client) {
        drop_database(client, &self.name).await.unwrap()
    }

    pub async fn connect(&self) -> Result<Client, crate::error::Error> {
        connect_to_db(&self.connect_config).await
    }

    pub fn get_env(&self) -> Vec<(String, String)> {
        let host = match &self.connect_config.get_hosts()[0] {
            tokio_postgres::config::Host::Tcp(tcp_host) => tcp_host.to_string(),
            tokio_postgres::config::Host::Unix(path) => path.to_string_lossy().to_string(),
        };

        let port = self.connect_config.get_ports()[0].to_string();

        vec![
            ("PGHOST".to_string(), host),
            ("PGPORT".to_string(), port),
            ("PGDATABASE".to_string(), self.name.clone()),
            (
                "PGUSER".to_string(),
                self.connect_config.get_user().unwrap().to_string(),
            ),
        ]
    }

    pub fn config(&self) -> Config {
        self.connect_config.clone()
    }
}

pub struct MinervaClusterConfig {
    pub image_name: String,
    pub image_tag: String,
    pub config_file: PathBuf,
    pub worker_count: u8,
}

impl Default for MinervaClusterConfig {
    fn default() -> Self {
        MinervaClusterConfig {
            image_name: DEFAULT_CITUS_IMAGE.to_string(),
            image_tag: DEFAULT_CITUS_TAG.to_string(),
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            worker_count: 3,
        }
    }
}

pub struct MinervaCluster {
    controller_container: ContainerAsync<GenericImage>,
    worker_containers: Vec<std::pin::Pin<Box<ContainerAsync<GenericImage>>>>,
    pub controller_host: url::Host,
    pub controller_port: u16,
}

impl MinervaCluster {
    pub async fn start(
        config: &MinervaClusterConfig,
    ) -> Result<MinervaCluster, crate::error::Error> {
        let network_name = generate_name(6);

        let controller_container = create_citus_container(
            &config.image_name,
            &config.image_tag,
            &format!("{}_coordinator", network_name),
            Some(5432),
            &config.config_file,
        )
        .with_network(network_name.clone())
        .start()
        .await
        .map_err(|e| {
            crate::error::Error::Runtime(
                format!("Could not create coordinator container: {e}").into(),
            )
        })?;

        let controller_host = controller_container.get_host().await.map_err(|e| {
            crate::error::Error::Runtime(
                format!("Could not get coordinator container external host address: {e}").into(),
            )
        })?;

        let controller_port = controller_container
            .get_host_port_ipv4(5432)
            .await
            .map_err(|e| {
                crate::error::Error::Runtime(
                    format!("Could not get coordinator container external port: {e}").into(),
                )
            })?;

        debug!("Connecting to controller");
        let mut client = connect_db(controller_host.clone(), controller_port).await;

        let coordinator_host = controller_container
            .get_bridge_ip_address()
            .await
            .map_err(|e| {
                crate::error::Error::Runtime(
                    format!("Could not get coordinator container internal host address: {e}")
                        .into(),
                )
            })?;

        let coordinator_port: i32 = 5432;
        debug!(
            "Setting Citus coordinator host address: {}:{}",
            &coordinator_host, &coordinator_port
        );

        client
            .execute(
                "SELECT citus_set_coordinator_host($1, $2)",
                &[&coordinator_host.to_string(), &coordinator_port],
            )
            .await
            .unwrap();

        let mut node_containers = Vec::new();

        for i in 1..(config.worker_count + 1) {
            let name = format!("{}_node{i}", network_name);
            let container = create_citus_container(
                &config.image_name,
                &config.image_tag,
                &name,
                None,
                &config.config_file,
            )
            .with_network(network_name.clone())
            .start()
            .await
            .map_err(|e| {
                Error::Runtime(format!("Could not start worker node container: {e}").into())
            })?;

            let container_address = container.get_bridge_ip_address().await.unwrap();

            debug!("Adding worker {container_address}");

            sleep(Duration::from_secs(1)).await;

            add_worker(&mut client, container_address, 5432)
                .await
                .expect("Could not connect worker");

            node_containers.push(Box::pin(container));
        }

        Ok(MinervaCluster {
            controller_container,
            worker_containers: node_containers,
            controller_host,
            controller_port,
        })
    }

    pub fn size(&self) -> usize {
        self.worker_containers.len()
    }

    pub async fn connect_to_coordinator(&self) -> Client {
        let controller_host = self
            .controller_container
            .get_host()
            .await
            .expect("Controller host");
        let controller_port = self
            .controller_container
            .get_host_port_ipv4(5432)
            .await
            .expect("Controller port");

        connect_db(controller_host, controller_port).await
    }

    pub async fn connect_to_db(&self, database_name: &str) -> Result<Client, crate::error::Error> {
        connect_to_db(&self.connect_config(database_name)).await
    }

    pub fn connect_config(&self, database_name: &str) -> Config {
        let mut config = Config::new();

        config
            .host(self.controller_host.to_string())
            .port(self.controller_port)
            .user("postgres")
            .dbname(database_name)
            .ssl_mode(tokio_postgres::config::SslMode::Disable);

        config
    }

    pub async fn create_db(&self) -> Result<TestDatabase, crate::error::Error> {
        let database_name = generate_name(16);
        let mut client = self.connect_to_coordinator().await;
        create_database(&mut client, &database_name).await?;

        Ok(TestDatabase {
            name: database_name.clone(),
            connect_config: self.connect_config(&database_name),
        })
    }
}
