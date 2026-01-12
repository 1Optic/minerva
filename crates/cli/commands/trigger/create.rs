use std::path::PathBuf;

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

impl TriggerCreate {
    async fn create(&self) -> CmdResult {
        let trigger = load_trigger_from_file(&self.definition)?;

        println!("Loaded definition, creating trigger");

        let mut client = connect_db().await?;

        let change = AddTrigger {
            trigger,
            verify: self.verify,
        };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

impl Cmd for TriggerCreate {
    fn run(&self) -> CmdResult {
        env_logger::init();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
