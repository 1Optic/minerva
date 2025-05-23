use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::ExitCode;

pub mod commands;
pub mod interact;

use crate::commands::aggregation::AggregationOpt;
use crate::commands::attributestore::AttributeStoreOpt;
use crate::commands::common::Cmd;
use crate::commands::define::DefineOpt;
use crate::commands::diff::DiffOpt;
use crate::commands::dump::DumpOpt;
use crate::commands::graph::GraphOpt;
use crate::commands::initialize::InitializeOpt;
use crate::commands::loaddata::LoadDataOpt;
use crate::commands::relation::RelationOpt;
use crate::commands::schema::SchemaOpt;
use crate::commands::start::StartOpt;
use crate::commands::trendmaterialization::TrendMaterializationOpt;
use crate::commands::trendstore::TrendStoreOpt;
use crate::commands::trigger::TriggerOpt;
use crate::commands::update::UpdateOpt;
use crate::commands::virtualentity::VirtualEntityOpt;

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, name = "minerva", arg_required_else_help = true)]
struct Cli {
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand, PartialEq)]
enum Commands {
    #[command(about = "Show the definition used for initializing a new Minerva database")]
    Schema(SchemaOpt),
    #[command(about = "Complete dump of a Minerva instance")]
    Dump(DumpOpt),
    #[command(about = "Create a diff between Minerva instance definitions")]
    Diff(DiffOpt),
    #[command(about = "Create a graph of a Minerva instance")]
    Graph(GraphOpt),
    #[command(about = "Update a Minerva database from an instance definition")]
    Update(UpdateOpt),
    #[command(about = "Initialize a complete Minerva instance")]
    Initialize(InitializeOpt),
    #[command(about = "Manage trend stores")]
    TrendStore(TrendStoreOpt),
    #[command(about = "Manage triggers")]
    Trigger(TriggerOpt),
    #[command(about = "Manage attribute stores")]
    AttributeStore(AttributeStoreOpt),
    #[command(about = "Manage trend materrializations")]
    TrendMaterialization(TrendMaterializationOpt),
    #[command(about = "Load data into Minerva database")]
    LoadData(LoadDataOpt),
    #[command(about = "Manage relations")]
    Relation(RelationOpt),
    #[command(about = "Start Minerva instance")]
    Start(StartOpt),
    #[command(about = "Generate standard aggregations")]
    Aggregation(AggregationOpt),
    #[command(about = "Manage virtual entities")]
    VirtualEntity(VirtualEntityOpt),
    #[command(about = "Define Minerva instance")]
    Define(DefineOpt),
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() -> ExitCode {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    let cli = Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();

        print_completions(generator, &mut cmd);
    }

    let result = match cli.command {
        Some(Commands::Schema(schema)) => schema.run().await,
        Some(Commands::Dump(dump)) => dump.run().await,
        Some(Commands::Diff(diff)) => diff.run().await,
        Some(Commands::Graph(graph)) => graph.run().await,
        Some(Commands::Update(update)) => update.run().await,
        Some(Commands::Initialize(initialize)) => initialize.run().await,
        Some(Commands::TrendStore(trend_store)) => trend_store.run().await,
        Some(Commands::Trigger(trigger)) => trigger.run().await,
        Some(Commands::AttributeStore(attribute_store)) => attribute_store.run().await,
        Some(Commands::TrendMaterialization(trend_materialization)) => {
            trend_materialization.run().await
        }
        Some(Commands::LoadData(load_data)) => load_data.run().await,
        Some(Commands::Relation(relation)) => relation.run().await,
        Some(Commands::Start(start)) => start.run().await,
        Some(Commands::Aggregation(aggregation)) => aggregation.run().await,
        Some(Commands::VirtualEntity(virtual_entity)) => virtual_entity.run().await,
        Some(Commands::Define(define)) => define.run().await,
        None => return ExitCode::FAILURE,
    };

    if let Err(e) = result {
        println!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
