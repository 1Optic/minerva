use std::env;
use std::time::Duration;

use log::debug;
use postgres_secrets::PgPass;
use rustls::ClientConfig as RustlsClientConfig;

use tokio_postgres::{config::SslMode, Config};
use tokio_postgres::{Client, NoTls};
use tokio_postgres_rustls::MakeRustlsConnect;

use serde::{Deserialize, Serialize};

use crate::error::DatabaseError;

use super::error::{ConfigurationError, Error};

static ENV_DB_CONN: &str = "MINERVA_DB_CONN";

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterConfig {
    pub nodes: Vec<Node>,
}

pub fn get_db_config() -> Result<Config, Error> {
    let config = if let Ok(value) = env::var(ENV_DB_CONN) {
        Config::new().options(&value).clone()
    } else {
        // No single environment variable set, let's check for psql settings
        let port: u16 = env::var("PGPORT").unwrap_or("5432".into()).parse().unwrap();
        let mut config = Config::new();

        let env_sslmode = env::var("PGSSLMODE").unwrap_or("prefer".into());

        let sslmode = match env_sslmode.to_lowercase().as_str() {
            "disable" => SslMode::Disable,
            "prefer" => SslMode::Prefer,
            "require" => SslMode::Require,
            _ => {
                return Err(Error::Configuration(ConfigurationError {
                    msg: format!("Unsupported SSL mode '{}'", &env_sslmode),
                }))
            }
        };

        let default_user_name = env::var("USER").unwrap_or("postgres".into());

        let host = env::var("PGHOST").unwrap_or("/var/run/postgresql".into());
        let user = env::var("PGUSER").unwrap_or(default_user_name);
        let dbname = env::var("PGDATABASE").unwrap_or("postgres".into());

        let config = config
            .host(&host)
            .port(port)
            .user(&user)
            .dbname(&dbname)
            .ssl_mode(sslmode);

        let pg_password = env::var("PGPASSWORD");

        match pg_password {
            Ok(password) => config.password(password).clone(),
            Err(_) => {
                let pgpass = PgPass::load();
                match pgpass {
                    Ok(pgpass) => {
                        let query = pgpass
                            .query()
                            .hostname(&host)
                            .and_then(|q| q.port(port))
                            .and_then(|q| q.database(&dbname))
                            .and_then(|q| q.username(&user));

                        match query {
                            Ok(q) => match q.find() {
                                Ok(Some(creds)) => config.password(creds.password).clone(),
                                _ => config.clone(),
                            },
                            Err(_) => config.clone(),
                        }
                    }
                    Err(_) => config.clone(),
                }
            }
        }
    };

    Ok(config)
}

pub async fn connect_db() -> Result<Client, Error> {
    connect_to_db(&get_db_config()?, 3).await
}

pub async fn connect_to_db(config: &Config, retry_count: usize) -> Result<Client, Error> {
    let client = if config.get_ssl_mode() != SslMode::Disable {
        let mut roots = rustls::RootCertStore::empty();

        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            roots.add(cert).unwrap();
        }

        let tls_config = RustlsClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);

        let mut attempt_num = 1;

        let (client, connection) = loop {
            match config.connect(tls.clone()).await.map_err(|e| {
                ConfigurationError::from_msg(format!(
                    "Could not setup TLS database connection to {:?}: {}",
                    &config, e
                ))
            }) {
                Ok((client, connection)) => break (client, connection),
                Err(e) => {
                    debug!("Could not connect to database on attempt {attempt_num}: {e:?}");

                    if attempt_num > retry_count {
                        return Err(Error::Database(DatabaseError::from_msg(format!(
                            "Failed to connect after {retry_count} retries"
                        ))));
                    }

                    tokio::time::sleep(Duration::from_secs(1)).await;
                    attempt_num += 1;
                }
            };
        };

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });

        client
    } else {
        let mut attempt_num = 1;

        let (client, connection) = loop {
            match config.connect(NoTls).await.map_err(|e| {
                ConfigurationError::from_msg(format!(
                    "Could not setup TLS database connection to {:?}: {}",
                    &config, e
                ))
            }) {
                Ok((client, connection)) => break (client, connection),
                Err(e) => {
                    debug!("Could not connect to database on attempt {attempt_num}: {e:?}");

                    if attempt_num > retry_count {
                        return Err(Error::Database(DatabaseError::from_msg(format!(
                            "Failed to connect after {retry_count} retries"
                        ))));
                    }

                    tokio::time::sleep(Duration::from_secs(1)).await;
                    attempt_num += 1;
                }
            };
        };

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });

        client
    };

    Ok(client)
}

pub async fn create_database(client: &Client, database_name: &str) -> Result<(), String> {
    let query = format!("CREATE DATABASE \"{database_name}\"");

    client
        .execute(&query, &[])
        .await
        .map_err(|e| format!("Error creating database '{database_name}': {e}"))?;

    Ok(())
}

pub async fn drop_database(client: &mut Client, database_name: &str) -> Result<(), String> {
    let query = format!("DROP DATABASE IF EXISTS \"{database_name}\"");

    client
        .execute(&query, &[])
        .await
        .map_err(|e| format!("Error dropping database '{database_name}': {e}"))?;

    Ok(())
}
