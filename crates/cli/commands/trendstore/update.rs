use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use dialoguer::Confirm;

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::{load_trend_store, load_trend_store_from_file};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreUpdate {
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
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
                    ignore_trend_extra_data: false,
                    ignore_trend_data_type: false,
                };

                let changes = trend_store_db.diff(&trend_store, diff_options);

                if !changes.is_empty() {
                    println!("Updating trend store");

                    for change in changes {
                        println!("* {change}");

                        if Confirm::new()
                            .with_prompt("Apply change?")
                            .interact()
                            .map_err(|e| {
                                Error::Runtime(RuntimeError {
                                    msg: format!("Could not process input: {e}"),
                                })
                            })?
                        {
                            let mut tx = client.transaction().await?;

                            let apply_result = change.apply(&mut tx).await;

                            match apply_result {
                                Ok(_) => {
                                    tx.commit().await?;
                                    println!("{}", &change);
                                }
                                Err(e) => {
                                    tx.rollback().await?;
                                    println!("Error applying update: {e}");
                                }
                            }
                        }
                    }
                } else {
                    println!("Trend store already up-to-date")
                }

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error loading trend store: {e}"),
            })),
        }
    }
}
