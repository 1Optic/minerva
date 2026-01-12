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
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub fn run(&self) -> CmdResult {
        match &self.command {
            TriggerOptCommands::List(list) => list.run(),
            TriggerOptCommands::Create(create) => create.run(),
            TriggerOptCommands::Delete(delete) => delete.run(),
            TriggerOptCommands::Enable(enable) => enable.run(),
            TriggerOptCommands::Disable(disable) => disable.run(),
            TriggerOptCommands::Update(update) => update.run(),
            TriggerOptCommands::Rename(rename) => rename.run(),
            TriggerOptCommands::Dump(dump) => dump.run(),
            TriggerOptCommands::Verify(verify) => verify.run(),
            TriggerOptCommands::PreviewNotifications(preview_notifications) => {
                preview_notifications.run()
            }
            TriggerOptCommands::CreateNotifications(create_notifications) => {
                create_notifications.run()
            }
        }
    }
}
