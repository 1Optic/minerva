use std::time::Duration;

use async_trait::async_trait;
use clap::Parser;

use minerva::trend_store::load_trend_store;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDump {
    #[arg(help = "data source of trend store to dump")]
    data_source: String,
    #[arg(help = "entity type of trend store to dump")]
    entity_type: String,
    #[arg(
        help="granularity of trend store to dump",
        value_parser=humantime::parse_duration
    )]
    granularity: Duration,
}

#[async_trait]
impl Cmd for TrendStoreDump {
    async fn run(&self) -> CmdResult {
        let client = connect_db().await?;

        let trend_store = load_trend_store(
            &client,
            &self.data_source,
            &self.entity_type,
            &self.granularity,
        )
        .await?;

        let trend_store_definition = trend_store.dump()?;

        println!("{}", trend_store_definition);

        Ok(())
    }
}
