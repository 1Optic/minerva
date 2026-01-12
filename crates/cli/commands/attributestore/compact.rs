use clap::Parser;
use minerva::attribute_store::materialize_curr_ptr::materialize_curr_ptr;
use tokio_postgres::Client;

use minerva::attribute_store::compact::{
    compact_attribute_store_by_id, compact_attribute_store_by_name, CompactError,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreCompact {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
    #[arg(long, help = "compact all modified attribute stores")]
    all_modified: bool,
    #[arg(long, help = "limit how many records to compact")]
    limit: Option<usize>,
    #[arg(long, help = "limit how many records to compact and loop until done")]
    limit_loop: Option<usize>,
    #[arg(long, help = "statement timeout of the executed queries")]
    statement_timeout: Option<String>,
}

impl AttributeStoreCompact {
    async fn compact(&self) -> CmdResult {
        let mut client = connect_db().await?;

        client
            .execute("SET citus.max_intermediate_result_size = -1", &[])
            .await?;

        if let Some(statement_timeout) = &self.statement_timeout {
            let query = format!("SET statement_timeout = {statement_timeout}");

            client.execute(&query, &[]).await?;
        }

        let (limit, loop_until_done) = if self.limit_loop.is_some() {
            (self.limit_loop, true)
        } else if self.limit.is_some() {
            (self.limit, false)
        } else {
            (None, false)
        };

        if let Some(id) = self.id {
            let transaction = client.transaction().await?;

            let result = compact_attribute_store_by_id(&transaction, id, limit).await?;

            transaction.commit().await?;

            println!(
                "Compacted attribute store '{}'({}): {}",
                result.attribute_store_name, result.attribute_store_id, result.record_count
            );
        } else if let Some(name) = &self.name {
            let mut done: bool = false;

            while !done {
                let transaction = client.transaction().await?;

                let result = compact_attribute_store_by_name(&transaction, name, limit).await?;

                transaction.commit().await?;

                println!(
                    "Compacted attribute store '{}'({}): {}",
                    result.attribute_store_name, result.attribute_store_id, result.record_count
                );

                if !loop_until_done || result.record_count == 0 {
                    done = true;
                }
            }
        } else if self.all_modified {
            compact_all_attribute_stores(&mut client, limit).await?;
        }

        Ok(())
    }
}

impl Cmd for AttributeStoreCompact {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.compact())
    }
}

async fn compact_all_attribute_stores(
    client: &mut Client,
    limit: Option<usize>,
) -> Result<(), CompactError> {
    let query = "SELECT ast.id, ast::text FROM attribute_directory.attribute_store ast LEFT JOIN attribute_directory.attribute_store_compacted astc ON astc.attribute_store_id = ast.id JOIN attribute_directory.attribute_store_modified astm ON astm.attribute_store_id = ast.id WHERE astc.compacted IS NULL OR astm.modified <> astc.compacted";

    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        println!("All attribute stores are already compacted, nothing to do");
        return Ok(());
    }

    for row in rows {
        let id = row.get(0);
        let attribute_store_name: String = row.get(1);

        let transaction = client
            .transaction()
            .await
            .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

        println!("Compacting attribute store '{attribute_store_name}'({id})");

        let result = compact_attribute_store_by_id(&transaction, id, limit).await?;

        // When any attribute data is compacted, also update the curr-ptr data
        if result.record_count > 0 {
            println!("Materializing curr-ptr table for attribute store '{attribute_store_name}'");

            let result = materialize_curr_ptr(&transaction, id).await?;

            println!(
                "Materialized curr-ptr table for attribute store '{}': {} records",
                result.attribute_store_name, result.record_count
            );
        }

        transaction
            .commit()
            .await
            .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

        println!(
            "Compacted attribute store '{}'({}): {}",
            result.attribute_store_name, result.attribute_store_id, result.record_count
        );
    }

    Ok(())
}
