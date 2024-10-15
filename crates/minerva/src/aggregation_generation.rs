use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::OnceLock;
use std::time::Duration;
use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::meas_value::DataType;
use crate::trend_materialization::{
    TrendFunctionMaterialization, TrendMaterializationFunction, TrendMaterializationSource,
};
use crate::trend_store::{Trend, TrendStore, TrendStorePart};
use crate::{instance::MinervaInstance, relation::Relation};

#[derive(Error, Debug)]
pub enum AggregationGenerationError {
    #[error("runtime issue: {0}")]
    Runtime(String),
    #[error("could not load hints: {0}")]
    HintLoading(String),
    #[error("could not generate entity aggregation: {0}")]
    EntityAggregation(String),
    #[error("could not generate time aggregation: {0}")]
    TimeAggregation(String),
}

pub fn generate_all_standard_aggregations(instance_root: &Path) -> Result<(), AggregationGenerationError> {
    let instance = MinervaInstance::load_from(instance_root);

    let aggregation_hints = load_aggregation_hints(instance_root)?;

    for trend_store in &instance.trend_stores {
        if let Some(title) = &trend_store.title {
            if title.to_lowercase().contains("raw") {
                // For now, we determine the raw data trend stores based on the title, but this
                // should be done based on the fact that there is no materialization as source.
                generate_standard_aggregations(
                    &instance,
                    trend_store.clone(),
                    &instance.relations,
                    &aggregation_hints,
                )?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
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
            _ => return Err(format!("Unsupported aggregation type '{first_part}'")),
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

fn load_aggregation_hints(instance_root: &Path) -> Result<Vec<AggregationHint>, AggregationGenerationError> {
    let mut aggregation_hints: Vec<AggregationHint> = Vec::new();

    let path: PathBuf = [
        instance_root,
        Path::new("aggregation"),
        Path::new("aggregation_hints.yaml"),
    ]
    .iter()
    .collect();

    let f = std::fs::File::open(&path).map_err(|e| {
        AggregationGenerationError::HintLoading(format!(
            "Could not open aggregation hints file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let raw_hints: HashMap<String, String> = serde_yaml::from_reader(f).map_err(|e| {
        AggregationGenerationError::HintLoading(format!(
            "Could not read aggregation hints from file '{}': {}",
            path.display(),
            e
        ))
    })?;

    for (key, value) in &raw_hints {
        aggregation_hints.push(AggregationHint::parse(key.to_string(), value).map_err(|e| AggregationGenerationError::HintLoading(e))?);
    }

    Ok(aggregation_hints)
}

fn generate_standard_aggregations(
    minerva_instance: &MinervaInstance,
    trend_store: TrendStore,
    relations: &[Relation],
    aggregation_hints: &[AggregationHint],
) -> Result<(), AggregationGenerationError> {
    let entity_relations: Vec<(&Relation, String)> = relations
        .iter()
        .filter_map(|r| {
            // Currently only by convention, the relation name describes source and target entity
            // types '<SOURCE_TYPE>-><TARGET_TYPE>' and we try to extract these here.
            let mut split = r.name.split("->");

            let source_type = split.next().unwrap();

            if let Some(target_type) = split.next() {
                if source_type == trend_store.entity_type {
                    // Only generate an aggregation when the source type matches the entity type of
                    // the trend store.
                    return Some((r, target_type.to_string()));
                }
            }

            None
        })
        .collect();

    for (relation, target_type) in &entity_relations {
        build_entity_aggregation(
            minerva_instance,
            &trend_store,
            relation,
            target_type,
            aggregation_hints,
        ).map_err(AggregationGenerationError::EntityAggregation)?;
    }

    static STANDARD_AGGREGATIONS: OnceLock<HashMap<Duration, Vec<(Duration, Duration)>>> =
        OnceLock::new();

    let standard_aggregations = STANDARD_AGGREGATIONS.get_or_init(|| {
        vec![
            (
                Duration::from_secs(900),
                vec![
                    (Duration::from_secs(900), Duration::from_secs(3600)),
                    (Duration::from_secs(900), Duration::from_secs(86400)),
                    (
                        Duration::from_secs(86400),
                        humantime::parse_duration("1w").unwrap(),
                    ),
                    (
                        Duration::from_secs(86400),
                        humantime::parse_duration("1month").unwrap(),
                    ),
                ],
            ),
            (
                Duration::from_secs(86400),
                vec![
                    (
                        Duration::from_secs(86400),
                        humantime::parse_duration("1w").unwrap(),
                    ),
                    (
                        Duration::from_secs(86400),
                        humantime::parse_duration("1month").unwrap(),
                    ),
                ],
            ),
        ]
        .into_iter()
        .collect()
    });

    let aggregations = &standard_aggregations[&trend_store.granularity];

    for (source_granularity, target_granularity) in aggregations {
        let target_trend_store = build_time_aggregation(
            minerva_instance,
            &trend_store,
            source_granularity,
            target_granularity,
        ).map_err(AggregationGenerationError::TimeAggregation)?;

        for (relation, target_type) in &entity_relations {
            build_entity_aggregation(
                minerva_instance,
                &target_trend_store,
                relation,
                target_type,
                aggregation_hints,
            ).map_err(AggregationGenerationError::EntityAggregation)?;
        }
    }

    Ok(())
}

fn build_time_aggregation(
    minerva_instance: &MinervaInstance,
    trend_store: &TrendStore,
    source_granularity: &Duration,
    target_granularity: &Duration,
) -> Result<TrendStore, String> {
    let time_aggregation =
        generate_time_aggregation(trend_store, source_granularity, target_granularity)?;

    compile_time_aggregation(minerva_instance, &time_aggregation)
}

fn compile_time_aggregation(
    minerva_instance: &MinervaInstance,
    aggregation: &TimeAggregation,
) -> Result<TrendStore, String> {
    let target_trend_store = write_time_aggregations(minerva_instance, aggregation)?;

    let trend_store_file_name = format!("{}.yaml", trend_store_name(&target_trend_store)?);

    let trend_store_file_path: PathBuf = [
        minerva_instance.instance_root.clone().unwrap(),
        PathBuf::from("trend"),
        trend_store_file_name.into(),
    ]
    .iter()
    .collect();

    let file = File::create(trend_store_file_path).unwrap();

    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, &target_trend_store).unwrap();

    Ok(target_trend_store)
}

fn trend_store_name(trend_store: &TrendStore) -> Result<String, String> {
    Ok(format!(
        "{}_{}_{}",
        trend_store.data_source,
        trend_store.entity_type,
        granularity_to_suffix(&trend_store.granularity)?
    ))
}

fn write_time_aggregations(
    minerva_instance: &MinervaInstance,
    aggregation: &TimeAggregation,
) -> Result<TrendStore, String> {
    let mut target_trend_store_parts: Vec<TrendStorePart> = Vec::new();

    let trend_store = minerva_instance
        .trend_stores
        .iter()
        .find(|trend_store| {
            aggregation
                .source
                .eq(&trend_store_name(trend_store).unwrap())
        })
        .unwrap();

    for agg_part in &aggregation.parts {
        let source_part = trend_store
            .parts
            .iter()
            .find(|part| part.name == agg_part.source)
            .ok_or(format!(
                "No source definition found for aggregation part '{}'(source: '{}')",
                agg_part.name, agg_part.source
            ))?;

        let materialization_file_path: PathBuf = [
            minerva_instance.instance_root.clone().unwrap(),
            PathBuf::from("materialization"),
            PathBuf::from(format!("{}.yaml", agg_part.name)),
        ]
        .iter()
        .collect();

        let (aggregation, target_trend_store_part) = define_part_time_aggregation(
            source_part,
            &trend_store.granularity,
            aggregation.mapping_function.clone(),
            &aggregation.granularity,
            agg_part.name.clone(),
        )?;

        target_trend_store_parts.push(target_trend_store_part);

        println!(
            "Writing time materialization to '{}'",
            materialization_file_path.to_string_lossy()
        );

        let file = File::create(materialization_file_path).unwrap();

        let writer = BufWriter::new(file);

        serde_yaml::to_writer(writer, &aggregation).unwrap();
    }

    static PARTITION_SIZE_MAPPING: OnceLock<HashMap<Duration, Duration>> = OnceLock::new();

    let partition_size_mapping: &HashMap<Duration, Duration> =
        PARTITION_SIZE_MAPPING.get_or_init(|| {
            vec![
                (Duration::from_secs(900), Duration::from_secs(86400)),
                (Duration::from_secs(1800), Duration::from_secs(86400 * 2)),
                (Duration::from_secs(3600), Duration::from_secs(86400 * 4)),
                (
                    Duration::from_secs(86400),
                    humantime::parse_duration("3month").unwrap(),
                ),
                (
                    humantime::parse_duration("1w").unwrap(),
                    humantime::parse_duration("1y").unwrap(),
                ),
                (
                    humantime::parse_duration("1month").unwrap(),
                    humantime::parse_duration("5y").unwrap(),
                ),
            ]
            .into_iter()
            .collect()
        });

    Ok(TrendStore {
        title: Some("Generated by Minerva aggregation generation command".to_string()),
        data_source: trend_store.data_source.clone(),
        entity_type: trend_store.entity_type.clone(),
        granularity: aggregation.granularity,
        partition_size: *partition_size_mapping
            .get(&aggregation.granularity)
            .unwrap(),
        parts: target_trend_store_parts,
    })
}

fn define_part_time_aggregation(
    source_part: &TrendStorePart,
    source_granularity: &Duration,
    mapping_function: String,
    target_granularity: &Duration,
    name: String,
) -> Result<(TrendFunctionMaterialization, TrendStorePart), String> {
    let materialization = TrendFunctionMaterialization {
        target_trend_store_part: name.clone(),
        enabled: true,
        processing_delay: Duration::from_secs(1800),
        stability_delay: Duration::from_secs(300),
        reprocessing_period: Duration::from_secs(86400 * 7),
        sources: vec![TrendMaterializationSource {
            trend_store_part: source_part.name.clone(),
            mapping_function,
        }],
        function: time_aggregate_function(source_part, target_granularity)?,
        fingerprint_function: define_time_fingerprint_sql(
            source_part,
            source_granularity,
            target_granularity,
        ),
        description: None,
    };

    let mut aggregate_trends: Vec<Trend> = source_part
        .trends
        .iter()
        .map(define_time_aggregate_trend)
        .collect();

    if !aggregate_trends
        .iter()
        .any(|trend| trend.name.eq("samples"))
    {
        aggregate_trends.push(Trend {
            name: "samples".to_string(),
            data_type: DataType::Integer,
            description: "Number of source records".to_string(),
            time_aggregation: "sum".to_string(),
            entity_aggregation: "sum".to_string(),
            extra_data: serde_json::Value::Object(serde_json::Map::new()),
        });
    }

    let target_trend_store_part = TrendStorePart {
        name,
        trends: aggregate_trends,
        generated_trends: vec![],
    };

    Ok((materialization, target_trend_store_part))
}

fn define_time_aggregate_trend(trend: &Trend) -> Trend {
    Trend {
        name: trend.name.clone(),
        data_type: aggregate_data_type(trend.data_type, &trend.time_aggregation),
        time_aggregation: trend.time_aggregation.clone(),
        entity_aggregation: trend.entity_aggregation.clone(),
        description: trend.description.clone(),
        extra_data: trend.extra_data.clone(),
    }
}

fn define_time_fingerprint_sql(
    source_part: &TrendStorePart,
    source_granularity: &Duration,
    target_granularity: &Duration,
) -> String {
    let source_granularity_suffix = granularity_to_suffix(source_granularity).unwrap();
    let target_granularity_suffix = granularity_to_suffix(target_granularity).unwrap();
    [
        "SELECT max(modified.last), format('{%s}', string_agg(format('\"%s\":\"%s\"', t, modified.last), ','))::jsonb\n".to_string(),
        format!("FROM generate_series($1 - interval '{target_granularity_suffix}' + interval '{source_granularity_suffix}', $1, interval '{source_granularity_suffix}') t\n"),
        "LEFT JOIN (\n".to_string(),
        "  SELECT timestamp, last\n".to_string(),
        "  FROM trend_directory.trend_store_part part\n".to_string(),
        "  JOIN trend_directory.modified ON modified.trend_store_part_id = part.id\n".to_string(),
        format!("  WHERE part.name = '{}'\n", source_part.name),
        ") modified ON modified.timestamp = t;\n".to_string(),
    ].join("")
}

fn time_aggregate_function(
    source_part: &TrendStorePart,
    target_granularity: &Duration,
) -> Result<TrendMaterializationFunction, String> {
    let trend_columns: Vec<String> = source_part
        .trends
        .iter()
        .map(|trend| {
            format!(
                "  \"{}\" {}",
                trend.name,
                aggregate_data_type(trend.data_type, &trend.time_aggregation)
            )
        })
        .collect();

    let trend_column_expressions: Vec<String> = source_part
        .trends
        .iter()
        .map(|trend| {
            format!(
                "      {}(t.\"{}\")::{} AS \"{}\"",
                trend.time_aggregation,
                trend.name,
                aggregate_data_type(trend.data_type, &trend.time_aggregation),
                trend.name,
            )
        })
        .collect();

    let mut column_expressions = vec![
        "      entity_id".to_string(),
        "      $2 AS timestamp".to_string(),
    ];

    let mut result_columns = vec![
        "  \"entity_id\" integer".to_string(),
        "  \"timestamp\" timestamp with time zone".to_string(),
    ];

    if !source_part
        .trends
        .iter()
        .any(|trend| trend.name.eq("samples"))
    {
        column_expressions.push("      (count(*))::integer AS samples".to_string());
        result_columns.push("  samples integer".to_string());
    }

    column_expressions.extend(trend_column_expressions);

    result_columns.extend(trend_columns);

    let return_type_sql = format!("TABLE (\n{}\n)\n", result_columns.join(",\n"));

    let src = [
        "BEGIN\n".to_string(),
        "RETURN QUERY EXECUTE $query$\n".to_string(),
        "    SELECT\n".to_string(),
        column_expressions.join(",\n"),
        "\n".to_string(),
        format!("    FROM trend.\"{}\" AS t\n", source_part.name),
        "    WHERE $1 < timestamp AND timestamp <= $2\n".to_string(),
        "    GROUP BY entity_id\n".to_string(),
        format!(
            "$query$ USING $1 - interval '{}', $1;\n",
            humantime::format_duration(*target_granularity)
        ),
        "END;\n".to_string(),
    ]
    .join("");

    Ok(TrendMaterializationFunction {
        return_type: return_type_sql,
        src,
        language: "plpgsql".to_string(),
    })
}

struct TimeAggregation {
    pub source: String,
    pub granularity: Duration,
    pub mapping_function: String,
    pub parts: Vec<AggregationPart>,
}

fn generate_time_aggregation(
    trend_store: &TrendStore,
    source_granularity: &Duration,
    target_granularity: &Duration,
) -> Result<TimeAggregation, String> {
    let target_granularity_suffix = granularity_to_suffix(target_granularity)?;
    let source_granularity_suffix = granularity_to_suffix(source_granularity)?;

    let source_name = translate_time_aggregation_part_name(
        &trend_store_name(trend_store)?,
        &source_granularity_suffix,
    )?;

    let mapping_function = format!(
        "trend.mapping_{}->{}",
        source_granularity_suffix, target_granularity_suffix
    );

    let parts: Vec<AggregationPart> = trend_store
        .parts
        .iter()
        .map(|part| AggregationPart {
            name: translate_time_aggregation_part_name(&part.name, &target_granularity_suffix)
                .unwrap(),
            source: translate_time_aggregation_part_name(&part.name, &source_granularity_suffix)
                .unwrap(),
        })
        .collect();

    Ok(TimeAggregation {
        source: source_name,
        granularity: *target_granularity,
        mapping_function,
        parts,
    })
}

fn translate_time_aggregation_part_name(
    name: &str,
    target_granularity_suffix: &str,
) -> Result<String, String> {
    let re = regex::Regex::new("^(.*)_[^_]+$").unwrap();

    match re.captures(name) {
        Some(captures) => {
            let entity_type_and_data_source = &captures[1];

            Ok(format!(
                "{entity_type_and_data_source}_{target_granularity_suffix}"
            ))
        }
        None => Err(format!("Could not translate part name '{}'", name)),
    }
}

fn build_entity_aggregation(
    minerva_instance: &MinervaInstance,
    trend_store: &TrendStore,
    relation: &Relation,
    target_entity_type: &str,
    aggregation_hints: &[AggregationHint],
) -> Result<(), String> {
    let default_hint = AggregationHint {
        relation: relation.name.clone(),
        aggregation_type: AggregationType::FunctionMaterialization,
        prefix: None,
    };
    let aggregation_hint = aggregation_hints
        .iter()
        .find(|hint| hint.relation == relation.name)
        .unwrap_or(&default_hint);

    let entity_aggregation = generate_entity_aggregation(
        trend_store,
        relation,
        aggregation_hint.aggregation_type,
        target_entity_type,
        aggregation_hint.prefix.clone(),
    )?;

    let file_name = format!("{}.yaml", entity_aggregation.name);
    let aggregation_file_path: PathBuf = [PathBuf::from("aggregation"), PathBuf::from(&file_name)]
        .iter()
        .collect();

    let aggregation_context = AggregationContext {
        definition: entity_aggregation,
        source_definition: trend_store.clone(),
        aggregation_file_path,
    };

    compile_entity_aggregation(minerva_instance, &aggregation_context)?;

    Ok(())
}

#[derive(Debug)]
struct AggregationPart {
    pub name: String,
    pub source: String,
}

struct EntityAggregation {
    pub name: String,
    pub data_source: String,
    pub entity_type: String,
    pub relation: String,
    pub aggregation_type: AggregationType,
    pub parts: Vec<AggregationPart>,
}

struct AggregationContext {
    definition: EntityAggregation,
    source_definition: TrendStore,
    aggregation_file_path: PathBuf,
}

fn generate_entity_aggregation(
    trend_store: &TrendStore,
    relation: &Relation,
    aggregation_type: AggregationType,
    target_entity_type: &str,
    aggregation_prefix: Option<String>,
) -> Result<EntityAggregation, String> {
    let granularity_suffix = granularity_to_suffix(&trend_store.granularity)?;
    let name = match aggregation_prefix.clone() {
        Some(prefix) => format!(
            "{}_{}_{}_{}",
            trend_store.data_source, prefix, target_entity_type, granularity_suffix
        ),
        None => format!(
            "{}_{}_{}",
            trend_store.data_source, target_entity_type, granularity_suffix
        ),
    };

    let parts: Vec<AggregationPart> = trend_store
        .parts
        .iter()
        .map(|part| AggregationPart {
            name: translate_entity_aggregation_part_name(
                part.name.clone(),
                target_entity_type,
                aggregation_prefix.clone(),
            )
            .unwrap(),
            source: part.name.clone(),
        })
        .collect();

    let entity_aggregation = EntityAggregation {
        name: name.clone(),
        data_source: trend_store.data_source.clone(),
        entity_type: target_entity_type.to_string(),
        relation: relation.name.clone(),
        aggregation_type,
        parts,
    };

    Ok(entity_aggregation)
}

fn compile_entity_aggregation(
    minerva_instance: &MinervaInstance,
    aggregation_context: &AggregationContext,
) -> Result<(), String> {
    match aggregation_context.definition.aggregation_type {
        AggregationType::FunctionMaterialization => {
            write_function_entity_aggregations(minerva_instance, aggregation_context)
        }
        AggregationType::View => {
            generate_view_entity_aggregation(minerva_instance, aggregation_context)
        }
        AggregationType::ViewMaterialization => {
            write_view_entity_aggregations(minerva_instance, aggregation_context)
        }
    }
}

fn write_function_entity_aggregations(
    minerva_instance: &MinervaInstance,
    aggregation_context: &AggregationContext,
) -> Result<(), String> {
    for part in &aggregation_context.source_definition.parts {
        let default_dest_part = AggregationPart {
            name: translate_source_part_name(aggregation_context, &part.name)?,
            source: part.name.clone(),
        };

        let dest_part = aggregation_context
            .definition
            .parts
            .iter()
            .find(|p| p.source == part.name)
            .unwrap_or(&default_dest_part);

        let aggregation = define_function_part_entity_aggregation(
            part,
            aggregation_context.definition.relation.clone(),
            dest_part.name.clone(),
        );

        let file_path: PathBuf = [
            minerva_instance.instance_root.clone().unwrap(),
            PathBuf::from("materialization"),
            PathBuf::from(format!("{}.yaml", aggregation.target_trend_store_part)),
        ]
        .iter()
        .collect();

        println!(
            "Writing entity materialization to '{}'",
            file_path.to_string_lossy()
        );

        let file = File::create(file_path).unwrap();

        let mut writer = BufWriter::new(file);

        let relative_aggregation_file_path =
            aggregation_context.aggregation_file_path.to_string_lossy();
        let relative_source_definition_path = format!(
            "{}_{}_{}",
            aggregation_context.source_definition.data_source,
            aggregation_context.source_definition.entity_type,
            granularity_to_suffix(&aggregation_context.source_definition.granularity)?
        );

        writeln!(
            writer,
            "###########################################################################"
        )
        .unwrap();
        writeln!(writer, "#").unwrap();
        writeln!(
            writer,
            "# This file is automatically generated by the `minerva aggregation` command"
        )
        .unwrap();
        writeln!(writer, "#").unwrap();
        writeln!(
            writer,
            "# definition:         {}",
            relative_aggregation_file_path
        )
        .unwrap();
        writeln!(
            writer,
            "# source trend store: {}",
            relative_source_definition_path
        )
        .unwrap();
        writeln!(writer, "#").unwrap();
        writeln!(
            writer,
            "###########################################################################"
        )
        .unwrap();

        serde_yaml::to_writer(writer, &aggregation).unwrap();
    }

    Ok(())
}

fn define_function_part_entity_aggregation(
    source_part: &TrendStorePart,
    relation: String,
    name: String,
) -> TrendFunctionMaterialization {
    TrendFunctionMaterialization {
        target_trend_store_part: name,
        enabled: true,
        processing_delay: Duration::from_secs(1800),
        stability_delay: Duration::from_secs(300),
        reprocessing_period: Duration::from_secs(86400 * 3),
        sources: vec![TrendMaterializationSource {
            trend_store_part: source_part.name.clone(),
            mapping_function: "trend.mapping_id".to_string(),
        }],
        function: entity_aggregation_function(source_part, relation),
        fingerprint_function: define_fingerprint_sql(source_part),
        description: None,
    }
}

fn entity_aggregation_function(
    source_part: &TrendStorePart,
    relation_name: String,
) -> TrendMaterializationFunction {
    let trend_columns: Vec<String> = source_part
        .trends
        .iter()
        .map(|trend| {
            format!(
                "  {}(\"{}\")::{} AS \"{}\"",
                trend.entity_aggregation,
                trend.name,
                aggregate_data_type(trend.data_type, &trend.time_aggregation),
                trend.name
            )
        })
        .collect();

    let mut result_columns = vec![
        "\"entity_id\" integer".to_string(),
        "\"timestamp\" timestamp with time zone".to_string(),
    ];

    let mut columns = vec![
        "  r.target_id AS entity_id".to_string(),
        "  $1 AS timestamp".to_string(),
    ];

    if !source_part
        .trends
        .iter()
        .any(|trend| trend.name.eq("samples"))
    {
        columns.push("  count(*)::integer AS samples".to_string());
        result_columns.push("samples integer".to_string());
    }

    result_columns.extend(source_part.trends.iter().map(|trend| {
        format!(
            "\"{}\" {}",
            trend.name,
            aggregate_data_type(trend.data_type, &trend.time_aggregation)
        )
    }));

    columns.extend(trend_columns);

    let return_type_sql = format!("TABLE (\n{}\n)\n", result_columns.join(",\n"));

    let columns_part = columns.join(",\n       ");

    let src = [
        "BEGIN\n".to_string(),
        "RETURN QUERY EXECUTE $query$\n".to_string(),
        "    SELECT\n".to_string(),
        format!("        {columns_part}\n"),
        format!("    FROM trend.\"{}\" t\n", source_part.name),
        format!("    JOIN relation.\"{relation_name}\" r ON t.entity_id = r.source_id\n"),
        "    WHERE timestamp = $1\n".to_string(),
        "    GROUP BY r.target_id".to_string(),
        "$query$ USING $1;\n".to_string(),
        "END;".to_string(),
    ]
    .join("");

    TrendMaterializationFunction {
        return_type: return_type_sql,
        src,
        language: "plpgsql".to_string(),
    }
}

fn aggregate_data_type(data_type: DataType, aggregation: &str) -> DataType {
    static AGGREGATE_DATA_TYPE_MAPPING_SUM: OnceLock<HashMap<DataType, DataType>> = OnceLock::new();
    static AGGREGATE_DATA_TYPE_MAPPING_AVG: OnceLock<HashMap<DataType, DataType>> = OnceLock::new();

    let sum_mapping = AGGREGATE_DATA_TYPE_MAPPING_SUM.get_or_init(|| {
        vec![
            (DataType::Int2, DataType::Int8),
            (DataType::Integer, DataType::Int8),
            (DataType::Int8, DataType::Int8),
            (DataType::Double, DataType::Double),
            (DataType::Real, DataType::Real),
            (DataType::Numeric, DataType::Numeric),
        ]
        .into_iter()
        .collect()
    });

    let avg_mapping = AGGREGATE_DATA_TYPE_MAPPING_AVG.get_or_init(|| {
        vec![
            (DataType::Int2, DataType::Numeric),
            (DataType::Integer, DataType::Numeric),
            (DataType::Int8, DataType::Numeric),
            (DataType::Double, DataType::Double),
            (DataType::Real, DataType::Double),
            (DataType::Numeric, DataType::Numeric),
        ]
        .into_iter()
        .collect()
    });

    match aggregation.to_uppercase().as_str() {
        "SUM" => *sum_mapping.get(&data_type).unwrap_or(&data_type),
        "AVG" => *avg_mapping.get(&data_type).unwrap_or(&data_type),
        _ => data_type,
    }
}

fn define_fingerprint_sql(source_part: &TrendStorePart) -> String {
    [
        format!(
            "SELECT modified.last, format('{{\"{}\": \"%s\"}}', modified.last)::jsonb\n",
            source_part.name
        ),
        "FROM trend_directory.modified\n".to_string(),
        "JOIN trend_directory.trend_store_part ttsp ON ttsp.id = modified.trend_store_part_id\n"
            .to_string(),
        format!(
            "WHERE ttsp::name = '{}' AND modified.timestamp = $1;\n",
            source_part.name
        ),
    ]
    .join("")
}

fn translate_source_part_name(
    aggregation_context: &AggregationContext,
    name: &str,
) -> Result<String, String> {
    let granularity_suffix =
        granularity_to_suffix(&aggregation_context.source_definition.granularity).unwrap();

    let pattern = format!("_([^_]+)_{granularity_suffix}$");

    let re = regex::Regex::new(&pattern).unwrap();

    let part_specific_name = match re.captures(name) {
        Some(captures) => captures[1].to_string(),
        None => {
            return Err(format!(
                "Could not extract part specific string from '{name}'"
            ))
        }
    };

    Ok(format!(
        "{}_{}_{}_{}",
        aggregation_context.definition.data_source,
        aggregation_context.definition.entity_type,
        part_specific_name,
        granularity_suffix
    ))
}

fn generate_view_entity_aggregation(
    _minerva_instance: &MinervaInstance,
    _aggregation_context: &AggregationContext,
) -> Result<(), String> {
    Ok(())
}

fn write_view_entity_aggregations(
    _minerva_instance: &MinervaInstance,
    _aggregation_context: &AggregationContext,
) -> Result<(), String> {
    Ok(())
}

/// Translate a part name with standard naming convention <data_source>_<entity_type>_<granularity>
/// to <data_source_<target_entity_type>_<granularity>.
fn translate_entity_aggregation_part_name(
    name: String,
    target_entity_type: &str,
    aggregation_prefix: Option<String>,
) -> Result<String, String> {
    let re = regex::Regex::new("^([^_]+)_([^_]+)_(.*)$").unwrap();

    match re.captures(&name) {
        Some(captures) => {
            let data_source = &captures[1];
            let _entity_type = &captures[2];
            let tail = &captures[3];

            match aggregation_prefix {
                Some(prefix) => Ok(format!(
                    "{data_source}_{target_entity_type}_{prefix}_{tail}"
                )),
                None => Ok(format!("{data_source}_{target_entity_type}_{tail}")),
            }
        }
        None => Err(format!("Could not translate part name '{name}'")),
    }
}

fn granularity_to_suffix(granularity: &Duration) -> Result<String, String> {
    static GRANULARITY_SUFFIX_MAPPING: OnceLock<HashMap<Duration, String>> = OnceLock::new();

    let standard_aggregations = GRANULARITY_SUFFIX_MAPPING.get_or_init(|| {
        [
            (Duration::from_secs(900), "15m".to_string()),
            (Duration::from_secs(3600), "1d".to_string()),
            (humantime::parse_duration("1d").unwrap(), "1d".to_string()),
            (humantime::parse_duration("1w").unwrap(), "1w".to_string()),
            (
                humantime::parse_duration("1month").unwrap(),
                "1month".to_string(),
            ),
        ]
        .into_iter()
        .collect()
    });

    standard_aggregations
        .get(granularity)
        .ok_or(format!("No predefined granularity '{:?}'", granularity))
        .cloned()
}

