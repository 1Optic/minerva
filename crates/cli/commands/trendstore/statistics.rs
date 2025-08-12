use async_trait::async_trait;
use clap::Parser;
use minerva::{change::Change, changes::trend_store::CreateStatistics};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreStatistics {
    #[arg(
        help = "trend store part to do statistics on",
        long = "trend-store-part"
    )]
    trend_store_part: Option<String>,
}

#[async_trait]
impl Cmd for TrendStoreStatistics {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;
        let cmd = CreateStatistics {
            trend_store_part_name: self.trend_store_part.clone(),
        };
        cmd.apply(&mut client).await?;

        Ok(())
    }
}
