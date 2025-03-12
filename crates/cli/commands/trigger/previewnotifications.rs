use async_trait::async_trait;
use chrono::{DateTime, Local};
use clap::Parser;

use comfy_table::Table;

use minerva::error::DatabaseError;
use minerva::trigger::get_notifications;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerPreviewNotifications {
    #[arg(help = "trigger name")]
    name: String,
    #[arg(help = "timestamp")]
    timestamp: DateTime<Local>,
}

#[async_trait]
impl Cmd for TriggerPreviewNotifications {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let triggers = get_notifications(&mut client, &self.name, self.timestamp)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error getting notifications: {e}")))?;

        let mut table = Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["entity_id", "timestamp", "weight", "data"]);
        for trigger in triggers {
            table.add_row(vec![
                trigger.0.to_string(),
                trigger.1,
                trigger.2.to_string(),
                trigger.4,
            ]);
        }

        println!("{table}");

        Ok(())
    }
}
