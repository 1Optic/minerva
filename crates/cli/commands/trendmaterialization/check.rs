use async_trait::async_trait;
use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};
use minerva::trend_materialization::{check_trend_materialization, load_materializations};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationCheck {}

#[async_trait]
impl Cmd for TrendMaterializationCheck {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client).await?;

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
