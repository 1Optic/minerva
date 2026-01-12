use std::path::PathBuf;

use clap::Parser;

use minerva::attribute_store::{load_attribute_store_from_file, AddAttributeStore, AttributeStore};
use minerva::change::Change;
use minerva::error::{Error, RuntimeError};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreCreate {
    #[arg(help = "attribute store definition file")]
    definition: PathBuf,
}

impl AttributeStoreCreate {
    async fn create(&self) -> CmdResult {
        let attribute_store: AttributeStore = load_attribute_store_from_file(&self.definition)?;

        println!("Loaded definition, creating attribute store");

        let mut client = connect_db().await?;

        let change = AddAttributeStore { attribute_store };

        let result = change.apply(&mut client).await;

        match result {
            Ok(_) => {
                println!("Created attribute store");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error creating attribute store: {e}"),
            })),
        }
    }
}

impl Cmd for AttributeStoreCreate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
