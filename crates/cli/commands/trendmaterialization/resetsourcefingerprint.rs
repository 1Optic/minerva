use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::reset_source_fingerprint;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationResetSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

impl TrendMaterializationResetSourceFingerprint {
    async fn reset(&self) -> CmdResult {
        let mut client = connect_db().await?;

        reset_source_fingerprint(&mut client, &self.materialization)
            .await
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!("Error updating trend materialization: {e}"),
                })
            })?;

        println!("Updated trend materialization");

        Ok(())
    }
}

impl Cmd for TrendMaterializationResetSourceFingerprint {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.reset())
    }
}
