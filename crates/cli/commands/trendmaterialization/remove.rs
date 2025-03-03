use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trend_materialization::RemoveTrendMaterialization;

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

        let change = RemoveTrendMaterialization {
            name: self.name.clone(),
        };

        change.apply(&mut client).await?;

        println!("Removed trend materialization '{}'", &self.name);

        Ok(())
    }
}
