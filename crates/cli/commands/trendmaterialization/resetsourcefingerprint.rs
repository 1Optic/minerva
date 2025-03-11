use async_trait::async_trait;
use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::reset_source_fingerprint;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationResetSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationResetSourceFingerprint {
    async fn run(&self) -> CmdResult {
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
