use std::path::PathBuf;

use async_trait::async_trait;
use clap::{Parser, ValueHint};

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{
    check_trend_materialization, trend_materialization_from_config, AddTrendMaterialization,
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

#[async_trait]
impl Cmd for TrendMaterializationCreate {
    async fn run(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, creating trend materialization");
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        let change = AddTrendMaterialization {
            trend_materialization: trend_materialization.clone(),
        };

        change.apply(&mut transaction).await?;

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
            Ok(_) => {
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
