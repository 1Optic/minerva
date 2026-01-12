use std::path::PathBuf;

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

impl AttributeStoreUpdate {
    async fn update(&self) -> CmdResult {
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

        if changes.is_empty() {
            println!("Attribute store already up-to-date");
        } else {
            println!("Updating attribute store");

            for change in changes {
                let apply_result = change.apply(&mut client).await;

                match apply_result {
                    Ok(_) => {
                        println!("{}", &change);
                    }
                    Err(e) => {
                        println!("Error applying update: {e}");
                    }
                }
            }
        }

        Ok(())
    }
}

impl Cmd for AttributeStoreUpdate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.update())
    }
}
