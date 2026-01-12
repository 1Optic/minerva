use std::path::PathBuf;

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

impl TrendStoreCreate {
    async fn create(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        println!("Loaded definition, creating trend store");

        let mut client = connect_db().await?;

        let change = AddTrendStore { trend_store };

        change.apply(&mut client).await?;

        println!("Created trend store");

        Ok(())
    }
}

impl Cmd for TrendStoreCreate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
