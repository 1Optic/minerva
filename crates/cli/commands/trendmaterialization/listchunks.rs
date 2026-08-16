use crate::commands::common::{Cmd, CmdResult, connect_db};
use clap::Parser;
use comfy_table::{ContentLineStyle, LineStyle, TableStyle};
use materialize::materialize::{MaterializeConfig, load_materialization_chunks};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationChunkList {
    max_chunks: Option<usize>,
}

impl TrendMaterializationChunkList {
    async fn list_chunks(&self) -> CmdResult {
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
        let style = TableStyle::new()
            .top_border(LineStyle::none())
            .header_lines(ContentLineStyle::none().junction('┆'))
            .header_separator(LineStyle::none().junction('╪').fill('═'))
            .content_lines(ContentLineStyle::none().junction('┆'))
            .row_separator(LineStyle::none())
            .bottom_border(LineStyle::none());
        table.load_style(style);
        table.set_header(vec!["Timestamp", "Name"]);

        for chunk in chunks {
            table.add_row(vec![chunk.timestamp.to_string(), chunk.name.to_string()]);
        }

        println!("{table}");

        Ok(())
    }
}

impl Cmd for TrendMaterializationChunkList {
    fn run(&self) -> CmdResult {
        env_logger::init();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.list_chunks())
    }
}
