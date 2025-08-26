use async_trait::async_trait;
use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};
use materialize::materialize::{load_materialization_chunks, MaterializeConfig};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationChunkList {
    max_chunks: Option<usize>,
}

#[async_trait]
impl Cmd for TrendMaterializationChunkList {
    async fn run(&self) -> CmdResult {
        env_logger::init();

        let client = connect_db().await?;

        let materialize_config = MaterializeConfig {
            max_materializations: 10,
            oldest_first: false,
            tags: None,
        };

        let chunks = load_materialization_chunks(&client, &materialize_config)
            .await
            .unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Timestamp", "Name"]);

        for chunk in chunks {
            table.add_row(vec![chunk.timestamp.to_string(), chunk.name.to_string()]);
        }

        println!("{table}");

        Ok(())
    }
}
