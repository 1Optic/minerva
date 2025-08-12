use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use assert_cmd::prelude::*;
use minerva::error::{Error, RuntimeError};

use log::{debug, error, info};
use postgres_protocol::escape::{escape_identifier, escape_literal};
use rand::distr::{Alphanumeric, SampleString};
use rand::Rng;
use serde::{Deserialize, Serialize};
use testcontainers::core::{ContainerPort, ContainerRequest};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tokio::io::AsyncBufReadExt;
use tokio_postgres::Client;
use toxiproxy_rust::proxy::ProxyPack;

use minerva::cluster::{
    MinervaCluster, MinervaClusterConfig, MinervaClusterConnector, TestDatabase,
};
use minerva::schema::{create_schema, SchemaCreationError};

const TOXIPROXY_API_PORT: u16 = 8474;
const DB_PROXIED_PORT: u16 = 5432;
const MINERVA_SERVICE_USER: &str = "minerva_service";

#[must_use]
pub fn generate_name(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

#[must_use]
pub fn get_available_port(ip_addr: Ipv4Addr) -> Option<u16> {
    (1000..50000).find(|port| port_available(SocketAddr::V4(SocketAddrV4::new(ip_addr, *port))))
}

#[must_use]
fn port_available(addr: SocketAddr) -> bool {
    TcpListener::bind(addr).is_ok()
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

pub fn db_config_to_env(config: &tokio_postgres::Config) -> Vec<(String, String)> {
    let host = match &config.get_hosts()[0] {
        tokio_postgres::config::Host::Tcp(tcp_host) => tcp_host.to_string(),
        tokio_postgres::config::Host::Unix(path) => path.to_string_lossy().to_string(),
    };

    let port = config.get_ports()[0].to_string();

    vec![
        ("PGHOST".to_string(), host),
        ("PGPORT".to_string(), port),
        (
            "PGDATABASE".to_string(),
            config.get_dbname().unwrap().to_string(),
        ),
        ("PGUSER".to_string(), config.get_user().unwrap().to_string()),
    ]
}

#[derive(Clone)]
pub struct MinervaServiceConfig {
    pub pg_host: String,
    pub pg_port: String,
    pub pg_sslmode: String,
    pub pg_database: String,
    pub pg_user: String,
    pub service_address: String,
    pub service_port: u16,
}

impl MinervaServiceConfig {
    pub fn from_test_stack(test_stack: &TestStack, database: &str) -> MinervaServiceConfig {
        let service_address = Ipv4Addr::new(127, 0, 0, 1);
        let service_port = get_available_port(service_address).unwrap();

        MinervaServiceConfig {
            pg_host: test_stack.toxiproxy_host.to_string(),
            pg_port: test_stack.proxied_db_port.to_string(),
            pg_sslmode: "disable".to_string(),
            pg_database: database.to_string(),
            pg_user: MINERVA_SERVICE_USER.to_string(),
            service_address: service_address.to_string(),
            service_port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.service_address, self.service_port)
    }
}

pub struct MinervaService {
    pub conf: MinervaServiceConfig,
    pub proc_handle: std::process::Child,
}

impl MinervaService {
    pub fn start(conf: MinervaServiceConfig) -> Result<MinervaService, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("minerva-service")?;

        cmd.env("PGHOST", &conf.pg_host)
            .env("PGPORT", &conf.pg_port)
            .env("PGSSLMODE", &conf.pg_sslmode)
            .env("PGDATABASE", &conf.pg_database)
            .env("PGUSER", &conf.pg_user)
            .env("SERVICE_ADDRESS", &conf.service_address)
            .env("SERVICE_PORT", conf.service_port.to_string());

        let proc_handle = cmd.spawn()?;

        Ok(MinervaService { conf, proc_handle })
    }

    pub async fn wait_for(&mut self) -> Result<(), Error> {
        let service_address = format!("{}:{}", self.conf.service_address, self.conf.service_port);

        let timeout = Duration::from_millis(1000);

        let ipv4_addr: SocketAddr = service_address.parse().unwrap();

        loop {
            let result = TcpStream::connect_timeout(&ipv4_addr, timeout);

            debug!("Trying to connect to service at {ipv4_addr}");

            if result.is_ok() {
                return Ok(());
            } else {
                // Check if process is still running
                let wait_result = self.proc_handle.try_wait().map_err(|e| {
                    RuntimeError::from_msg(format!("Could not wait for service exit: {e}"))
                })?;

                if let Some(status) = wait_result {
                    panic!("Service prematurely exited with code: {status}");
                }

                tokio::time::sleep(timeout).await;
            }
        }
    }

    #[must_use]
    pub fn base_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.conf.service_address, self.conf.service_port
        )
    }
}

impl Drop for MinervaService {
    fn drop(&mut self) {
        if let Err(e) = self.proc_handle.kill() {
            error!("Could not stop web service: {e}")
        } else {
            debug!("Stopped web service")
        }
    }
}

fn toxiproxy_container(name: &str, exposed_ports: &[u16]) -> ContainerRequest<GenericImage> {
    let image_name = "ghcr.io/shopify/toxiproxy";
    let image_tag = "2.11.0";
    let mut image = GenericImage::new(image_name, image_tag);

    for port in exposed_ports {
        image = image.with_exposed_port(ContainerPort::Tcp(*port));
    }

    image.with_container_name(name)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToxicPack {
    pub name: String,
    pub r#type: String,
    pub stream: String,
    pub toxicity: f32,
    pub attributes: HashMap<String, u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum ToxiError {
    #[error("cannot populate proxy: {0}")]
    Populate(String),
    #[error("cannot bring proxy down: {0}")]
    Down(String),
    #[error("cannot add toxic: {0}")]
    AddToxic(String),
}

pub struct ToxiClient {
    client: reqwest::Client,
    addr: String,
}

impl ToxiClient {
    pub fn new(addr: String) -> ToxiClient {
        ToxiClient {
            client: reqwest::Client::new(),
            addr,
        }
    }

    async fn populate(&self, proxies: &[ProxyPack]) -> Result<(), ToxiError> {
        let mut toxiproxy_url = reqwest::Url::parse(&format!("http://{}", self.addr))
            .map_err(|e| ToxiError::Populate(format!("cannot construct URL: {e}")))?;

        toxiproxy_url.set_path("populate");

        self.client
            .post(toxiproxy_url)
            .header("Content-Type", "application/json")
            .json(&proxies)
            .send()
            .await
            .map_err(|e| {
                ToxiError::Populate(format!("could not send request to Toxi service: {e}"))
            })?;

        Ok(())
    }

    async fn down(&self, name: &str) -> Result<(), ToxiError> {
        let mut toxiproxy_url = reqwest::Url::parse(&format!("http://{}", self.addr))
            .map_err(|e| ToxiError::Down(format!("cannot construct URL: {e}")))?;

        toxiproxy_url.set_path(&format!("proxies/{name}"));

        let data: HashMap<String, bool> = HashMap::from_iter([("enabled".to_string(), false)]);

        let response = self
            .client
            .post(toxiproxy_url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await
            .map_err(|e| ToxiError::Down(format!("could not send request to Toxi service: {e}")))?;

        let response_text = response.text().await.unwrap();

        debug!("response: {response_text}");

        Ok(())
    }

    async fn up(&self, name: &str) -> Result<(), ToxiError> {
        let mut toxiproxy_url = reqwest::Url::parse(&format!("http://{}", self.addr))
            .map_err(|e| ToxiError::Down(format!("cannot construct URL: {e}")))?;

        toxiproxy_url.set_path(&format!("proxies/{name}"));

        let data: HashMap<String, bool> = HashMap::from_iter([("enabled".to_string(), true)]);

        let response = self
            .client
            .post(toxiproxy_url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await
            .map_err(|e| ToxiError::Down(format!("could not send request to Toxi service: {e}")))?;

        let response_text = response.text().await.unwrap();

        debug!("response: {response_text}");

        Ok(())
    }

    async fn add_toxic(&self, proxy_name: &str, toxic: &ToxicPack) -> Result<(), ToxiError> {
        let mut toxiproxy_url = reqwest::Url::parse(&format!("http://{}", self.addr))
            .map_err(|e| ToxiError::Down(format!("cannot construct URL: {e}")))?;

        toxiproxy_url.set_path(&format!("proxies/{proxy_name}/toxics"));

        let response = self
            .client
            .post(toxiproxy_url)
            .header("Content-Type", "application/json")
            .json(&toxic)
            .send()
            .await
            .map_err(|e| {
                ToxiError::AddToxic(format!("could not send request to Toxi service: {e}"))
            })?;

        let response_text = response.text().await.unwrap();

        debug!("response: {response_text}");

        Ok(())
    }

    async fn latency(&self, proxy_name: &str, latency: u32) -> Result<(), ToxiError> {
        let toxic = ToxicPack {
            name: "latency_down".to_string(),
            r#type: "latency".to_string(),
            stream: "downstream".to_string(),
            toxicity: 1.0,
            attributes: HashMap::from_iter([("latency".to_string(), latency)]),
        };

        self.add_toxic(proxy_name, &toxic).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TestStackError {
    #[error("Could not initialize Minerva cluster: {0}")]
    MinervaCluster(String),
    #[error("Could not initialize Toxiproxy: {0}")]
    ToxiProxy(String),
    #[error("Could not create database: {0}")]
    DatabaseCreate(String),
}

pub struct TestStackConfig {
    postgresql_config: PathBuf,
}

impl Default for TestStackConfig {
    fn default() -> Self {
        TestStackConfig {
            postgresql_config: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
        }
    }
}

impl TestStackConfig {
    pub async fn start(&self) -> Result<TestStack, TestStackError> {
        TestStack::start(self).await
    }
}

pub struct TestStack {
    cluster: MinervaCluster,
    #[allow(dead_code)]
    toxiproxy_container: ContainerAsync<GenericImage>,
    pub toxiproxy_host: url::Host,
    /// The outside mapped port on the ToxiProxy container that is proxied to the database.
    pub proxied_db_port: u16,
    toxi_client: ToxiClient,
}

impl TestStack {
    pub async fn start(config: &TestStackConfig) -> Result<TestStack, TestStackError> {
        let prefix = generate_name(8);

        let cluster_config = MinervaClusterConfig {
            config_file: config.postgresql_config.clone(),
            prefix: prefix.clone(),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config)
            .await
            .map_err(|e| TestStackError::MinervaCluster(format!("{e}")))?;

        let toxiproxy_container = toxiproxy_container(
            &format!("{prefix}_toxiproxy"),
            &[TOXIPROXY_API_PORT, DB_PROXIED_PORT],
        )
        .with_network(prefix)
        .start()
        .await
        .map_err(|e| {
            TestStackError::ToxiProxy(format!("Could not start ToxiProxy container: {e}"))
        })?;

        let toxiproxy_addr = toxiproxy_container.get_host().await.map_err(|e| {
            TestStackError::ToxiProxy(format!("Could not get ToxiProxy address: {e}"))
        })?;

        let toxiproxy_port = toxiproxy_container
            .get_host_port_ipv4(ContainerPort::Tcp(TOXIPROXY_API_PORT))
            .await
            .map_err(|e| {
                TestStackError::ToxiProxy(format!("Could not get ToxiProxy API port: {e}"))
            })?;

        let toxi_client = ToxiClient::new(format!("{toxiproxy_addr}:{toxiproxy_port}"));

        let db_internal_addr = cluster
            .controller_container
            .get_bridge_ip_address()
            .await
            .map_err(|e| {
                TestStackError::ToxiProxy(format!("Could not get database controller address: {e}"))
            })?;

        let proxies = vec![ProxyPack::new(
            "db".to_string(),
            format!("0.0.0.0:{DB_PROXIED_PORT}"),
            format!("{db_internal_addr}:5432"),
        )];

        toxi_client.populate(&proxies).await.unwrap();

        let proxied_db_port = toxiproxy_container
            .get_host_port_ipv4(DB_PROXIED_PORT)
            .await
            .unwrap();

        Ok(TestStack {
            cluster,
            toxiproxy_container,
            toxiproxy_host: toxiproxy_addr,
            proxied_db_port,
            toxi_client,
        })
    }

    pub async fn create_db(&self) -> Result<TestDatabase, TestStackError> {
        self.cluster
            .create_db()
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))
    }

    pub async fn create_db_with_minerva_schema(&self) -> Result<TestDatabase, TestStackError> {
        let db = self
            .cluster
            .create_db()
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        let mut client = db
            .connect()
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        create_schema_with_retry(&mut client, 5)
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        Ok(db)
    }

    pub async fn create_db_with_definition(&self) -> Result<TestDatabase, TestStackError> {
        let db = self
            .cluster
            .create_db()
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        let mut client = db
            .connect()
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        create_schema_with_retry(&mut client, 5)
            .await
            .map_err(|e| TestStackError::DatabaseCreate(e.to_string()))?;

        Ok(db)
    }

    pub fn controller_host(&self) -> String {
        self.cluster
            .connector
            .coordinator_connector
            .host
            .to_string()
    }

    pub fn controller_port(&self) -> String {
        self.cluster
            .connector
            .coordinator_connector
            .port
            .to_string()
    }

    pub async fn db_conn_down(&self) -> Result<(), TestStackError> {
        self.toxi_client.down("db").await.map_err(|e| {
            TestStackError::ToxiProxy(format!(
                "Could not bring database connection proxy down: {e}"
            ))
        })
    }

    pub async fn db_conn_up(&self) -> Result<(), TestStackError> {
        self.toxi_client.up("db").await.map_err(|e| {
            TestStackError::ToxiProxy(format!("Could not bring database connection proxy up: {e}"))
        })
    }

    pub async fn db_conn_latency(&self, latency: u32) -> Result<(), TestStackError> {
        self.toxi_client.latency("db", latency).await.map_err(|e| {
            TestStackError::ToxiProxy(format!("Could not set database connection latency: {e}"))
        })
    }

    pub fn db_config(&self, user: &str, database: &str) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();

        config
            .host(
                self.cluster
                    .connector
                    .coordinator_connector
                    .host
                    .to_string(),
            )
            .port(self.cluster.connector.coordinator_connector.port)
            .user(user)
            .dbname(database)
            .ssl_mode(tokio_postgres::config::SslMode::Disable);

        config
    }

    pub fn proxied_db_config(&self, user: &str, database: &str) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();

        config
            .host(self.toxiproxy_host.to_string())
            .port(self.proxied_db_port)
            .user(user)
            .dbname(database)
            .ssl_mode(tokio_postgres::config::SslMode::Disable);

        config
    }
}

pub async fn create_webservice_role(
    minerva_cluster_connector: &MinervaClusterConnector,
    name: &str,
) -> Result<(), Error> {
    // For some reason, when a role is created on a Citus coordinator, this propagates to the
    // worker nodes, but without the full inheritance of roles. That is why we explitly mention all
    // required roles.
    let create_role_sql = format!(
        r#"
DO
$do$
BEGIN
   IF EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE rolname = {}) THEN

      RAISE NOTICE 'Role "{}" already exists. Skipping.';
   ELSE
      BEGIN   -- nested block
        CREATE ROLE {} WITH login IN ROLE minerva_admin, minerva_writer, minerva;
      EXCEPTION
         WHEN duplicate_object THEN
            RAISE NOTICE 'Role "{}" was just created by a concurrent transaction. Skipping.';
      END;
   END IF;
END
$do$;
        "#,
        escape_literal(name),
        name,
        escape_identifier(name),
        name,
    );

    minerva_cluster_connector
        .create_role(&create_role_sql)
        .await?;

    Ok(())
}

pub async fn create_schema_with_retry(
    client: &mut Client,
    max_attempts: usize,
) -> Result<(), SchemaCreationError> {
    let mut attempt = 1;
    let mut initialized: bool = false;

    while !initialized && attempt <= max_attempts {
        let result = create_schema(client).await;

        match result {
            Ok(_) => {
                initialized = true;
            }
            Err(SchemaCreationError::TupleConcurrentlyUpdated) => {
                // This can happen during test running when schema initialization is running
                // parallel in multiple databases, just retry
                info!("Schema creation failed due to concurrent tuple updating.");

                let mut rng = rand::rng();

                let millis = rng.random_range(0..1000);

                tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
            }
            Err(e) => return Err(e),
        }

        attempt += 1;
    }

    Ok(())
}

pub async fn show_roles(
    client: tokio_postgres::Client,
    role: &str,
) -> Result<(), tokio_postgres::Error> {
    let query = r#"
WITH RECURSIVE cte AS (
SELECT oid, 0 AS steps, true AS inherit_option
FROM   pg_roles
WHERE  rolname = $1

UNION ALL
SELECT m.roleid, c.steps + 1, c.inherit_option AND m.inherit_option
FROM   cte c
JOIN   pg_auth_members m ON m.member = c.oid
)
SELECT oid, oid::regrole::text AS rolename, steps, inherit_option
FROM   cte;"#;

    let rows = client.query(query, &[&role]).await?;

    for row in rows {
        let rolename = row.get::<usize, &str>(1);
        let inherit_option = row.get::<usize, bool>(3);

        println!("role: {rolename}, inherit: {inherit_option}");
    }

    Ok(())
}
