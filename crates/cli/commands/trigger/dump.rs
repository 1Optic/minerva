use async_trait::async_trait;
use clap::Parser;

use minerva::trigger::{dump_trigger, load_trigger};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDump {
    #[arg(help = "trigger name")]
    name: String,
}

#[async_trait]
impl Cmd for TriggerDump {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trigger = load_trigger(&mut client, &self.name)
            .await
            .map_err(|e| e.to_database_error())?;

        let trigger_definition = dump_trigger(&trigger);

        println!("{trigger_definition}");

        Ok(())
    }
}
