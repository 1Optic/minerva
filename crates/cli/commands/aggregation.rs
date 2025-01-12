use async_trait::async_trait;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use minerva::{
    aggregation_generation::generate_all_standard_aggregations, instance::load_instance_config,
};

use super::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AggregationOpt {
    #[command(subcommand)]
    command: AggregationOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum AggregationOptCommands {
    #[command(about = "generate standard aggregations")]
    Generate(AggregationGenerate),
    #[command(about = "compile all default aggregations")]
    CompileAll(AggregationCompileAll),
}

impl AggregationOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            AggregationOptCommands::Generate(generate) => generate.run().await,
            AggregationOptCommands::CompileAll(compile_all) => compile_all.run().await,
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct AggregationGenerate {
    #[arg(short, long, help = "Minerva instance root directory")]
    instance_root: Option<PathBuf>,
}

#[async_trait]
impl Cmd for AggregationGenerate {
    async fn run(&self) -> CmdResult {
        let instance_root = match &self.instance_root {
            Some(path) => path.clone(),
            None => std::env::current_dir().unwrap(),
        };

        let instance_config = load_instance_config(&instance_root)
            .map_err(|e| format!("Could not load instance config: {e}"))?;

        generate_all_standard_aggregations(&instance_root, instance_config).map_err(|e| {
            minerva::error::Error::Runtime(minerva::error::RuntimeError::from_msg(e.to_string()))
        })
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct AggregationCompileAll {}

#[async_trait]
impl Cmd for AggregationCompileAll {
    async fn run(&self) -> CmdResult {
        Ok(())
    }
}
