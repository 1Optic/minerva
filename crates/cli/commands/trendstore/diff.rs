use std::path::PathBuf;

use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::{load_trend_store, load_trend_store_from_file};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDiff {
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
    #[arg(long)]
    ignore_trend_extra_data: bool,
    #[arg(long)]
    ignore_trend_data_type: bool,
    #[arg(long)]
    ignore_deletions: bool,
    #[arg(long)]
    stage_deletions: bool,
}

impl TrendStoreDiff {
    async fn diff(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        let client = connect_db().await?;

        let result = load_trend_store(&client, &(&trend_store).into()).await;

        match result {
            Ok(trend_store_db) => {
                let diff_options = minerva::trend_store::TrendStoreDiffOptions {
                    ignore_trend_extra_data: self.ignore_trend_extra_data,
                    ignore_trend_data_type: self.ignore_trend_data_type,
                    ignore_deletions: self.ignore_deletions,
                    instance_ignores: Vec::new(),
                    stage_deletions: self.stage_deletions,
                };

                let changes = trend_store_db.diff(&trend_store, diff_options);

                if changes.is_empty() {
                    println!("Trend store already up-to-date");
                } else {
                    println!("Differences with the database");

                    for change in changes {
                        println!("{}", &change);
                    }
                }

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error loading trend store: {e}"),
            })),
        }
    }
}

impl Cmd for TrendStoreDiff {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.diff())
    }
}
