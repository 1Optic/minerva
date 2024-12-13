use async_trait::async_trait;
use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::populate_source_fingerprint;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationPopulateSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationPopulateSourceFingerprint {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let result = populate_source_fingerprint(&mut client, &self.materialization).await;

        match result {
            Ok(_) => {
                println!(
                    "Populated state for trend materialization '{}'",
                    &self.materialization
                );

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "Error populating state for trend materialization '{}': {e}",
                    &self.materialization
                ),
            })),
        }
    }
}