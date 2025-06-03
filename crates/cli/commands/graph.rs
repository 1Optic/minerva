use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use clap::Parser;

use minerva::instance::{GraphNode, MinervaInstance};
use petgraph::{graph::NodeIndex, visit::DfsEvent, Graph};

use super::common::{connect_to_db, get_db_config, Cmd, CmdResult};

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
        env_logger::init();

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
            dependency_graph(&full_graph, start.clone())
        } else if let Some(start) = &self.dependees {
            dependee_graph(&full_graph, start.clone())
        } else {
            full_graph
        };

        let dot = petgraph::dot::Dot::with_attr_getters(
            &graph,
            &[petgraph::dot::Config::EdgeNoLabel],
            &|_graph, _edge_ref| "".to_string(),
            &|_graph, (_index, node)| match node {
                GraphNode::Table(_) => "shape=box".to_string(),
                GraphNode::TrendStorePart(_) => "shape=box".to_string(),
                GraphNode::TrendFunctionMaterialization(_) => {
                    "shape=box,style=\"rounded\"".to_string()
                }
                _ => "".to_string(),
            },
        );

        println!("{}", dot);

        Ok(())
    }
}

fn dependency_graph(graph: &Graph<GraphNode, String>, start: String) -> Graph<GraphNode, String> {
    let start_index = graph.node_indices().find(|index| {
        let node = graph.node_weight(*index);

        match node {
            Some(GraphNode::TrendStorePart(trend_store_part)) => trend_store_part.eq(&start),
            Some(GraphNode::AttributeStore(attribute_store)) => attribute_store.eq(&start),
            Some(GraphNode::Relation(relation)) => relation.eq(&start),
            _ => false,
        }
    });

    let mut subgraph: petgraph::Graph<GraphNode, String> = petgraph::Graph::new();
    let mut node_set: HashMap<GraphNode, NodeIndex> = HashMap::new();

    petgraph::visit::depth_first_search(&graph, start_index, |event| match event {
        DfsEvent::CrossForwardEdge(parent, child)
        | DfsEvent::BackEdge(parent, child)
        | DfsEvent::TreeEdge(parent, child) => {
            let p = graph.node_weight(parent).unwrap();

            let pi = match node_set.get(p) {
                None => {
                    let i = subgraph.add_node(p.clone());
                    node_set.insert(p.clone(), i);
                    i
                }
                Some(index) => *index,
            };

            let c = graph.node_weight(child).unwrap();

            let ci = match node_set.get(c) {
                None => {
                    let i = subgraph.add_node(c.clone());
                    node_set.insert(c.clone(), i);
                    i
                }
                Some(index) => *index,
            };

            subgraph.add_edge(pi, ci, "".to_string());
        }
        DfsEvent::Discover(_, _) | DfsEvent::Finish(_, _) => {}
    });

    subgraph
}

fn dependee_graph(graph: &Graph<GraphNode, String>, start: String) -> Graph<GraphNode, String> {
    let mut reversed = graph.clone();
    reversed.reverse();
    let mut result = dependency_graph(&reversed, start);
    result.reverse();
    result
}
