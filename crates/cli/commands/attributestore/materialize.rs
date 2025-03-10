use async_trait::async_trait;
use clap::Parser;
use tokio_postgres::Client;

use minerva::attribute_materialization::{
    load_attribute_materialization_by_id, load_attribute_materialization_by_name,
    load_attribute_materializations, AttributeMaterializeError,
};

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

        if let Some(attribute_materialization_id) = self.id {
            let transaction = client.transaction().await?;

            let attribute_materialization =
                load_attribute_materialization_by_id(&transaction, attribute_materialization_id)
                    .await?;

            let materialize_result = attribute_materialization
                .materialize_attribute(&transaction)
                .await
                .map_err(|e| {
                    format!(
                        "Could not materialize attribute data for '{attribute_materialization}': {e}",
                    )
                })?;

            println!(
                "Materialized attribute store {}: {}",
                attribute_materialization.attribute_store,
                materialize_result.materialized_record_count
            );
        } else if let Some(name) = &self.name {
            let transaction = client.transaction().await?;

            let attribute_materialization =
                load_attribute_materialization_by_name(&transaction, name).await?;

            let materialize_result = attribute_materialization
                .materialize_attribute(&transaction)
                .await
                .map_err(|e| {
                    format!(
                        "Could not materialize attribute data for '{attribute_materialization}': {e}"
                    )
                })?;

            transaction.commit().await?;

            println!(
                "Materialized attribute store {}: {}",
                attribute_materialization.attribute_store,
                materialize_result.materialized_record_count
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
    let attribute_materializations = load_attribute_materializations(client)
        .await
        .map_err(|e| AttributeMaterializeError::Unexpected(e.to_string()))?;

    for attribute_materialization in attribute_materializations {
        let materialize_result = attribute_materialization
            .materialize_attribute(client)
            .await?;
        println!(
            "Materialized {}: {} records",
            attribute_materialization, materialize_result.materialized_record_count
        );
    }

    Ok(())
}
