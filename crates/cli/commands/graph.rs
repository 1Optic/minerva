use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::instance::MinervaInstance;
use petgraph::{prelude::GraphMap, visit::{DfsPostOrder, Walker}};
use petgraph::data::ElementIterator;

use super::common::{connect_to_db, get_db_config, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct GraphOpt {
    #[arg(long = "from-dir", help = "load Minerva instance from directory")]
    from_dir: Option<PathBuf>,
    #[arg(long, help = "focus on specific node")]
    focus_on: Option<String>,
    #[arg(long)]
    dependency_order: bool,
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

        match &self.focus_on {
            None => {
                let graph = instance.dependency_graph();
                let dot =
                    petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);

                println!("{}", dot);
            }
            Some(node_ref) => {
                let graph = instance.dependency_graph();

                let index = graph.node_indices().find(|i| graph[*i].to_string().eq(node_ref)).unwrap();

                let mut dfs = DfsPostOrder::new(&graph, index);

                let graph_map = GraphMap::from_iter(dfs.iter(&graph));

                let dot = petgraph::dot::Dot::with_config(
                    &graph_map,
                    &[petgraph::dot::Config::EdgeNoLabel],
                );

                println!("{}", dot);
            }
        };

        Ok(())
    }
}
