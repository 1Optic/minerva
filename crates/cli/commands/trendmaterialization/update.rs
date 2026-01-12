use std::path::PathBuf;

use clap::Parser;

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{
    check_trend_materialization, trend_materialization_from_config,
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
    #[arg(
        short = 'L',
        long = "include-logic",
        help = "also update the materialization function or view"
    )]
    include_logic: bool,
}

impl TrendMaterializationUpdate {
    async fn update(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, updating trend materialization");
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        if self.include_logic {
            trend_materialization
                .update_definition(&mut transaction)
                .await?;
        }

        trend_materialization
            .update_sources(&mut transaction)
            .await?;

        trend_materialization
            .update_fingerprint_function(&mut transaction)
            .await?;

        trend_materialization
            .update_attributes(&mut transaction)
            .await?;

        let result = if self.verify {
            let report =
                check_trend_materialization(&mut transaction, &trend_materialization).await?;

            if report.is_empty() {
                Ok(())
            } else {
                Err(report.join("\n"))
            }
        } else {
            Ok(())
        };

        match result {
            Ok(()) => {
                transaction.commit().await?;

                println!("Updated trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error updating trend materialization: {e}"),
            })),
        }
    }
}

impl Cmd for TrendMaterializationUpdate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.update())
    }
}
