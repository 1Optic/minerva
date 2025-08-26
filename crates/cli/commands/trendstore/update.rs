use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::{load_trend_store, load_trend_store_from_file};

use crate::commands::common::{connect_db, Cmd, CmdResult};
use crate::interact::interact;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreUpdate {
    #[arg(short, long)]
    non_interactive: bool,
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
    #[arg(long)]
    ignore_trend_extra_data: bool,
    #[arg(long)]
    ignore_trend_data_type: bool,
    #[arg(long)]
    ignore_deletions: bool,
}

#[async_trait]
impl Cmd for TrendStoreUpdate {
    async fn run(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        let mut client = connect_db().await?;

        let result = load_trend_store(
            &client,
            &trend_store.data_source,
            &trend_store.entity_type,
            &trend_store.granularity,
        )
        .await;

        match result {
            Ok(trend_store_db) => {
                let diff_options = minerva::trend_store::TrendStoreDiffOptions {
                    ignore_trend_extra_data: self.ignore_trend_extra_data,
                    ignore_trend_data_type: self.ignore_trend_data_type,
                    ignore_deletions: self.ignore_deletions,
                    instance_ignores: Vec::new(),
                };

                let changes = trend_store_db.diff(&trend_store, diff_options);

                if changes.is_empty() {
                    println!("Trend store already up-to-date");
                } else {
                    println!("Updating trend store");

                    let num_changes = changes.len();

                    for (index, change) in changes.iter().enumerate() {
                        println!("* [{}/{num_changes}] {change}", index + 1);

                        if self.non_interactive || interact(&mut client, change.as_ref()).await? {
                            let apply_result = change.apply(&mut client).await;

                            match apply_result {
                                Ok(_) => {
                                    println!("{}", &change);
                                }
                                Err(e) => {
                                    println!("Error applying update: {e}");
                                }
                            }
                        }
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
