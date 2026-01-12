use clap::Parser;

use minerva::change::Change;
use minerva::trigger::DisableTrigger;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDisable {
    #[arg(help = "trigger name")]
    name: String,
}

impl TriggerDisable {
    async fn disable(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = DisableTrigger {
            trigger_name: self.name.clone(),
        };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

impl Cmd for TriggerDisable {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.disable())
    }
}
