use async_trait::async_trait;
use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationList {}

#[async_trait]
impl Cmd for TrendMaterializationList {
    async fn run(&self) -> CmdResult {
        let client = connect_db().await?;

        let rows = client
            .query(
                "SELECT m.id, tsp.name FROM trend_directory.materialization m JOIN trend_directory.trend_store_part tsp ON tsp.id = m.dst_trend_store_part_id",
                &[]
            )
            .await
            .unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Id", "Name"]);

        for row in rows {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            table.add_row(vec![id.to_string(), name.to_string()]);
        }

        println!("{table}");

        Ok(())
    }
}
