use chrono::{DateTime, Utc};
use clap::Parser;

use minerva::change::Change;
use minerva::trigger::CreateNotifications;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerCreateNotifications {
    #[arg(long = "timestamp", help = "timestamp")]
    timestamp: Option<DateTime<Utc>>,
    #[arg(help = "trigger name")]
    name: String,
}

impl TriggerCreateNotifications {
    async fn create(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = CreateNotifications {
            trigger_name: self.name.clone(),
            timestamp: self.timestamp.unwrap(),
        };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

impl Cmd for TriggerCreateNotifications {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.create())
    }
}
