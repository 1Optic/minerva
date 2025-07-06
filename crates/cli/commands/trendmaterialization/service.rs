use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use clap::Parser;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use crate::commands::common::{Cmd, CmdResult};

use materialize::materialize::{
    DBConfig, MaterializationChunk, MaterializationExecutor, MaterializationFetcher,
    MaterializeConfig, CONNECTION_CHECK_INTERVAL, MAX_CONNECTION_AGE,
};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationService {
    #[arg(
        long,
        help = "Number of materializations to run concurrently",
        default_value_t = 5
    )]
    concurrency: usize,

    #[arg(
        long,
        help = "Time between calls to the database (in ms)",
        default_value_t = 30000
    )]
    polling_interval: u64,

    #[arg(
        long = "max",
        help = "Maximum number of materializations to run",
        default_value_t = MaterializeConfig::default().max_materializations
    )]
    max_materializations: i64,

    #[arg(long, help = "Run only materializations with this tag")]
    tag: Option<Vec<String>>,

    #[arg(long, help = "Work from oldest to newest", default_value_t = MaterializeConfig::default().oldest_first)]
    oldest_first: bool,
}

#[async_trait]
impl Cmd for TrendMaterializationService {
    async fn run(&self) -> CmdResult {
        env_logger::init();

        let materialize_config = MaterializeConfig {
            oldest_first: self.oldest_first,
            max_materializations: self.max_materializations,
            tags: self.tag.clone(),
        };

        // The set of in-progress materializations. This is used to prevent executing the same materialization multiple times.
        let in_progress: HashSet<MaterializationChunk> = HashSet::new();

        let (queue_sender, queue_receiver) = unbounded_channel::<MaterializationChunk>();

        let mutex = Arc::new(Mutex::new(in_progress));

        let db_config = DBConfig::load_config().map_err(|e| format!("{e}"))?;
        let pool = db_config.create_pool().map_err(|e| format!("{e}"))?;

        let connection_check_interval = Duration::from_secs(CONNECTION_CHECK_INTERVAL);
        let max_connection_age = Duration::from_secs(MAX_CONNECTION_AGE);

        let pool_check_ref = pool.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(connection_check_interval).await;
                pool_check_ref.retain(|_, metrics| {
                    let age = Instant::now().duration_since(metrics.created);
                    let keep = age < max_connection_age;

                    if !keep {
                        println!(
                            "Dropping connection due to age ({age:?} > {max_connection_age:?})",
                        );
                    }

                    keep
                });
            }
        });

        let job_fetcher = MaterializationFetcher {
            check_interval: interval(Duration::from_millis(self.polling_interval)),
            pool: pool.clone(),
            in_progress_mutex: Arc::clone(&mutex),
            materialize_config,
        };

        tokio::spawn(job_fetcher.fetch_jobs(queue_sender));

        let executor = MaterializationExecutor {
            pool: pool.clone(),
            in_progress_mutex: Arc::clone(&mutex),
            concurrency: self.concurrency,
        };

        executor.execute(queue_receiver).await;

        Ok(())
    }
}
