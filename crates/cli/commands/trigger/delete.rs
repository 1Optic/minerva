use clap::Parser;

use minerva::change::Change;
use minerva::trigger::DeleteTrigger;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TriggerDelete {
    #[arg(help = "trigger name")]
    name: String,
}

impl TriggerDelete {
    async fn delete(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let change = DeleteTrigger {
            trigger_name: self.name.clone(),
        };

        change.apply(&mut client).await?;

        println!("Deleted trigger '{}'", &self.name);

        Ok(())
    }
}

impl Cmd for TriggerDelete {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.delete())
    }
}
