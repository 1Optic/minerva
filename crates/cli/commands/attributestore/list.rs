use async_trait::async_trait;
use clap::Parser;
use tokio_postgres::{Client, Row};

use crate::commands::common::{connect_db, Cmd, CmdResult};

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

#[async_trait]
impl Cmd for AttributeStoreList {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trend_stores = list_attribute_stores(&mut client).await.unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Id", "Name"]);

        for trend_store in trend_stores {
            table.add_row(vec![trend_store.0.to_string(), trend_store.1]);
        }

        println!("{table}");

        Ok(())
    }
}
