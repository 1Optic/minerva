use clap::{Parser, Subcommand};

use crate::commands::common::{Cmd, CmdResult};

pub mod compact;
pub mod create;
pub mod list;
pub mod materialize;
pub mod materializecurrptr;
pub mod update;

use compact::AttributeStoreCompact;
use create::AttributeStoreCreate;
use list::AttributeStoreList;
use materialize::AttributeStoreMaterialize;
use materializecurrptr::AttributeStoreMaterializeCurrPtr;
use update::AttributeStoreUpdate;

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreOpt {
    #[command(subcommand)]
    command: AttributeStoreOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum AttributeStoreOptCommands {
    #[command(about = "list existing attribute stores")]
    List(AttributeStoreList),
    #[command(about = "create an attribute store")]
    Create(AttributeStoreCreate),
    #[command(about = "update an attribute store")]
    Update(AttributeStoreUpdate),
    #[command(about = "materialize attribute store curr-ptr table")]
    MaterializeCurrPtr(AttributeStoreMaterializeCurrPtr),
    #[command(about = "compact attribute store history")]
    Compact(AttributeStoreCompact),
    #[command(about = "materialize attribute store")]
    Materialize(AttributeStoreMaterialize),
}

impl AttributeStoreOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            AttributeStoreOptCommands::List(list) => list.run().await,
            AttributeStoreOptCommands::Create(create) => create.run().await,
            AttributeStoreOptCommands::Update(update) => update.run().await,
            AttributeStoreOptCommands::MaterializeCurrPtr(materialize_curr_ptr) => {
                materialize_curr_ptr.run().await
            }
            AttributeStoreOptCommands::Compact(compact) => compact.run().await,
            AttributeStoreOptCommands::Materialize(materialize) => materialize.run().await,
        }
    }
}
