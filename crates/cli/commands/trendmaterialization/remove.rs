use clap::Parser;

use minerva::change::Change;
use minerva::trend_materialization::RemoveTrendMaterialization;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationRemove {
    #[arg(help = "trend materialization name")]
    name: String,
}

impl TrendMaterializationRemove {
    async fn remove(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = RemoveTrendMaterialization {
            name: self.name.clone(),
        };

        change.apply(&mut client).await?;

        println!("Removed trend materialization '{}'", &self.name);

        Ok(())
    }
}

impl Cmd for TrendMaterializationRemove {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.remove())
    }
}
