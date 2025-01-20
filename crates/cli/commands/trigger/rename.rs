use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};
use minerva::trigger::{load_trigger_from_file, RenameTrigger};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerRename {
    #[arg(
        short = 'v',
        long = "verify",
        help = "run basic verification commands after rename"
    )]
    verify: bool,
    #[arg(help = "renamed trigger definition file")]
    definition: PathBuf,
    #[arg(help = "old trigger name")]
    old_name: String,
}

#[async_trait]
impl Cmd for TriggerRename {
    async fn run(&self) -> CmdResult {
        let trigger = load_trigger_from_file(&self.definition)?;

        if trigger.name == self.old_name {
            return Err(Error::Runtime(RuntimeError::from_msg(format!(
                "Old name is the same as new name: '{}' = '{}'",
                &self.old_name, &trigger.name
            ))));
        }

        let mut client = connect_db().await?;

        let change = RenameTrigger {
            trigger,
            verify: self.verify,
            old_name: self.old_name.clone(),
        };

        let mut tx = client.transaction().await?;

        let message = change.apply(&mut tx).await?;

        tx.commit().await?;

        println!("{message}");

        Ok(())
    }
}
