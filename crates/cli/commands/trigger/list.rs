use clap::Parser;

use comfy_table::{ContentLineStyle, LineStyle, Table, TableStyle};

use minerva::error::DatabaseError;
use minerva::trigger::list_triggers;

use crate::commands::common::{Cmd, CmdResult, connect_db};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerList {}

impl TriggerList {
    async fn list_triggers(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let triggers = list_triggers(&mut client)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error listing triggers: {e}")))?;

        let mut table = Table::new();
        let style = TableStyle::new()
            .top_border(LineStyle::none())
            .header_lines(ContentLineStyle::none().junction('┆'))
            .header_separator(LineStyle::none().junction('╪').fill('═'))
            .content_lines(ContentLineStyle::none().junction('┆'))
            .row_separator(LineStyle::none())
            .bottom_border(LineStyle::none());
        table.load_style(style);
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

impl Cmd for TriggerList {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.list_triggers())
    }
}
