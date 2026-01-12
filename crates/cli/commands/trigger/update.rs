use std::path::PathBuf;

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

impl TriggerUpdate {
    async fn update(&self) -> CmdResult {
        let trigger = load_trigger_from_file(&self.definition)?;

        let mut client = connect_db().await?;

        let change = UpdateTrigger {
            trigger,
            verify: self.verify,
        };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

impl Cmd for TriggerUpdate {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.update())
    }
}
