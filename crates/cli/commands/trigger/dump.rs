use clap::Parser;

use minerva::trigger::{dump_trigger, load_trigger, TriggerError};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDump {
    #[arg(help = "trigger name")]
    name: String,
}

impl TriggerDump {
    async fn dump(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trigger = load_trigger(&mut client, &self.name)
            .await
            .map_err(TriggerError::to_database_error)?;

        let trigger_definition = dump_trigger(&trigger);

        println!("{trigger_definition}");

        Ok(())
    }
}

impl Cmd for TriggerDump {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.dump())
    }
}
