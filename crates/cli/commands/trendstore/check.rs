use std::path::PathBuf;

use clap::Parser;
use async_trait::async_trait;

use crate::commands::common::{Cmd, CmdResult};

use minerva::trend_store::load_trend_store_from_file;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreCheck {
    #[arg(help = "trend store definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TrendStoreCheck {
    async fn run(&self) -> CmdResult {
        let trend_store = load_trend_store_from_file(&self.definition)?;

        for trend_store_part in &trend_store.parts {
            let count = trend_store
                .parts
                .iter()
                .filter(|&p| p.name == trend_store_part.name)
                .count();

            if count > 1 {
                println!(
                    "Error: {} trend store parts with name '{}'",
                    count, &trend_store_part.name
                );
            }
        }

        Ok(())
    }
}
