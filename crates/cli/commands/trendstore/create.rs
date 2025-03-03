use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::trend_store::load_trend_store_from_file;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreCreate {
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TrendStoreCreate {
    async fn run(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        println!("Loaded definition, creating trend store");

        let mut client = connect_db().await?;

        let change = AddTrendStore { trend_store };

        change.apply(&mut client).await?;

        println!("Created trend store");

        Ok(())
    }
}
