use std::path::PathBuf;

use clap::{Parser, ValueHint};

use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{
    check_trend_materialization, trend_materialization_from_config,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationCreate {
    #[arg(help = "trend materialization definition file", value_hint = ValueHint::FilePath)]
    definition: PathBuf,
    #[arg(
        short = 'v',
        long = "verify",
        help = "run basic verification commands after creation"
    )]
    verify: bool,
}

impl TrendMaterializationCreate {
    async fn create(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, creating trend materialization");
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        trend_materialization
            .create(&mut transaction)
            .await
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!(
                        "Error adding trend materialization '{}': {}",
                        &trend_materialization, e
                    ),
                })
            })?;

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

                println!("Created trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error creating trend materialization: {e}"),
            })),
        }
    }
}

impl Cmd for TrendMaterializationCreate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
