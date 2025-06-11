use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use minerva::change::Change;
use minerva::graph::dependee_graph;
use minerva::graph::node_index_by_name;
use minerva::instance::DiffOptions;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
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
    plan_only: bool,
}

#[async_trait]
impl Cmd for UpdateOpt {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        print!("Reading Minerva instance from database... ");
        io::stdout().flush().unwrap();
        let instance_db = MinervaInstance::load_from_db(&mut client).await?;
        println!("Ok");

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

        print!(
            "Reading Minerva instance from '{}'... ",
            &minerva_instance_root.to_string_lossy()
        );
        io::stdout().flush().unwrap();
        let instance_def = MinervaInstance::load_from(&minerva_instance_root)?;
        println!("Ok");

        let diff_options = DiffOptions {
            ignore_trend_extra_data: self.ignore_trend_extra_data,
            ignore_trend_data_type: self.ignore_trend_data_type,
            ignore_deletions: self.ignore_deletions,
        };

        if self.plan_only {
            println!("Planning update");
            let planned_changes = plan_update(&instance_db, &instance_def, diff_options);

            for (index, change) in planned_changes.iter().enumerate() {
                println!("{} {}", index, change);
            }

            Ok(())
        } else {
            update(
                &mut client,
                &instance_db,
                &instance_def,
                !self.non_interactive,
                diff_options,
            )
            .await
        }
    }
}

fn plan_update(db_instance: &MinervaInstance, other: &MinervaInstance, diff_options: DiffOptions) -> Vec<Box<dyn Change + std::marker::Send>> {
    let mut planned_changes: Vec<Box<dyn Change + std::marker::Send>> = Vec::new();
    let changes = db_instance.diff(other, diff_options);

    // Split the changes between changes on existing objects and changes for new objects
    let (mut changes_to_existing_objects, changes_to_new_objects): (Vec<_>, Vec<_>) =
        changes.into_iter().partition(|c| c.existing_object().is_some());

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
        let start_index = node_index_by_name(&db_instance_graph, node.name()).unwrap();
        let g = dependee_graph(&db_instance_graph, start_index);

        // Only traverse if the graph has more than 1 node
        if g.raw_nodes().len() > 1 {
            let mut dfs = Dfs::new(&g, node_index);

            while let Some(nx) = dfs.next(&g) {
                let w = g.node_weight(nx).unwrap();

                let matching_changes = changes_to_existing_objects.extract_if(.., |change| {
                    let o = change.existing_object().unwrap();

                    o.to_string() == w.to_string()
                });

                planned_changes.extend(matching_changes);
            }
        }
    }

    if !changes_to_existing_objects.is_empty() {
        for c in &changes_to_existing_objects {
            println!("Change to existing: {}", c);
        }
    }

    planned_changes.extend(changes_to_new_objects);

    planned_changes
}

async fn update(
    client: &mut Client,
    db_instance: &MinervaInstance,
    other: &MinervaInstance,
    interactive: bool,
    diff_options: DiffOptions,
) -> CmdResult {
    let changes = db_instance.diff(other, diff_options);

    println!("Applying changes:");

    let num_changes = changes.len();

    for (index, change) in changes.iter().enumerate() {
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
