pub const DEFAULT_CITUS_IMAGE: &str = "citusdata/citus";
pub const DEFAULT_CITUS_TAG: &str = "13.0.3-alpine";
pub const DEFAULT_POSTGRES_USER: &str = "postgres";

use std::fmt::Display;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::path::{Path, PathBuf};

use bollard::image::BuildImageOptions;
use bollard::Docker;
use log::{debug, error, info};

use futures_util::StreamExt;
use rand::distr::{Alphanumeric, SampleString};

use tokio::io::AsyncBufReadExt;
use tokio::time::{sleep, Duration};
use tokio_postgres::config::Config;
use tokio_postgres::{Client, NoTls};

use testcontainers::core::{ContainerPort, ContainerRequest, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

use crate::database::{connect_to_db, create_database, drop_database};
use crate::error::{Error, RuntimeError};

#[must_use]
pub fn generate_name(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

pub fn create_citus_container(
    image_ref: &ImageRef,
    name: &str,
    exposed_port: Option<u16>,
    config_file: &Path,
) -> ContainerRequest<GenericImage> {
    let image = GenericImage::new(image_ref.image_name.clone(), image_ref.image_tag.clone())
        .with_wait_for(WaitFor::message_on_stdout(
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

#[must_use]
pub fn get_available_port(ip_addr: Ipv4Addr) -> Option<u16> {
    (1000..50000).find(|port| port_available(SocketAddr::V4(SocketAddrV4::new(ip_addr, *port))))
}

fn port_available(addr: SocketAddr) -> bool {
    TcpListener::bind(addr).is_ok()
}

pub async fn create_worker_node(
    image_ref: &ImageRef,
    network_name: &str,
    index: u8,
    config_file: &Path,
) -> Result<WorkerNode, crate::error::Error> {
    let container_name = format!("{network_name}_node{index}");
    let container = create_citus_container(image_ref, &container_name, None, config_file)
        .with_network(network_name)
        .start()
        .await
        .map_err(|e| {
            Error::Runtime(format!("Could not start worker node container: {e}").into())
        })?;

    let container_address = container.get_bridge_ip_address().await.unwrap();
    let host = container.get_host().await.unwrap();
    let postgresql_port = 5432;

    let external_port = container
        .get_host_port_ipv4(postgresql_port)
        .await
        .map_err(|e| {
            crate::error::Error::Runtime(
                format!("Could not get coordinator container external port: {e}").into(),
            )
        })?;

    let connector = Connector {
        host,
        port: external_port,
        internal_addr: container_address,
        internal_port: postgresql_port,
    };

    let config = connector.connect_config("postgres");
    let client = connect_to_db(&config, 3).await?;

    create_minerva_roles(&client).await?;

    Ok(WorkerNode {
        container,
        connector,
    })
}

pub async fn add_worker(
    client: &mut Client,
    host: IpAddr,
    port: u16,
) -> Result<(), MinervaClusterError> {
    let max_attempts = 10;
    let mut attempt: usize = 0;

    loop {
        attempt += 1;

        match client
            .execute(
                "SELECT citus_add_node($1, $2)",
                &[&(&host.to_string()), &i32::from(port)],
            )
            .await
        {
            Ok(_) => break Ok(()),
            Err(e) => {
                if attempt >= max_attempts {
                    break Err(MinervaClusterError::AddWorker {
                        attempts: attempt,
                        source: e,
                    });
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
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

            print!("{prefix}{buffer}");
            buffer.clear();
        }
    });
}

pub async fn connect_db(host: url::Host, port: u16) -> Client {
    let mut config = Config::new();

    let config = config
        .host(host.to_string().as_str())
        .port(port)
        .user(DEFAULT_POSTGRES_USER)
        .password("password");

    debug!("Connecting to database host '{host}' port '{port}'");

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

    debug!("Connected to database host '{host}' port '{port}'");

    client
}

#[derive(thiserror::Error, Debug)]
pub enum TestDatabaseError {
    #[error("Could not connect to test database: {0}")]
    Connect(String),
}

impl From<TestDatabaseError> for Error {
    fn from(value: TestDatabaseError) -> Self {
        Error::Runtime(RuntimeError::from_msg(value.to_string()))
    }
}

async fn create_minerva_roles(client: &Client) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "CREATE ROLE minerva NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE",
            &[],
        )
        .await?;
    client
        .execute(
            "CREATE ROLE minerva_writer NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE",
            &[],
        )
        .await?;
    client
        .execute(
            "CREATE ROLE minerva_admin NOSUPERUSER INHERIT NOCREATEDB NOCREATEROLE",
            &[],
        )
        .await?;

    Ok(())
}

pub struct TestDatabase {
    pub name: String,
    connect_config: Config,
}

impl TestDatabase {
    pub async fn drop_database(&self, client: &mut Client) {
        drop_database(client, &self.name).await.unwrap();
    }

    pub async fn connect(&self) -> Result<Client, TestDatabaseError> {
        connect_to_db(&self.connect_config, 3)
            .await
            .map_err(|e| TestDatabaseError::Connect(format!("{e}")))
    }

    #[must_use]
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

    #[must_use]
    pub fn config(&self) -> Config {
        self.connect_config.clone()
    }
}

pub struct ImageRef {
    pub image_name: String,
    pub image_tag: String,
}

impl Display for ImageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.image_name, self.image_tag)
    }
}

#[async_trait::async_trait]
pub trait ImageProvider {
    async fn image(&self) -> ImageRef;
}

pub struct FixedImageProvider {
    pub image_name: String,
    pub image_tag: String,
}

#[async_trait::async_trait]
impl ImageProvider for FixedImageProvider {
    async fn image(&self) -> ImageRef {
        ImageRef {
            image_name: self.image_name.clone(),
            image_tag: self.image_tag.clone(),
        }
    }
}

impl Default for FixedImageProvider {
    fn default() -> Self {
        FixedImageProvider {
            image_name: DEFAULT_CITUS_IMAGE.to_string(),
            image_tag: DEFAULT_CITUS_TAG.to_string(),
        }
    }
}

pub struct BuildImageProvider {
    pub definition_file: PathBuf,
    pub image_name: String,
    pub image_tag: String,
}

#[async_trait::async_trait]
impl ImageProvider for BuildImageProvider {
    async fn image(&self) -> ImageRef {
        let image_ref = ImageRef {
            image_name: self.image_name.clone(),
            image_tag: self.image_tag.clone(),
        };
        let t = image_ref.to_string();
        let docker = Docker::connect_with_socket_defaults().unwrap();

        let build_image_options = BuildImageOptions {
            dockerfile: "docker-image/Dockerfile",
            t: &t,
            ..Default::default()
        };

        let contents: Vec<u8> = std::fs::read(&self.definition_file).unwrap();

        let mut image_build_stream =
            docker.build_image(build_image_options, None, Some(contents.into()));

        while let Some(msg) = image_build_stream.next().await {
            match msg {
                Err(e) => {
                    error!("{e}");
                }
                Ok(build_info) => {
                    if let Some(text) = build_info.stream {
                        info!("{text}");
                    }
                }
            }
        }

        image_ref
    }
}

pub struct MinervaClusterConfig {
    pub image_provider: Box<dyn ImageProvider + Sync + Send>,
    pub config_file: PathBuf,
    pub worker_count: u8,
    pub prefix: String,
}

impl Default for MinervaClusterConfig {
    fn default() -> Self {
        MinervaClusterConfig {
            image_provider: Box::new(FixedImageProvider::default()),
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            worker_count: 3,
            prefix: generate_name(6),
        }
    }
}

pub struct WorkerNode {
    pub container: ContainerAsync<GenericImage>,
    pub connector: Connector,
}

#[derive(thiserror::Error, Debug)]
pub enum MinervaClusterError {
    #[error("Could not create database: {0}")]
    DatabaseCreation(String),
    #[error("Could not add worker after {attempts} attempts: {source}")]
    AddWorker {
        attempts: usize,
        source: tokio_postgres::Error,
    },
}

#[derive(Clone)]
pub struct Connector {
    pub host: url::Host,
    pub port: u16,
    pub internal_addr: IpAddr,
    pub internal_port: u16,
}

impl Connector {
    pub fn connect_config(&self, database_name: &str) -> Config {
        let mut config = Config::new();

        config
            .host(self.host.to_string())
            .port(self.port)
            .user(DEFAULT_POSTGRES_USER)
            .dbname(database_name)
            .ssl_mode(tokio_postgres::config::SslMode::Disable);

        config
    }
}

#[derive(Clone)]
pub struct MinervaClusterConnector {
    pub worker_connectors: Vec<Connector>,
    pub coordinator_connector: Connector,
}

impl MinervaClusterConnector {
    pub async fn create_db(&self) -> Result<TestDatabase, MinervaClusterError> {
        let database_name = generate_name(16);

        for worker_connector in &self.worker_connectors {
            {
                info!(
                    "Connecting to worker node '{}:{}'",
                    worker_connector.internal_addr, worker_connector.port
                );
                let config = worker_connector.connect_config(DEFAULT_POSTGRES_USER);
                let worker_client = connect_to_db(&config, 3).await.map_err(|e| {
                    MinervaClusterError::DatabaseCreation(format!(
                        "Could not connect to worker node: {e}"
                    ))
                })?;

                create_database(&worker_client, &database_name)
                    .await
                    .map_err(|e| {
                        MinervaClusterError::DatabaseCreation(format!(
                            "Could not create database on worker node: {e}"
                        ))
                    })?;

                info!("Created database '{database_name}' on worker node");
            }

            let worker_client_db =
                connect_to_db(&worker_connector.connect_config(&database_name), 3)
                    .await
                    .map_err(|e| {
                        MinervaClusterError::DatabaseCreation(format!(
                            "Could not connect to new database on worker node: {e}"
                        ))
                    })?;

            worker_client_db
                .execute("CREATE EXTENSION citus", &[])
                .await
                .map_err(|e| {
                    MinervaClusterError::DatabaseCreation(format!(
                        "Could not create Citus extension on worker node: {e}"
                    ))
                })?;

            info!("Created Citus extension on worker node in database '{database_name}'");
        }

        let config = self
            .coordinator_connector
            .connect_config(DEFAULT_POSTGRES_USER);
        let client = connect_to_db(&config, 3).await.map_err(|e| {
            MinervaClusterError::DatabaseCreation(format!("Could not connect to coordinator: {e}"))
        })?;

        create_database(&client, &database_name)
            .await
            .map_err(|e| {
                MinervaClusterError::DatabaseCreation(format!(
                    "Could not create database on coordinator: {e}"
                ))
            })?;

        let connect_config = self.coordinator_connector.connect_config(&database_name);

        debug!("Connecting to new database '{database_name}'");

        let mut db_client = connect_to_db(&connect_config, 3).await.map_err(|e| {
            MinervaClusterError::DatabaseCreation(format!(
                "Could not connect to new database on coordinator: {e}"
            ))
        })?;

        db_client
            .execute("CREATE EXTENSION citus", &[])
            .await
            .map_err(|e| {
                MinervaClusterError::DatabaseCreation(format!(
                    "Could not create Citus extension on coordinator: {e}"
                ))
            })?;

        let coordinator_addr = self.coordinator_connector.internal_addr;
        let coordinator_port: i32 = self.coordinator_connector.internal_port.into();

        db_client
            .execute(
                "SELECT citus_set_coordinator_host($1, $2)",
                &[&coordinator_addr.to_string(), &coordinator_port],
            )
            .await
            .map_err(|e| {
                MinervaClusterError::DatabaseCreation(format!(
                    "Could not set Citus coordinator host in new database on coordinator: {e}"
                ))
            })?;

        for worker_connector in &self.worker_connectors {
            add_worker(
                &mut db_client,
                worker_connector.internal_addr,
                worker_connector.internal_port,
            )
            .await
            .map_err(|e| {
                MinervaClusterError::DatabaseCreation(format!(
                    "Could not connect worker node to coordinator: {e}"
                ))
            })?;
        }

        Ok(TestDatabase {
            name: database_name.clone(),
            connect_config,
        })
    }
}

pub struct MinervaCluster {
    pub controller_container: ContainerAsync<GenericImage>,
    pub workers: Vec<std::pin::Pin<Box<WorkerNode>>>,
    pub connector: MinervaClusterConnector,
    pub network: String,
}

impl MinervaCluster {
    pub async fn start(
        config: &MinervaClusterConfig,
    ) -> Result<MinervaCluster, crate::error::Error> {
        let network_name = config.prefix.clone();

        let image_ref = config.image_provider.image().await;

        let controller_container = create_citus_container(
            &image_ref,
            &format!("{network_name}_coordinator"),
            Some(5432),
            &config.config_file,
        )
        .with_network(network_name.clone())
        .start()
        .await
        .map_err(|e| {
            crate::error::Error::Runtime(
                format!("Could not create coordinator container of image '{image_ref}': {e}")
                    .into(),
            )
        })?;

        print_stdout(
            "Coordinator STDOUT: ".to_string(),
            controller_container.stdout(true),
        );

        print_stdout(
            "Coordinator STDERR: ".to_string(),
            controller_container.stderr(true),
        );

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

        let coordinator_addr = controller_container
            .get_bridge_ip_address()
            .await
            .map_err(|e| {
                crate::error::Error::Runtime(
                    format!("Could not get coordinator container internal host address: {e}")
                        .into(),
                )
            })?;

        let postgresql_port: u16 = 5432;

        let coordinator_connector = Connector {
            host: controller_host,
            port: controller_port,
            internal_addr: coordinator_addr,
            internal_port: postgresql_port,
        };

        debug!(
            "Setting Citus coordinator host address: {}:{}",
            &coordinator_addr, &postgresql_port
        );

        let mut workers = Vec::new();

        let image_ref = config.image_provider.image().await;

        for i in 1..=config.worker_count {
            let worker =
                create_worker_node(&image_ref, &network_name, i, &config.config_file).await?;

            workers.push(Box::pin(worker));
        }

        let cluster_connector = MinervaClusterConnector {
            coordinator_connector,
            worker_connectors: workers.iter().map(|w| w.connector.clone()).collect(),
        };

        let config = cluster_connector
            .coordinator_connector
            .connect_config("postgres");
        let client = connect_to_db(&config, 3).await?;

        create_minerva_roles(&client).await?;

        Ok(MinervaCluster {
            controller_container,
            workers,
            connector: cluster_connector,
            network: network_name,
        })
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.workers.len()
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
        connect_to_db(&self.connect_config(database_name), 3).await
    }

    #[must_use]
    pub fn connect_config(&self, database_name: &str) -> Config {
        let mut config = Config::new();

        config
            .host(self.connector.coordinator_connector.host.to_string())
            .port(self.connector.coordinator_connector.port)
            .user(DEFAULT_POSTGRES_USER)
            .dbname(database_name)
            .ssl_mode(tokio_postgres::config::SslMode::Disable);

        config
    }

    pub async fn create_db(&self) -> Result<TestDatabase, MinervaClusterError> {
        self.connector.create_db().await
    }
}
