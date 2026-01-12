use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::populate_source_fingerprint;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationPopulateSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

impl TrendMaterializationPopulateSourceFingerprint {
    async fn populate(&self) -> CmdResult {
        let mut client = connect_db().await?;

        populate_source_fingerprint(&mut client, &self.materialization)
            .await
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!(
                        "Error populating state for trend materialization '{}': {e}",
                        &self.materialization
                    ),
                })
            })?;

        println!(
            "Populated state for trend materialization '{}'",
            &self.materialization
        );

        Ok(())
    }
}

impl Cmd for TrendMaterializationPopulateSourceFingerprint {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.populate())
    }
}
