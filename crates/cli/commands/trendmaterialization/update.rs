use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{
    check_trend_materialization, trend_materialization_from_config, UpdateTrendMaterialization,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationUpdate {
    #[arg(help = "trend materialization definition file")]
    definition: PathBuf,
    #[arg(
        short = 'v',
        long = "verify",
        help = "run basic verification commands after creation"
    )]
    verify: bool,
}

#[async_trait]
impl Cmd for TrendMaterializationUpdate {
    async fn run(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, updating trend materialization");
        let mut client = connect_db().await?;

        let change = UpdateTrendMaterialization {
            trend_materialization: trend_materialization.clone(),
        };

        change.apply(&mut client).await?;

        let result = if self.verify {
            let report =
                check_trend_materialization(&mut client, &trend_materialization).await?;

            if report.is_empty() {
                Ok(())
            } else {
                Err(report.join("\n"))
            }
        } else {
            Ok(())
        };

        match result {
            Ok(_) => {
                println!("Updated trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error updating trend materialization: {e}"),
            })),
        }
    }
}
