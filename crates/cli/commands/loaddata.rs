use clap::Parser;
use std::path::PathBuf;

use minerva::error::ConfigurationError;
use minerva::loading::{load_data, ParserConfig, TrendsFrom, TrendsFromHeader};

use super::common::{connect_db, Cmd, CmdResult};

static NULL_VALUE: &str = "";

#[derive(Debug, Parser, PartialEq)]
pub struct LoadDataOpt {
    #[arg(long, help = "Data source of data")]
    data_source: Option<String>,
    #[arg(long, help = "File with parser configuration")]
    parser_config: Option<PathBuf>,
    #[arg(long, help = "Create partitions for timestamps in data")]
    create_partitions: bool,
    #[arg(help = "File to load")]
    file: PathBuf,
}

impl LoadDataOpt {
    async fn load_data(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let parser_config: ParserConfig = match &self.parser_config {
            None => ParserConfig {
                entity_type: "node".into(),
                granularity: "15m".into(),
                trends: TrendsFrom::Header(TrendsFromHeader {
                    entity_column: String::from("node"),
                    timestamp_column: String::from("timestamp"),
                }),
                extra: None,
                null_value: NULL_VALUE.to_string(),
            },
            Some(path) => {
                let config_file = std::fs::File::open(path)
                    .map_err(|e| ConfigurationError::from_msg(format!("{e}")))?;
                serde_json::from_reader(config_file).unwrap()
            }
        };

        let data_source = match &self.data_source {
            None => "minerva-cli".to_string(),
            Some(d) => d.to_string(),
        };

        let result = load_data(
            &mut client,
            &data_source,
            &parser_config,
            &self.file,
            self.create_partitions,
        )
        .await;

        match result {
            Err(e) => {
                println!(
                    "Could not load CSV file '{}': {}",
                    &self.file.as_path().to_string_lossy(),
                    e
                );
            }
            Ok(()) => {
                println!(
                    "Finished processing file '{}'",
                    &self.file.as_path().to_string_lossy()
                );
            }
        }

        Ok(())
    }
}

impl Cmd for LoadDataOpt {
    fn run(&self) -> CmdResult {
        env_logger::init();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.load_data())
    }
}
