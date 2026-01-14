use std::time::Duration;

use clap::Parser;

use minerva::trend_store::{load_trend_store, TrendStoreRef};

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

impl TrendStoreDump {
    async fn dump(&self) -> CmdResult {
        let client = connect_db().await?;

        let trend_store_ref = TrendStoreRef {
            data_source: self.data_source.clone(),
            entity_type: self.entity_type.clone(),
            granularity: self.granularity,
        };

        let trend_store = load_trend_store(&client, &trend_store_ref).await?;

        let trend_store_definition = trend_store.dump()?;

        println!("{}", trend_store_definition);

        Ok(())
    }
}

impl Cmd for TrendStoreDump {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.dump())
    }
}
