use async_trait::async_trait;
use clap::Parser;

use comfy_table::Table;

use minerva::error::DatabaseError;
use minerva::trigger::list_triggers;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerList {}

#[async_trait]
impl Cmd for TriggerList {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let triggers = list_triggers(&mut client)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error listing triggers: {e}")))?;

        let mut table = Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec![
            "Name",
            "Notification Store",
            "Granularity",
            "Default Interval",
            "Enabled",
        ]);
        for trigger in triggers {
            table.add_row(vec![
                trigger.name,
                trigger.notification_store,
                trigger.granularity,
                trigger.default_interval,
                trigger.enabled.to_string(),
            ]);
        }

        println!("{table}");

        Ok(())
    }
}
