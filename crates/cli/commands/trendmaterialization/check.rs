use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};
use minerva::{
    error::RuntimeError,
    trend_materialization::{check_trend_materialization, load_materializations},
};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationCheck {}

impl TrendMaterializationCheck {
    async fn check(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client)
            .await
            .map_err(|e| RuntimeError::from_msg(format!("Could not load materializations: {e}")))?;

        for materialization in materializations {
            let issues = check_trend_materialization(&mut client, &materialization)
                .await
                .unwrap();

            if issues.is_empty() {
                println!("'{}': Ok", &materialization);
            } else {
                println!("'{}':", &materialization);
                for issue in issues {
                    println!(" - {issue}");
                }
            }
        }

        Ok(())
    }
}

impl Cmd for TrendMaterializationCheck {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.check())
    }
}
