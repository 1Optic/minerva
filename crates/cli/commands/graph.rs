use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use super::common::{connect_to_db, get_db_config, Cmd, CmdResult};
use minerva::{
    graph::{dependee_graph, dependency_graph, node_index_by_name, render_graph},
    instance::MinervaInstance,
};

#[derive(Debug, Parser, PartialEq)]
pub struct GraphOpt {
    #[arg(long = "from-dir", help = "load Minerva instance from directory")]
    from_dir: Option<PathBuf>,
    #[arg(long)]
    dependency_order: bool,
    #[arg(long)]
    dependencies: Option<String>,
    #[arg(long)]
    dependees: Option<String>,
}

#[async_trait]
impl Cmd for GraphOpt {
    async fn run(&self) -> CmdResult {
        let instance = match &self.from_dir {
            Some(with_dir) => MinervaInstance::load_from(with_dir)?,
            None => {
                let db_config = get_db_config()?;

                let mut client = connect_to_db(&db_config).await?;

                MinervaInstance::load_from_db(&mut client).await?
            }
        };

        let full_graph = instance.dependency_graph();

        let graph = if let Some(start) = &self.dependencies {
            let start_index = node_index_by_name(&full_graph, start).unwrap();
            dependency_graph(&full_graph, start_index)
        } else if let Some(start) = &self.dependees {
            let start_index = node_index_by_name(&full_graph, start).unwrap();
            dependee_graph(&full_graph, start_index)
        } else {
            full_graph
        };

        let dot = render_graph(&graph);

        println!("{}", dot);

        Ok(())
    }
}
