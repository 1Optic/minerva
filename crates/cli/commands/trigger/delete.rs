use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::DeleteTrigger;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDelete {
    #[arg(help = "trigger name")]
    name: String,
}

#[async_trait]
impl Cmd for TriggerDelete {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = DeleteTrigger {
            trigger_name: self.name.clone(),
        };

        let mut tx = client.transaction().await?;

        change.apply(&mut tx).await?;

        tx.commit().await?;

        println!("Deleted trigger '{}'", &self.name);

        Ok(())
    }
}
