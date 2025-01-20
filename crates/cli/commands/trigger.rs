use clap::{Parser, Subcommand};

pub mod create;
pub mod createnotifications;
pub mod delete;
pub mod disable;
pub mod dump;
pub mod enable;
pub mod list;
pub mod previewnotifications;
pub mod rename;
pub mod update;
pub mod verify;

use create::TriggerCreate;
use createnotifications::TriggerCreateNotifications;
use delete::TriggerDelete;
use disable::TriggerDisable;
use dump::TriggerDump;
use enable::TriggerEnable;
use list::TriggerList;
use previewnotifications::TriggerPreviewNotifications;
use rename::TriggerRename;
use update::TriggerUpdate;
use verify::TriggerVerify;

use crate::commands::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerOpt {
    #[command(subcommand)]
    command: TriggerOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TriggerOptCommands {
    #[command(about = "list configured triggers")]
    List(TriggerList),
    #[command(about = "create a trigger")]
    Create(TriggerCreate),
    #[command(about = "delete a trigger")]
    Delete(TriggerDelete),
    #[command(about = "enable a trigger")]
    Enable(TriggerEnable),
    #[command(about = "disable a trigger")]
    Disable(TriggerDisable),
    #[command(about = "update a trigger")]
    Update(TriggerUpdate),
    #[command(about = "rename a trigger")]
    Rename(TriggerRename),
    #[command(about = "dump a trigger definition")]
    Dump(TriggerDump),
    #[command(about = "run basic verification on a trigger")]
    Verify(TriggerVerify),
    #[command(about = "preview notifications of a trigger")]
    PreviewNotifications(TriggerPreviewNotifications),
    #[command(about = "create notifications of a trigger")]
    CreateNotifications(TriggerCreateNotifications),
}

impl TriggerOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            TriggerOptCommands::List(list) => list.run().await,
            TriggerOptCommands::Create(create) => create.run().await,
            TriggerOptCommands::Delete(delete) => delete.run().await,
            TriggerOptCommands::Enable(enable) => enable.run().await,
            TriggerOptCommands::Disable(disable) => disable.run().await,
            TriggerOptCommands::Update(update) => update.run().await,
            TriggerOptCommands::Rename(rename) => rename.run().await,
            TriggerOptCommands::Dump(dump) => dump.run().await,
            TriggerOptCommands::Verify(verify) => verify.run().await,
            TriggerOptCommands::PreviewNotifications(preview_notifications) => {
                preview_notifications.run().await
            }
            TriggerOptCommands::CreateNotifications(create_notifications) => {
                create_notifications.run().await
            }
        }
    }
}
