use async_trait::async_trait;
use chrono::{DateTime, Local};
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::CreateNotifications;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerCreateNotifications {
    #[arg(long = "timestamp", help = "timestamp")]
    timestamp: Option<DateTime<Local>>,
    #[arg(help = "trigger name")]
    name: String,
}

#[async_trait]
impl Cmd for TriggerCreateNotifications {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = CreateNotifications {
            trigger_name: self.name.clone(),
            timestamp: self.timestamp,
        };

        let mut tx = client.transaction().await?;

        let message = change.apply(&mut tx).await?;

        tx.commit().await?;

        println!("{message}");

        Ok(())
    }
}
