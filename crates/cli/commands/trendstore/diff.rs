use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::{
    load_trend_store, load_trend_store_from_file,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};


#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDiff {
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TrendStoreDiff {
    async fn run(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        let client = connect_db().await?;

        let result = load_trend_store(
            &client,
            &trend_store.data_source,
            &trend_store.entity_type,
            &trend_store.granularity,
        )
        .await;

        match result {
            Ok(trend_store_db) => {
                let changes = trend_store_db.diff(&trend_store);

                if !changes.is_empty() {
                    println!("Differences with the database");

                    for change in changes {
                        println!("{}", &change);
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
