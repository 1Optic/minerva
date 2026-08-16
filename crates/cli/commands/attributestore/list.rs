use clap::Parser;
use tokio_postgres::{Client, Row};
use comfy_table::{ContentLineStyle, LineStyle, TableStyle};

use crate::commands::common::{Cmd, CmdResult, connect_db};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreList {}

pub async fn list_attribute_stores(conn: &mut Client) -> Result<Vec<(i32, String)>, String> {
    let query = concat!(
        "SELECT ast.id, ast::text ",
        "FROM attribute_directory.attribute_store ast",
    );

    let result = conn.query(query, &[]).await.unwrap();

    let attribute_stores = result
        .into_iter()
        .map(|row: Row| (row.get::<usize, i32>(0), row.get::<usize, String>(1)))
        .collect();

    Ok(attribute_stores)
}

impl AttributeStoreList {
    async fn list(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trend_stores = list_attribute_stores(&mut client).await.unwrap();

        let mut table = comfy_table::Table::new();
        let style = TableStyle::new()
            .top_border(LineStyle::none())
            .header_lines(ContentLineStyle::none().junction('┆'))
            .header_separator(LineStyle::none().junction('╪').fill('═'))
            .content_lines(ContentLineStyle::none().junction('┆'))
            .row_separator(LineStyle::none())
            .bottom_border(LineStyle::none());
        table.load_style(style);
        table.set_header(vec!["Id", "Name"]);

        for trend_store in trend_stores {
            table.add_row(vec![trend_store.0.to_string(), trend_store.1]);
        }

        println!("{table}");

        Ok(())
    }
}

impl Cmd for AttributeStoreList {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.list())
    }
}
