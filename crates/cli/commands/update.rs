use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::graph::dependee_graph;
use minerva::graph::node_index_by_name;
use minerva::graph::render_graph_with_changes;
use minerva::graph::GraphNode;
use minerva::instance::load_instance_config;
use minerva::instance::DiffOptions;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use tokio_postgres::Client;

use minerva::error::{ConfigurationError, Error};
use minerva::instance::MinervaInstance;

use super::common::{connect_db, Cmd, CmdResult, ENV_MINERVA_INSTANCE_ROOT};
use crate::interact::interact;

#[derive(Debug, Parser, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct UpdateOpt {
    #[arg(short, long)]
    non_interactive: bool,
    #[arg(help = "Minerva instance root directory")]
    instance_root: Option<PathBuf>,
    #[arg(long)]
    ignore_trend_extra_data: bool,
    #[arg(long)]
    ignore_trend_data_type: bool,
    #[arg(long)]
    ignore_deletions: bool,
    #[arg(long, help = "Only generate a plan for the update steps and order")]
    plan_only: Option<String>,
}

#[async_trait]
impl Cmd for UpdateOpt {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        //print!("Reading Minerva instance from database... ");
        io::stdout().flush().unwrap();
        let instance_db = MinervaInstance::load_from_db(&mut client).await?;
        //println!("Ok");

        let minerva_instance_root = match &self.instance_root {
            Some(root) => {
                // Next to passing on the Minerva instance root directory, we need to set the
                // environment variable for any child processes that might be started during
                // initialization.
                std::env::set_var(ENV_MINERVA_INSTANCE_ROOT, root);

                root.clone()
            }
            None => match env::var(ENV_MINERVA_INSTANCE_ROOT) {
                Ok(v) => PathBuf::from(v),
                Err(e) => {
                    return Err(Error::Configuration(ConfigurationError {
                        msg: format!(
                            "Environment variable '{}' could not be read: {}",
                            &ENV_MINERVA_INSTANCE_ROOT, e
                        ),
                    }));
                }
            },
        };

        //print!(
        //    "Reading Minerva instance from '{}'... ",
        //    &minerva_instance_root.to_string_lossy()
        //);
        //io::stdout().flush().unwrap();
        let instance_def = MinervaInstance::load_from(&minerva_instance_root)?;
        let instance_config = load_instance_config(&minerva_instance_root).map_err(|e| {
            minerva::error::ConfigurationError::from_msg(format!(
                "Could not load instance config: {e}"
            ))
        })?;
        //println!("Ok");

        let diff_options = DiffOptions {
            ignore_trend_extra_data: self.ignore_trend_extra_data,
            ignore_trend_data_type: self.ignore_trend_data_type,
            ignore_deletions: self.ignore_deletions,
            instance_ignores: instance_config.deployment.ignore,
        };

        let update_plan = plan_update(&instance_db, &instance_def, diff_options);

        if let Some(plan_output_format) = &self.plan_only {
            if plan_output_format.eq("plain") {
                for (index, change) in update_plan.changes.iter().enumerate() {
                    println!("{} {}", index + 1, change);
                }
            } else if plan_output_format.eq("dot") {
                println!("{}", update_plan.render_dot());
            }

            Ok(())
        } else {
            update(&mut client, update_plan, !self.non_interactive).await
        }
    }
}

struct UpdatePlan {
    dependency_graph: Graph<GraphNode, String>,
    changes: Vec<Box<dyn Change + std::marker::Send>>,
}

impl UpdatePlan {
    pub fn render_dot(&self) -> String {
        render_graph_with_changes(&self.dependency_graph, &self.changes)
    }
}

fn plan_update(
    db_instance: &MinervaInstance,
    other: &MinervaInstance,
    diff_options: DiffOptions,
) -> UpdatePlan {
    let mut planned_changes: Vec<Box<dyn Change + std::marker::Send>> = Vec::new();
    let changes = db_instance.diff(other, diff_options);

    // Split the changes between changes on existing objects and changes for new objects
    let (mut changes_to_existing_objects, changes_to_new_objects): (Vec<_>, Vec<_>) = changes
        .into_iter()
        .partition(|c| c.existing_object().is_some());

    let db_instance_graph = db_instance.dependency_graph();

    // First find all 'root' nodes of sub-dependency graphs
    let root_nodes: Vec<NodeIndex> = db_instance_graph
        .node_indices()
        .filter(|n| {
            db_instance_graph
                .edges_directed(*n, petgraph::Direction::Outgoing)
                .count()
                == 0
        })
        .collect();

    // First take the changes of existing elements

    for node_index in root_nodes {
        let node = db_instance_graph.node_weight(node_index).unwrap();
        let mut dep_graph = dependee_graph(&db_instance_graph, node_index);

        // Only traverse if the graph has more than 1 node
        if dep_graph.raw_nodes().len() > 1 {
            let dep_graph_root_node_index = node_index_by_name(&dep_graph, node.name()).unwrap();
            dep_graph.reverse();
            let mut dfs = Dfs::new(&dep_graph, dep_graph_root_node_index);

            while let Some(nx) = dfs.next(&dep_graph) {
                let w = dep_graph.node_weight(nx).unwrap();

                let matching_changes = changes_to_existing_objects.extract_if(.., |change| {
                    let o = change.existing_object().unwrap();

                    o.to_string() == w.to_string()
                });

                planned_changes.extend(matching_changes);
            }
        } else {
            // Check root node itself
        }
    }

    if !changes_to_existing_objects.is_empty() {
        for c in &changes_to_existing_objects {
            println!("Change to existing left: {c}");
        }
    }

    planned_changes.extend(changes_to_new_objects);

    UpdatePlan {
        dependency_graph: db_instance_graph,
        changes: planned_changes,
    }
}

async fn update(client: &mut Client, plan: UpdatePlan, interactive: bool) -> CmdResult {
    println!("Applying changes:");

    let num_changes = plan.changes.len();

    for (index, change) in plan.changes.iter().enumerate() {
        println!("\n\n* [{}/{num_changes}] {change}", index + 1);

        if !interactive || interact(client, change.as_ref()).await? {
            match change.apply(client).await {
                Ok(message) => {
                    println!("> {}", &message);
                }
                Err(err) => {
                    println!("! Error applying change: {}", &err);
                }
            }
        }
    }

    Ok(())
}
