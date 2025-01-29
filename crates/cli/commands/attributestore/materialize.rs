use async_trait::async_trait;
use clap::Parser;
use tokio_postgres::Client;

use minerva::{attribute_materialization::{attribute_materialization_view_name, load_attribute_materializations}, attribute_store::{materialize::{
    materialize_attribute, AttributeMaterializeError, AttributeStoreRef,
}, materialize_curr_ptr::{materialize_curr_ptr, materialize_curr_ptr_by_name}}};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreMaterialize {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
}

#[async_trait]
impl Cmd for AttributeStoreMaterialize {
    async fn run(&self) -> CmdResult {
        env_logger::init();
        let mut client = connect_db().await?;

        client
            .execute("SET citus.max_intermediate_result_size = -1", &[])
            .await?;

        if let Some(id) = self.id {
            let transaction = client.transaction().await?;

            let view_name: &str = "";
            let attribute_store = AttributeStoreRef {
                id: 9,
                name: "".to_string(),
            };

            let materialize_result = materialize_attribute(&transaction, &attribute_store, view_name).await.map_err(|e| format!("Could not materialize attribute data: {e}"))?;

            println!(
                "Compacted attribute store {}: {}",
                attribute_store, materialize_result.materialized_record_count
            );

            let result = materialize_curr_ptr(&transaction, attribute_store.id).await?;

            transaction.commit().await?;

            println!(
                "Compacted attribute store '{}'({}): {}",
                result.attribute_store_name, result.attribute_store_id, result.record_count
            );
        } else if let Some(name) = &self.name {
            let transaction = client.transaction().await?;

            let result = materialize_curr_ptr_by_name(&transaction, name).await?;

            transaction.commit().await?;

            println!(
                "Compacted attribute store '{}'({}): {}",
                result.attribute_store_name, result.attribute_store_id, result.record_count
            );
        } else {
            materialize_all_attribute_stores(&mut client)
                .await
                .map_err(|e| format!("Could not materialize all attribute stores: {e}"))?;
        }

        Ok(())
    }
}

async fn materialize_all_attribute_stores(
    client: &mut Client,
) -> Result<(), AttributeMaterializeError> {
    let attribute_materializations = load_attribute_materializations(client).await.map_err(|e| AttributeMaterializeError::Unexpected(e.to_string()))?;

    for attribute_materialization in attribute_materializations {
        let name = format!("{}_{}", attribute_materialization.attribute_store.data_source, attribute_materialization.attribute_store.entity_type);
        let attribute_store_ref = AttributeStoreRef::from_name(client, &name).await.map_err(|e|AttributeMaterializeError::Unexpected(e.to_string()))?; 
        let view_name = attribute_materialization_view_name(&attribute_materialization.attribute_store);
        let materialize_result = materialize_attribute(client, &attribute_store_ref, &view_name).await?;
        println!("Materialized {}: {} records", attribute_materialization, materialize_result.materialized_record_count);
    }

    Ok(())
}
