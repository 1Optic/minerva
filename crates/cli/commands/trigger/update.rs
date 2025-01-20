use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::{load_trigger_from_file, UpdateTrigger};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerUpdate {
    #[arg(
        short = 'v',
        long = "verify",
        help = "run basic verification commands after update"
    )]
    verify: bool,
    #[arg(help = "trigger definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TriggerUpdate {
    async fn run(&self) -> CmdResult {
        let trigger = load_trigger_from_file(&self.definition)?;

        let mut client = connect_db().await?;

        let change = UpdateTrigger {
            trigger,
            verify: self.verify,
        };

        let mut tx = client.transaction().await?;

        let message = change.apply(&mut tx).await?;

        tx.commit().await?;

        println!("{message}");

        Ok(())
    }
}
