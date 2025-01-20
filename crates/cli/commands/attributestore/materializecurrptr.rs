use async_trait::async_trait;
use clap::Parser;
use minerva::attribute_store::materialize_curr_ptr::{
    materialize_curr_ptr, materialize_curr_ptr_by_name,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreMaterializeCurrPtr {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
    #[arg(long, help = "materialize all modified attribute stores")]
    all_modified: bool,
}

#[async_trait]
impl Cmd for AttributeStoreMaterializeCurrPtr {
    async fn run(&self) -> CmdResult {
        let client = connect_db().await?;

        if let Some(id) = self.id {
            println!(
                "Materializing curr-ptr table for attribute store with Id {}",
                id
            );

            let result = materialize_curr_ptr(&client, id).await?;

            println!(
                "Materialized curr-ptr table for attribute store with Id {}: {} records",
                id, result.record_count
            );
        } else if let Some(name) = &self.name {
            println!(
                "Materializing curr-ptr table for attribute store '{}'",
                name
            );

            let result = materialize_curr_ptr_by_name(&client, name).await?;

            println!(
                "Materialized curr-ptr table for attribute store '{}': {} records",
                name, result.record_count
            );
        } else if self.all_modified {
            let query = "SELECT ast.id, ast::text FROM attribute_directory.attribute_store ast LEFT JOIN attribute_directory.attribute_store_curr_materialized ascm ON ascm.attribute_store_id = ast.id LEFT JOIN attribute_directory.attribute_store_modified asm ON asm.attribute_store_id = ascm.attribute_store_id WHERE asm.modified <> ascm.materialized or (ascm.materialized is null and asm.modified is not null)";

            let rows = client.query(query, &[]).await?;

            for row in rows {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);

                println!(
                    "Materializing curr-ptr table for attribute store '{}'",
                    name
                );

                let result = materialize_curr_ptr(&client, id).await?;

                println!(
                    "Materialized curr-ptr table for attribute store '{}': {} records",
                    name, result.record_count
                );
            }
        }

        Ok(())
    }
}
