
use async_trait::async_trait;
use clap::Parser;
use tokio_postgres::Client;

use minerva::attribute_store::{compact::compact_attribute_store_by_id, materialize::{
    materialize_attribute, AttributeMaterializeError, AttributeStoreRef,
}};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreMaterialize {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
    #[arg(long, help = "limit how many records to compact")]
    limit: Option<usize>,
    #[arg(long, help = "limit how many records to compact and loop until done")]
    limit_loop: Option<usize>,
}

#[async_trait]
impl Cmd for AttributeStoreMaterialize {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        client
            .execute("SET citus.max_intermediate_result_size = -1", &[])
            .await?;

        let (limit, loop_until_done) = if self.limit_loop.is_some() {
            (self.limit_loop, true)
        } else if self.limit.is_some() {
            (self.limit, false)
        } else {
            (None, false)
        };

        if let Some(id) = self.id {
            let transaction = client.transaction().await?;

            let view_name: &str = "";
            let attribute_store = AttributeStoreRef {
                id: 9,
                name: "".to_string(),
            };

            let record_count = materialize_attribute(&transaction, attribute_store, view_name).await.map_err(|e| format!("Could not materialize attribute data: {e}"))?;

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
                    done = true
                }
            }
        } else {
            materialize_all_attribute_stores(&mut client, limit).await?;
        }

        Ok(())
    }
}

async fn materialize_all_attribute_stores(
    client: &mut Client,
    limit: Option<usize>,
) -> Result<(), CompactError> {
    Ok(())
}
