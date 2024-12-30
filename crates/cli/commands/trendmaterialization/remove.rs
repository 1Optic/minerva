use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{RemoveTrendMaterialization, load_trend_materialization};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationRemove {
    #[arg(help = "trend materialization name")]
    name: String,
}

#[async_trait]
impl Cmd for TrendMaterializationRemove {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        let materialization = load_trend_materialization(&mut transaction, &self.name).await.map_err(|e| Error::Runtime(RuntimeError::from_msg(format!("{e}"))))?;

        let change = RemoveTrendMaterialization {
            materialization,
        };

        change.apply(&mut transaction).await?;

        transaction.commit().await?;

        println!("Removed trend materialization '{}'", &self.name);

        Ok(())
    }
}
