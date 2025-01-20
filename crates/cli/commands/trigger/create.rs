use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::{load_trigger_from_file, AddTrigger};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerCreate {
    #[arg(
        short = 'v',
        long = "verify",
        help = "run basic verification commands after creation"
    )]
    verify: bool,
    #[arg(long = "enable", help = "enable the trigger after creation")]
    enable: bool,
    #[arg(help = "trigger definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TriggerCreate {
    async fn run(&self) -> CmdResult {
        env_logger::init();
        let trigger = load_trigger_from_file(&self.definition)?;

        println!("Loaded definition, creating trigger");

        let mut client = connect_db().await?;

        let change = AddTrigger {
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
