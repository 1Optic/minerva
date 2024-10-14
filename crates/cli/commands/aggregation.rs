use std::time::Duration;
use std::{collections::HashMap, fmt::Display, path::{Path, PathBuf}};

use async_trait::async_trait;

use clap::{Parser, Subcommand};
use minerva::{instance::MinervaInstance, relation::Relation};
use minerva::trend_store::TrendStore;

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
            None => std::env::current_dir().unwrap()
        };

        generate_all_standard_aggregations(&instance_root);

        Ok(())
    }
}

fn generate_all_standard_aggregations(instance_root: &Path) -> Result<(), String> {
    let instance = MinervaInstance::load_from(instance_root);

    let aggregation_hints = load_aggregation_hints(instance_root)?;

    for trend_store in instance.trend_stores {
        if let Some(title) = &trend_store.title {
            if title.to_lowercase().contains("raw") {
                // For now, we determine the raw data trend stores based on the title, but this
                // should be done based on the fact that there is no materialization as source.
                generate_standard_aggregations(instance_root, trend_store, &instance.relations, &aggregation_hints);
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
enum AggregationType {
    View,
    ViewMaterialization,
    FunctionMaterialization,
}

impl Display for AggregationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::View => write!(f, "VIEW"),
            Self::ViewMaterialization => write!(f, "VIEW_MATERIALIZATION"),
            Self::FunctionMaterialization => write!(f, "FUNCTION_MATERIALIZATION"),
        }
    }
}

#[derive(Debug)]
struct AggregationHint {
    relation: String,
    aggregation_type: AggregationType,
    prefix: Option<String>,
}

impl AggregationHint {
    fn parse(relation: String, raw_hint: &str) -> Result<AggregationHint, String> {
        let mut split = raw_hint.split('+');

        let first_part = split.next().unwrap();

        let aggregation_type = match first_part {
            "VIEW" => AggregationType::View,
            "VIEW_MATERIALIZATION" => AggregationType::ViewMaterialization,
            "FUNCTION_MATERIALIZATION" => AggregationType::FunctionMaterialization,
            _ => {
                return Err(format!("Unsupported aggregation type '{first_part}'"))
            }
        };

        let prefix = split.next();

        Ok(AggregationHint {
            relation,
            aggregation_type,
            prefix: prefix.map(|s| s.to_string()),
        })
    }    
}

impl Display for AggregationHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, "{}: {}+{}", self.relation, self.aggregation_type, prefix)
        } else {
            write!(f, "{}: {}", self.relation, self.aggregation_type)
        }
    }
}

fn load_aggregation_hints(instance_root: &Path) -> Result<Vec<AggregationHint>, String> {
    let mut aggregation_hints: Vec<AggregationHint> = Vec::new();

    let path: PathBuf = [
        instance_root,
        Path::new("aggregation"),
        Path::new("aggregation_hints.yaml"),
    ].iter().collect();

    let f = std::fs::File::open(&path).map_err(|e| {
        format!(
            "Could not open aggregation hints file '{}': {}",
            path.display(),
            e
        )
    })?;

    let raw_hints: HashMap<String, String> = serde_yaml::from_reader(f).map_err(|e| {
        format!(
            "Could not read aggregation hints from file '{}': {}",
            path.display(),
            e
        )
    })?;

    for (key, value) in &raw_hints {
        aggregation_hints.push(AggregationHint::parse(key.to_string(), value)?);
    }

    Ok(aggregation_hints)
}

fn generate_standard_aggregations(instance_root: &Path, trend_store: TrendStore, relations: &[Relation], aggregation_hints: &[AggregationHint]) {
    let entity_relations: Vec<(&Relation, String)> = relations.iter().filter_map(|r| {
        let mut split = r.name.split("->");

        let source_type = split.next().unwrap();

        if let Some(target_type) = split.next() {
            if source_type == trend_store.entity_type {
                return Some((r, target_type.to_string()))
            }
        }

        None
    }).collect();

    for (relation, target_type) in entity_relations {
        generate_entity_aggregation(instance_root, &trend_store, relation, &target_type, aggregation_hints);
    }
}

fn generate_entity_aggregation(instance_root: &Path, trend_store: &TrendStore, relation: &Relation, target_entity_type: &str, aggregation_hints: &[AggregationHint]) -> Result<(), String> {
    let default_hint = AggregationHint { relation: relation.name.clone(), aggregation_type: AggregationType::FunctionMaterialization, prefix: None};
    let aggregation_hint = aggregation_hints
        .iter()
        .find(|hint| hint.relation == relation.name)
        .unwrap_or(&default_hint);

    println!("{}", relation.name);

    generate_entity_aggregation_yaml(instance_root, trend_store, target_entity_type, aggregation_hint.prefix.clone())?;

    Ok(())
}

fn generate_entity_aggregation_yaml(instance_root: &Path, trend_store: &TrendStore, target_entity_type: &str, aggregation_prefix: Option<String>) -> Result<(), String> {
    let granularity_suffix = granularity_to_suffix(trend_store.granularity)?;
    let name = match aggregation_prefix {
        Some(prefix) => format!("{}_{}_{}_{}", trend_store.data_source, prefix, target_entity_type, granularity_suffix),
        None => format!("{}_{}_{}", trend_store.data_source, target_entity_type, granularity_suffix),
    };

    let base_name = format!("{}_{}_{}", trend_store.data_source, target_entity_type, granularity_suffix);
    let file_name = format!("{}.yaml", name);
    let aggregation_file_path: PathBuf = [
        instance_root,
        Path::new("aggregation"),
        Path::new(&file_name),
    ].iter().collect();

    println!("{name}");
    Ok(())
}

const GRAN_15M: Duration = Duration::from_secs(900);

fn granularity_to_suffix(granularity: Duration) -> Result<String, String> {
    match granularity {
        GRAN_15M => Ok("15m".to_string()),
        _ => Err(format!("No predefined granularity '{:?}'", granularity))
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct AggregationCompileAll {
}

#[async_trait]
impl Cmd for AggregationCompileAll {
    async fn run(&self) -> CmdResult {

        Ok(())
    }
}
