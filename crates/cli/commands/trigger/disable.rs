use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::DisableTrigger;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDisable {
    #[arg(help = "trigger name")]
    name: String,
}

#[async_trait]
impl Cmd for TriggerDisable {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = DisableTrigger {
            trigger_name: self.name.clone(),
        };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}
