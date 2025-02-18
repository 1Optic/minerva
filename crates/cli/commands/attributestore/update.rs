use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::attribute_store::{
    load_attribute_store, load_attribute_store_from_file, AttributeStore, AttributeStoreDiffOptions,
};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreUpdate {
    #[arg(help = "attribute store definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for AttributeStoreUpdate {
    async fn run(&self) -> CmdResult {
        let attribute_store: AttributeStore = load_attribute_store_from_file(&self.definition)?;

        println!("Loaded definition, updating attribute store");

        let mut client = connect_db().await?;

        let attribute_store_db = load_attribute_store(
            &mut client,
            &attribute_store.data_source,
            &attribute_store.entity_type,
        )
        .await?;

        let diff_options = AttributeStoreDiffOptions {
            ignore_deletions: false,
        };

        let changes = attribute_store_db.diff(&attribute_store, diff_options);

        if !changes.is_empty() {
            println!("Updating attribute store");

            for change in changes {
                let mut tx = client.transaction().await?;

                let apply_result = change.apply(&mut tx).await;

                match apply_result {
                    Ok(_) => {
                        tx.commit().await?;
                        println!("{}", &change);
                    }
                    Err(e) => {
                        println!("Error applying update: {e}");
                    }
                }
            }
        } else {
            println!("Attribute store already up-to-date");
        }

        Ok(())
    }
}
