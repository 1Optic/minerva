use async_trait::async_trait;
use clap::Parser;

use minerva::trend_materialization::load_materializations;

use crate::commands::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationDump {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationDump {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client).await?;

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
