use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::EnableTrigger;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerEnable {
    #[arg(help = "trigger name")]
    name: String,
}

#[async_trait]
impl Cmd for TriggerEnable {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = EnableTrigger {
            trigger_name: self.name.clone(),
        };

        let mut tx = client.transaction().await?;

        let message = change.apply(&mut tx).await?;

        tx.commit().await?;

        println!("{message}");

        Ok(())
    }
}
