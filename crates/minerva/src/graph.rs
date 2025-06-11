use std::collections::HashMap;
use std::fmt::Display;

use petgraph::{graph::NodeIndex, visit::DfsEvent, Graph};

use crate::change::Change;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum GraphNode {
    Table(String),
    TrendStorePart(String),
    AttributeStore(String),
    AttributeMaterialization(String),
    TrendViewMaterialization(String),
    TrendFunctionMaterialization(String),
    Relation(String),
    VirtualEntity(String),
}

impl GraphNode {
    pub fn matches_ref(&self, node_ref: &str) -> bool {
        self.to_string().eq(node_ref)
    }

    pub fn name(&self) -> &str {
        match self {
            GraphNode::Table(name) => name,
            GraphNode::TrendStorePart(name) => name,
            GraphNode::AttributeStore(name) => name,
            GraphNode::AttributeMaterialization(name) => name,
            GraphNode::TrendViewMaterialization(name) => name,
            GraphNode::TrendFunctionMaterialization(name) => name,
            GraphNode::Relation(name) => name,
            GraphNode::VirtualEntity(name) => name,
        }
    }
}

impl Display for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphNode::Table(name) => {
                write!(f, "Table({})", name)
            }
            GraphNode::TrendStorePart(name) => {
                write!(f, "TrendStorePart({})", name)
            }
            GraphNode::AttributeStore(name) => {
                write!(f, "AttributeStore({})", name)
            }
            GraphNode::AttributeMaterialization(name) => {
                write!(f, "AttributeMaterialization({})", name)
            }
            GraphNode::TrendViewMaterialization(name) => {
                write!(f, "TrendViewMaterialization({})", name)
            }
            GraphNode::TrendFunctionMaterialization(name) => {
                write!(f, "TrendFunctionMaterialization({})", name)
            }
            GraphNode::Relation(name) => {
                write!(f, "Relation({})", name)
            }
            GraphNode::VirtualEntity(name) => {
                write!(f, "VirtualEntity({})", name)
            }
        }
    }
}

pub fn node_index_by_name(graph: &Graph<GraphNode, String>, name: &str) -> Option<NodeIndex> {
    graph.node_indices().find(|index| {
        let node = graph.node_weight(*index);

        match node {
            Some(node) => node.name().eq(name),
            _ => false,
        }
    })
}

pub fn dependency_graph(
    graph: &Graph<GraphNode, String>,
    start_index: NodeIndex,
) -> Graph<GraphNode, String> {
    let mut subgraph: petgraph::Graph<GraphNode, String> = petgraph::Graph::new();
    let mut node_set: HashMap<GraphNode, NodeIndex> = HashMap::new();

    petgraph::visit::depth_first_search(&graph, Some(start_index), |event| match event {
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

pub fn dependee_graph(
    graph: &Graph<GraphNode, String>,
    start_index: NodeIndex,
) -> Graph<GraphNode, String> {
    let mut reversed = graph.clone();
    reversed.reverse();
    let mut result = dependency_graph(&reversed, start_index);
    result.reverse();
    result
}

pub fn render_graph(graph: &Graph<GraphNode, String>) -> String {
    petgraph::dot::Dot::with_attr_getters(
        graph,
        &[petgraph::dot::Config::EdgeNoLabel],
        &|_graph, _edge_ref| "".to_string(),
        &|_graph, (_index, node)| match node {
            GraphNode::Table(_) => "shape=box".to_string(),
            GraphNode::VirtualEntity(_) => "shape=box,style=\"rounded\"".to_string(),
            GraphNode::Relation(_) => "shape=box,style=\"rounded\"".to_string(),
            GraphNode::TrendStorePart(_) => "shape=box".to_string(),
            GraphNode::TrendFunctionMaterialization(_) => "shape=box,style=\"rounded\"".to_string(),
            GraphNode::TrendViewMaterialization(_) => "shape=box,style=\"rounded\"".to_string(),
            GraphNode::AttributeStore(_) => "shape=box".to_string(),
            GraphNode::AttributeMaterialization(_) => "shape=box,style=\"rounded\"".to_string(),
        },
    )
    .to_string()
}

pub fn render_graph_with_changes(
    graph: &Graph<GraphNode, String>,
    changes: &[Box<dyn Change + Send>],
) -> String {
    petgraph::dot::Dot::with_attr_getters(
        graph,
        &[petgraph::dot::Config::EdgeNoLabel],
        &|_graph, _edge_ref| "".to_string(),
        &|_graph, (_index, node)| {
            let val = changes.iter().enumerate().find(|(_index, change)| {
                change
                    .existing_object()
                    .is_some_and(|o| o.to_string() == node.to_string())
            });

            let attr = match node {
                GraphNode::Table(_) => "shape=box".to_string(),
                GraphNode::VirtualEntity(_) => "shape=box,style=\"rounded\"".to_string(),
                GraphNode::Relation(_) => "shape=box,style=\"rounded\"".to_string(),
                GraphNode::TrendStorePart(_) => "shape=box".to_string(),
                GraphNode::TrendFunctionMaterialization(_) => {
                    "shape=box,style=\"rounded\"".to_string()
                }
                GraphNode::TrendViewMaterialization(_) => "shape=box,style=\"rounded\"".to_string(),
                GraphNode::AttributeStore(_) => "shape=box".to_string(),
                GraphNode::AttributeMaterialization(_) => "shape=box,style=\"rounded\"".to_string(),
            };

            if let Some((change_index, _change)) = val {
                format!(
                    "{},xlabel={},style=filled,fillcolor=lightblue",
                    attr,
                    change_index + 1
                )
            } else {
                attr
            }
        },
    )
    .to_string()
}
