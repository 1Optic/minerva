use clap::Parser;

use minerva::{error::RuntimeError, trend_materialization::load_materializations};

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationDump {
    #[arg(help = "materialization ")]
    materialization: String,
}

impl TrendMaterializationDump {
    async fn dump(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client)
            .await
            .map_err(|e| RuntimeError::from_msg(format!("Could not load materializations: {e}")))?;

        for materialization in materializations {
            let materialization_name = materialization.name();
            if materialization_name == self.materialization {
                let definition = materialization.dump().unwrap();

                println!("{definition}");
            }
        }

        Ok(())
    }
}

impl Cmd for TrendMaterializationDump {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.dump())
    }
}
