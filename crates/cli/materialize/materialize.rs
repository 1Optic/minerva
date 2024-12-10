use core::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Instant;

use deadpool_postgres::tokio_postgres;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use futures::StreamExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::time::Interval;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub const CONNECTION_CHECK_INTERVAL: u64 = 60;
pub const MAX_CONNECTION_AGE: u64 = 3600;

#[derive(Debug)]
pub enum MaterializeError {
    UnexpectedError(String),
}

impl std::fmt::Display for MaterializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MaterializeError::UnexpectedError(message) => {
                write!(f, "{}", &message)
            }
        }
    }
}

impl std::error::Error for MaterializeError {}

#[derive(Clone)]
pub struct MaterializeConfig {
    pub tags: Option<Vec<String>>,
    pub oldest_first: bool,
    pub max_materializations: i64,
}

#[derive(Debug, Clone)]
pub struct MaterializationChunk {
    materialization_id: i32,
    name: String,
    timestamp: chrono::DateTime<chrono::Local>,
}

impl std::fmt::Display for MaterializationChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", &self.timestamp, &self.name)
    }
}

impl PartialEq for MaterializationChunk {
    fn eq(&self, other: &Self) -> bool {
        self.materialization_id == other.materialization_id && self.timestamp == other.timestamp
    }
}

impl Hash for MaterializationChunk {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.materialization_id.hash(hasher);
        self.timestamp.hash(hasher);
    }
}

impl Eq for MaterializationChunk {}

impl MaterializationChunk {
    pub fn from_row(row: &tokio_postgres::Row) -> Result<Self, MaterializeError> {
        let materialization_id: i32 = row.try_get(0).map_err(|e| {
            MaterializeError::UnexpectedError(format!("Could not get materialization Id: {e}"))
        })?;
        let name: String = row.try_get(1).map_err(|e| {
            MaterializeError::UnexpectedError(format!("Could not get materialization name: {e}"))
        })?;
        let timestamp: chrono::DateTime<chrono::Local> = row.try_get(2).map_err(|e| {
            MaterializeError::UnexpectedError(format!(
                "Could not get materialization timestamp: {e}"
            ))
        })?;

        Ok(MaterializationChunk {
            materialization_id,
            name,
            timestamp,
        })
    }

    pub async fn check_statistics(
        &self,
        client: &deadpool_postgres::ClientWrapper,
    ) -> Result<(), MaterializeError> {
        // Check if PostgreSQL statistics are up-to-date to prevent
        // inefficient query plans to be used.
        let sources_query = concat!(
            "SELECT timestamp_mapping_func::text, tsp.name ",
            "FROM trend_directory.materialization m ",
            "JOIN trend_directory.materialization_trend_store_link mtsl ON mtsl.materialization_id = m.id ",
            "JOIN trend_directory.trend_store_part tsp ON tsp.id = mtsl.trend_store_part_id ",
            "WHERE m::text = $1",
        );

        let rows = client
            .query(sources_query, &[&self.name])
            .await
            .map_err(|e| {
                let message = format!(
                    "Error getting sources for materialization '{}': {}",
                    &self.name, &e
                );
                MaterializeError::UnexpectedError(message)
            })?;

        for row in rows {
            let materialization_source = MaterializationSource::from_row(&row)?;

            match materialization_source
                .partition_statistics(client, &self.timestamp)
                .await
            {
                Ok(partition_stats) => match partition_stats.stats {
                    Some(_) => {}
                    None => {
                        let result = partition_stats.analyze_timestamp(client).await;

                        match result {
                            Ok(_) => {
                                println!("Updated statistics of '{}'", &partition_stats.name);
                            }
                            Err(e) => {
                                println!(
                                    "Error updating statistics of '{}': {}",
                                    &partition_stats.name, e
                                );
                            }
                        }
                    }
                },
                Err(e) => println!(
                    "Could not fetch or create statistics of {} - {}: {}",
                    &materialization_source, &self.timestamp, e
                ),
            }
        }

        Ok(())
    }

    pub async fn materialize(
        &self,
        client: &deadpool_postgres::ClientWrapper,
    ) -> Result<i32, MaterializeError> {
        let materialize_query = format!(
            "SELECT (trend_directory.materialize(m, '{}'::timestamptz)).row_count FROM trend_directory.materialization m WHERE m::text = '{}'",
            &self.timestamp,
            &self.name
        );

        let result = client.query_one(materialize_query.as_str(), &[]).await;

        match result {
            Ok(row) => {
                let record_count: i32 = row.try_get(0).map_err(|e| {
                    MaterializeError::UnexpectedError(format!("Could not get row count: {e}"))
                })?;

                Ok(record_count)
            }
            Err(e) => Err(MaterializeError::UnexpectedError(format!(
                "Error executing materializing: {e}"
            ))),
        }
    }
}

struct PartitionStats {
    pub name: String,
    pub stats: Option<f32>,
}

impl PartitionStats {
    pub async fn analyze_timestamp(
        &self,
        client: &deadpool_postgres::tokio_postgres::Client,
    ) -> Result<u64, deadpool_postgres::tokio_postgres::Error> {
        let materialize_query = format!("ANALYZE trend_partition.\"{}\"(timestamp)", &self.name);

        client.execute(materialize_query.as_str(), &[]).await
    }
}

struct MaterializationSource {
    pub timestamp_mapping_func: String,
    pub name: String,
}

impl std::fmt::Display for MaterializationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", &self.name, &self.timestamp_mapping_func)
    }
}

impl MaterializationSource {
    pub fn from_row(row: &tokio_postgres::Row) -> Result<Self, MaterializeError> {
        let timestamp_mapping_func: String = row.try_get(0).map_err(|e| {
            MaterializeError::UnexpectedError(format!(
                "Could not get timestamp mapping function: {e}"
            ))
        })?;
        let name: String = row
            .try_get(1)
            .map_err(|e| MaterializeError::UnexpectedError(format!("Could not get name: {e}")))?;

        Ok(Self {
            timestamp_mapping_func,
            name,
        })
    }

    pub async fn empty_partition_statistics(
        &self,
        client: &deadpool_postgres::ClientWrapper,
        timestamp: &chrono::DateTime<chrono::Local>,
    ) -> Result<PartitionStats, MaterializeError> {
        let name_query = concat!(
            "SELECT partition.name ",
            "FROM trend_directory.trend_store_part tsp ",
            "JOIN trend_directory.partition ON partition.trend_store_part_id = tsp.id ",
            "WHERE tsp.name = $1 ",
            "AND partition.from <= $2 ",
            "AND partition.to > $2 ",
            "GROUP BY partition.name"
        );

        let query_result = client.query_one(name_query, &[&self.name, timestamp]).await;

        match query_result {
            Ok(row) => {
                let name: String = row.try_get(0).map_err(|e| {
                    MaterializeError::UnexpectedError(format!("Could not get partition name: {e}"))
                })?;
                let stats: Option<f32> = None;

                Ok(PartitionStats { name, stats })
            }
            Err(e) => Err(MaterializeError::UnexpectedError(format!(
                "Could not load partition data: {e}"
            ))),
        }
    }

    pub async fn partition_statistics(
        &self,
        client: &deadpool_postgres::ClientWrapper,
        timestamp: &chrono::DateTime<chrono::Local>,
    ) -> Result<PartitionStats, MaterializeError> {
        let stats_query = "SELECT partition_name, stats FROM trend_directory.timestamp_statistics($1, $2) WHERE partition_name IS NOT NULL";

        let query_result = client
            .query_one(stats_query, &[&self.name, timestamp])
            .await;

        match query_result {
            Ok(row) => {
                let name: String = row.try_get(0).map_err(|e| {
                    MaterializeError::UnexpectedError(format!("Could not get partition name: {e}"))
                })?;
                let stats: Option<f32> = row.try_get(1).map_err(|e| {
                    MaterializeError::UnexpectedError(format!("Could not get statistics: {e}"))
                })?;

                Ok(PartitionStats { name, stats })
            }
            Err(_) => self.empty_partition_statistics(client, timestamp).await,
        }
    }
}

async fn load_materialization_chunks(
    client: &deadpool_postgres::ClientWrapper,
    config: &MaterializeConfig,
) -> Result<Vec<MaterializationChunk>, MaterializeError> {
    let mut query_parts: Vec<String> = Vec::new();

    query_parts.push(String::from(concat!(
        "SELECT m.id, m::text, ms.timestamp ",
        "FROM trend_directory.materialization_state ms ",
        "JOIN trend_directory.materialization m ",
        "ON m.id = ms.materialization_id ",
        "JOIN trend_directory.trend_store_part tsp ",
        "ON tsp.id = m.dst_trend_store_part_id ",
        "JOIN trend_directory.trend_store ts ON ts.id = tsp.trend_store_id",
    )));

    // Join the tag table only if tag filtering is requested
    if config.tags.is_some() {
        query_parts.push(String::from(concat!(
            "JOIN trend_directory.materialization_tag_link mtl ON mtl.materialization_id = m.id ",
            "JOIN directory.tag ON tag.id = mtl.tag_id",
        )));
    }

    query_parts.push(String::from(concat!(
        "WHERE now() - ts.retention_period < ms.timestamp ",
        "AND (",
        "source_fingerprint != processed_fingerprint OR ",
        "processed_fingerprint IS NULL",
        ") AND m.enabled AND ms.timestamp + m.processing_delay < now() AND ms.timestamp + m.reprocessing_period > now() ",
        "AND (ms.max_modified IS NULL OR (now() - ms.max_modified) > m.stability_delay)"
    )));

    // Only apply tag filtering if a filter is specified
    if let Some(t) = &config.tags {
        let comparison = format!(
            "tag.name = ANY('{{{}}}'::text[])",
            t.iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        query_parts.push(format!("AND {}", comparison));
    }

    let order = match config.oldest_first {
        true => "ASC",
        false => "DESC",
    };

    query_parts.push(format!(
        "ORDER BY ms.timestamp {}, ts.granularity ASC LIMIT $2",
        order
    ));

    let query = query_parts.join(" ");

    let rows = client
        .query(query.as_str(), &[&config.max_materializations.clone()])
        .await
        .map_err(|e| MaterializeError::UnexpectedError(format!("{e}")))?;

    let materialization_chunks = rows
        .into_iter()
        .map(|row| MaterializationChunk::from_row(&row))
        .filter_map(|x| match x {
            Ok(m) => Some(m),
            Err(e) => {
                println!("Error reading materialization chunk: {}", e);

                None
            }
        })
        .collect();

    Ok(materialization_chunks)
}

pub fn create_db_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    if let Ok(db_host) = env::var("DB_HOST") {
        pg_config.host(db_host.as_str());
    }

    if let Ok(db_port) = env::var("DB_PORT") {
        let port: u16 = db_port.parse::<u16>().unwrap();
        pg_config.port(port);
    }

    if let Ok(db_host_path) = env::var("DB_HOST_PATH") {
        pg_config.host_path(db_host_path);
    }

    pg_config.user(env::var("DB_USER").unwrap().as_str());

    if let Ok(db_password) = env::var("DB_PASSWORD") {
        pg_config.password(db_password);
    }

    pg_config.dbname(env::var("DB_NAME").unwrap().as_str());

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);

    Pool::builder(mgr).max_size(16).build().unwrap()
}

pub struct MaterializationFetcher {
    pub check_interval: Interval,
    pub pool: Pool,
    pub in_progress_mutex: Arc<Mutex<HashSet<MaterializationChunk>>>,
    pub materialize_config: MaterializeConfig,
}

impl MaterializationFetcher {
    pub async fn fetch_jobs(mut self, queue_sender: UnboundedSender<MaterializationChunk>) {
        loop {
            self.check_interval.tick().await;

            let client = match self.pool.get().await {
                Ok(v) => v,
                Err(e) => {
                    println!("Error connecting to database: {}", &e);
                    continue;
                }
            };

            let mut guard = self.in_progress_mutex.lock().await;

            match load_materialization_chunks(&client, &self.materialize_config).await {
                Ok(materializations) => {
                    let row_count = materializations.len();
                    let mut new = 0;

                    for materialization in materializations {
                        if guard.insert(materialization.clone()) {
                            let message = format!("{}", &materialization);

                            match queue_sender.send(materialization) {
                                Err(e) => println!(
                                    "Could not queue materialization chunk {}: {}",
                                    &message, e
                                ),
                                Ok(_) => new += 1,
                            }
                        }
                    }

                    let in_progress_count = guard.len();
                    println!(
                        "Loaded {} materialization chunks, queued {} new, queue size: {}",
                        row_count, new, in_progress_count
                    );
                }
                Err(e) => {
                    println!("Error retrieving materializations: {}", e);
                }
            }
        }
    }
}

pub struct MaterializationExecutor {
    pub pool: Pool,
    pub in_progress_mutex: Arc<Mutex<HashSet<MaterializationChunk>>>,
    pub concurrency: usize,
}

async fn materialize(
    pool: Pool,
    in_progress_mutex: Arc<Mutex<HashSet<MaterializationChunk>>>,
    materialization: MaterializationChunk,
) {
    let conn_result = pool.get().await;

    match conn_result {
        Err(e) => {
            println!("Error connecting to database: {}", &e);
        }
        Ok(client) => {
            let start = Instant::now();

            let result = client
                .query_one("SELECT major FROM system.version()", &[])
                .await;
            let mut check_statistics = true;

            match result {
                Ok(row) => {
                    let version: i16 = row.get(0);
                    if version >= 6 {
                        check_statistics = false;
                    }
                }
                Err(e) => {
                    println!("Error checking Minerva version: {}", e)
                }
            }
            if check_statistics {
                let check_result = materialization.check_statistics(&client).await;

                if let Err(e) = check_result {
                    println!("Error checking statistics: {}", e);
                }
            }

            match materialization.materialize(&client).await {
                Ok(record_count) => {
                    let duration = start.elapsed();

                    println!(
                        "Materialized {}: {} ({} ms)",
                        &materialization,
                        record_count,
                        duration.as_millis()
                    );
                }
                Err(e) => {
                    println!("Error materializing {}: {}", &materialization, e);
                }
            }
        }
    }

    let mut guard = in_progress_mutex.lock().await;

    guard.remove(&materialization);
}

impl MaterializationExecutor {
    pub async fn execute(&self, queue_receiver: UnboundedReceiver<MaterializationChunk>) {
        let materializations = UnboundedReceiverStream::new(queue_receiver)
            .map(|materialization| {
                materialize(
                    self.pool.clone(),
                    Arc::clone(&self.in_progress_mutex),
                    materialization,
                )
            })
            .buffer_unordered(self.concurrency);

        materializations
            .for_each(|_| async {
                // Todo: move result reporting here.
            })
            .await;
    }
}
