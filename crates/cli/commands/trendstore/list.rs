use clap::Parser;

use comfy_table;

use minerva::trend_store::list_trend_stores;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreList {}

impl TrendStoreList {
    async fn list(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trend_stores = list_trend_stores(&mut client).await.unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Id", "Data Source", "Entity Type", "Granularity"]);

        for trend_store in trend_stores {
            table.add_row(vec![
                trend_store.0.to_string(),
                trend_store.1,
                trend_store.2,
                trend_store.3,
            ]);
        }

        println!("{table}");

        Ok(())
    }
}

impl Cmd for TrendStoreList {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.list())
    }
}
