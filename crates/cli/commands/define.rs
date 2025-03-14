use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use serde::{Deserialize, Serialize};

use super::common::{
    Cmd, CmdResult,
};

#[derive(Debug, Parser, PartialEq)]
pub struct DefineOpt {
    #[arg(
        short = 'i',
        long = "instance-root",
        help = "compare with other Minerva instance directory"
    )]
    instance_root: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct TrendDefinition {
    name: String,
}

#[async_trait]
impl Cmd for DefineOpt {
    async fn run(&self) -> CmdResult {
        let trend_definitions: Vec<TrendDefinition> = serde_json::from_reader(std::io::stdin()).unwrap();

        for trend_definition in trend_definitions {
            println!("- {}", trend_definition.name);
        }
        Ok(())
    }
}
