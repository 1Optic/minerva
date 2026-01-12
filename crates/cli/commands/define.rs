use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use minerva::aggregation_generation::{granularity_to_partition_size, save_trend_store};
use minerva::attribute_store::{Attribute, AttributeStore};
use minerva::error::RuntimeError;
use minerva::instance::{load_instance_config, load_trend_stores_from, InstanceConfig};
use minerva::meas_value::DataType;
use minerva::trend_store::{Trend, TrendStore, TrendStorePart};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct DefineOpt {
    #[arg(help = "Root directory of Minerva instance to write to")]
    instance_root: PathBuf,
    #[arg(help = "Directory with trend and attribute definitions")]
    definitions_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct TrendDefinition {
    data_source: String,
    entity_type: String,
    #[serde(with = "humantime_serde")]
    granularity: Duration,
    part: String,
    name: String,
    data_type: DataType,
    description: String,
    time_aggregation: String,
    entity_aggregation: String,
    extra_data: Value,
}

impl TrendDefinition {
    pub fn get_trend_store_key(&self) -> TrendStoreKey {
        TrendStoreKey {
            data_source: self.data_source.clone(),
            entity_type: self.entity_type.clone(),
            granularity: self.granularity,
        }
    }

    pub fn get_default_part_name(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            self.data_source,
            self.entity_type,
            self.part,
            humantime::format_duration(self.granularity)
        )
    }

    pub fn get_numbered_part_name(&self, number: u16) -> String {
        format!(
            "{}_{}_{}{}_{}",
            self.data_source,
            self.entity_type,
            self.part,
            number,
            humantime::format_duration(self.granularity)
        )
    }
}

impl From<&TrendDefinition> for Trend {
    fn from(value: &TrendDefinition) -> Self {
        Trend {
            name: value.name.clone(),
            data_type: value.data_type,
            description: value.description.clone(),
            entity_aggregation: value.entity_aggregation.clone(),
            time_aggregation: value.time_aggregation.clone(),
            extra_data: value.extra_data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AttributeDefinition {
    data_source: String,
    entity_type: String,
    name: String,
    data_type: DataType,
    description: String,
    extra_data: Value,
}

impl From<&AttributeDefinition> for Attribute {
    fn from(value: &AttributeDefinition) -> Self {
        Attribute {
            name: value.name.clone(),
            data_type: value.data_type,
            description: value.description.clone(),
            extra_data: value.extra_data.clone(),
        }
    }
}

impl Cmd for DefineOpt {
    fn run(&self) -> CmdResult {
        let trend_definitions_file_path = self.definitions_dir.join("trends.json");
        let trend_definitions_file = File::open(&trend_definitions_file_path).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not open trend definitions file '{}': {e}",
                trend_definitions_file_path.to_string_lossy()
            ))
        })?;
        let trend_definitions: Vec<TrendDefinition> =
            serde_json::from_reader(BufReader::new(trend_definitions_file)).map_err(|e| {
                RuntimeError::from_msg(format!(
                    "Could not load trend definitions file '{}': {e}",
                    trend_definitions_file_path.to_string_lossy()
                ))
            })?;

        let attribute_definitions_file_path = self.definitions_dir.join("attributes.json");
        let attribute_definitions_file =
            File::open(&attribute_definitions_file_path).map_err(|e| {
                RuntimeError::from_msg(format!(
                    "Could not open attribute definitions file '{}': {e}",
                    attribute_definitions_file_path.to_string_lossy()
                ))
            })?;

        let mut attribute_definitions: Vec<AttributeDefinition> =
            serde_json::from_reader(BufReader::new(attribute_definitions_file)).map_err(|e| {
                RuntimeError::from_msg(format!(
                    "Could not load attribute definitions file '{}': {e}",
                    attribute_definitions_file_path.to_string_lossy()
                ))
            })?;

        let instance_config = load_instance_config(&self.instance_root).map_err(|e| {
            RuntimeError::from_msg(format!("Could not load instance configuration: {e}"))
        })?;

        let current_trend_stores: Vec<TrendStore> =
            load_trend_stores_from(&self.instance_root).collect();

        let trend_stores = define_trend_stores(
            &trend_definitions,
            &current_trend_stores,
            TrendStorePartParameters::default(),
            &instance_config,
        );

        for trend_store in &trend_stores {
            println!("Saving trend store '{}'", trend_store);
            save_trend_store(&self.instance_root, trend_store).unwrap();
        }

        let attribute_stores =
            define_attribute_stores(&mut attribute_definitions, &instance_config);

        for attribute_store in &attribute_stores {
            println!("Saving attribute store '{}'", attribute_store);
            save_attribute_store(&self.instance_root, attribute_store).unwrap();
        }

        Ok(())
    }
}

pub fn save_attribute_store(
    instance_root: &Path,
    attribute_store: &AttributeStore,
) -> Result<(), String> {
    let attribute_store_file_name = format!(
        "{}_{}.yaml",
        attribute_store.data_source, attribute_store.entity_type
    );

    let attribute_store_dir_path: PathBuf =
        PathBuf::from_iter([instance_root, &PathBuf::from("attribute")]);

    // Check if the attribute directory exists
    if !attribute_store_dir_path.is_dir() {
        std::fs::create_dir(&attribute_store_dir_path).map_err(|e| {
            format!(
                "Could not create attribute store directory '{}': {e}",
                &attribute_store_dir_path.to_string_lossy()
            )
        })?;
    }

    let attribute_store_file_path: PathBuf = PathBuf::from_iter([
        &attribute_store_dir_path,
        &PathBuf::from(attribute_store_file_name),
    ]);

    let file = File::create(attribute_store_file_path).unwrap();

    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, &attribute_store).unwrap();

    Ok(())
}

struct TrendStorePartParameters {
    pub base_width: u16,
    pub max_row_width: u16,
    pub column_size: u16,
}

impl Default for TrendStorePartParameters {
    fn default() -> Self {
        TrendStorePartParameters {
            base_width: 36,
            max_row_width: 8160,
            column_size: 15,
        }
    }
}

#[derive(PartialEq, Eq)]
struct TrendStorePartHolder {
    base_name: String,
    trend_store_key: TrendStoreKey,
    parts: HashMap<String, TrendStorePart>,
}

impl Ord for TrendStorePartHolder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.base_name.cmp(&other.base_name)
    }
}

impl PartialOrd for TrendStorePartHolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TrendStorePartHolder {
    pub fn new(base_name: String, trend_store_key: TrendStoreKey) -> Self {
        TrendStorePartHolder {
            base_name,
            trend_store_key,
            parts: HashMap::new(),
        }
    }

    pub fn add_trend_to_existing_part(&mut self, part_name: &str, trend: Trend) {
        let part = self
            .parts
            .entry(part_name.to_string())
            .or_insert(TrendStorePart {
                name: part_name.to_string(),
                trends: Vec::new(),
                generated_trends: Vec::new(),
                has_alias_column: false,
            });

        part.trends.push(trend);
    }

    pub fn add_trend(
        &mut self,
        params: &TrendStorePartParameters,
        trend_definition: &TrendDefinition,
    ) -> String {
        let default_part_name = trend_definition.get_default_part_name();

        let mut part = self
            .parts
            .entry(default_part_name.clone())
            .or_insert(TrendStorePart {
                name: default_part_name.clone(),
                trends: Vec::new(),
                generated_trends: Vec::new(),
                has_alias_column: false,
            });

        let mut current_num = 1;

        while calc_row_size(params, part) > params.max_row_width {
            let next_part_name = trend_definition.get_numbered_part_name(current_num);

            part = self
                .parts
                .entry(next_part_name.clone())
                .or_insert(TrendStorePart {
                    name: next_part_name.clone(),
                    trends: Vec::new(),
                    generated_trends: Vec::new(),
                    has_alias_column: false,
                });

            current_num += 1;
        }

        part.trends.push(trend_definition.into());

        part.name.clone()
    }
}

fn calc_row_size(params: &TrendStorePartParameters, trend_store_part: &TrendStorePart) -> u16 {
    let num_columns = TryInto::<u16>::try_into(trend_store_part.trends.len()).unwrap();

    params.base_width + num_columns * params.column_size
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TrendStoreKey {
    data_source: String,
    entity_type: String,
    granularity: Duration,
}

fn define_trend_stores(
    trend_definitions: &[TrendDefinition],
    current_trend_stores: &[TrendStore],
    params: TrendStorePartParameters,
    instance_config: &InstanceConfig,
) -> Vec<TrendStore> {
    let re = regex::Regex::new(r"[1-9]$").unwrap();
    let mut trend_store_parts: HashMap<String, TrendStorePartHolder> = HashMap::new();

    // First check which trends already exist in the current trend stores and place them into the
    // same parts.

    let mut remaining_trend_definitions: Vec<&TrendDefinition> = Vec::new();

    for trend_definition in trend_definitions {
        let current_trend_store = current_trend_stores.iter().find(|t| {
            t.data_source.eq(&trend_definition.data_source)
                && t.entity_type.eq(&trend_definition.entity_type)
                && t.granularity.eq(&trend_definition.granularity)
        });

        // Check if this trend already existed in the current trend store
        let current_trend_store_part = current_trend_store.and_then(|t| {
            t.parts.iter().find(|p| {
                p.trends
                    .iter()
                    .any(|trend| trend.name.eq(&trend_definition.name))
            })
        });

        match current_trend_store_part {
            Some(current_part) => {
                // Check if the part name is suffixed with a digit, indicating that it is part of
                // a split part.
                let stripped_part_name = re.replace(&current_part.name, "").to_string();
                let part_holder = trend_store_parts
                    .entry(stripped_part_name.clone())
                    .or_insert(TrendStorePartHolder::new(
                        stripped_part_name.clone(),
                        trend_definition.get_trend_store_key(),
                    ));

                part_holder.add_trend_to_existing_part(&current_part.name, trend_definition.into());
            }
            None => {
                remaining_trend_definitions.push(trend_definition);
            }
        }
    }

    // Now give the remaining trends a place

    for trend_definition in remaining_trend_definitions {
        let part_holder = trend_store_parts
            .entry(trend_definition.get_default_part_name())
            .or_insert(TrendStorePartHolder::new(
                trend_definition.get_default_part_name(),
                trend_definition.get_trend_store_key(),
            ));

        part_holder.add_trend(&params, trend_definition);
    }

    // Collect the trend store parts into trend stores

    let mut trend_stores: HashMap<TrendStoreKey, TrendStore> = HashMap::new();

    let mut part_holders: Vec<TrendStorePartHolder> = trend_store_parts.into_values().collect();

    part_holders.sort();

    for part_holder in part_holders {
        let trend_store = trend_stores
            .entry(part_holder.trend_store_key.clone())
            .or_insert(TrendStore {
                title: Some("Raw data trend store generated from counter list".to_string()),
                data_source: part_holder.trend_store_key.data_source.clone(),
                entity_type: part_holder.trend_store_key.entity_type.clone(),
                granularity: part_holder.trend_store_key.granularity,
                partition_size: granularity_to_partition_size(
                    part_holder.trend_store_key.granularity,
                )
                .unwrap(),
                retention_period: instance_config
                    .granularity_to_retention(part_holder.trend_store_key.granularity)
                    .unwrap_or(Duration::from_secs(86400 * 30)),
                parts: Vec::new(),
            });

        let mut parts: Vec<TrendStorePart> = part_holder.parts.into_values().collect();
        parts.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        trend_store.parts.extend(parts);
    }

    trend_stores.into_values().collect()
}

fn define_attribute_stores(
    attribute_definitions: &mut [AttributeDefinition],
    instance_config: &InstanceConfig,
) -> Vec<AttributeStore> {
    attribute_definitions.sort_by(|a, b| {
        (&a.data_source, &a.entity_type, &a.name).cmp(&(&b.data_source, &b.entity_type, &b.name))
    });

    let mut attribute_stores: HashMap<(&str, &str), AttributeStore> = HashMap::new();

    for attribute_definition in attribute_definitions {
        // First determine the entity type name by checking if a name is already defined
        let entity_type_name = instance_config
            .entity_types
            .iter()
            .find(|t| {
                t.to_lowercase()
                    .eq(&attribute_definition.entity_type.to_lowercase())
            })
            .unwrap_or(&attribute_definition.entity_type);

        let attribute_store = attribute_stores
            .entry((&attribute_definition.data_source, entity_type_name))
            .or_insert_with(|| AttributeStore {
                data_source: attribute_definition.data_source.clone(),
                entity_type: entity_type_name.clone(),
                attributes: Vec::new(),
            });

        if attribute_store
            .attributes
            .iter()
            .any(|a| a.name.eq(&attribute_definition.name))
        {
        } else {
            attribute_store
                .attributes
                .push(Attribute::from(&*attribute_definition));
        }
    }

    if let Some(add_attributes) = &instance_config.attribute_extraction.add_attributes {
        for a in add_attributes {
            let attribute_store_key = (
                instance_config.attribute_extraction.data_source.as_str(),
                a.entity_type.as_str(),
            );

            let attribute_store =
                attribute_stores
                    .entry(attribute_store_key)
                    .or_insert_with(|| AttributeStore {
                        data_source: instance_config.attribute_extraction.data_source.clone(),
                        entity_type: a.entity_type.clone(),
                        attributes: Vec::new(),
                    });

            for add_attribute in &a.attributes {
                attribute_store.attributes.push(Attribute {
                    name: add_attribute.name.clone(),
                    data_type: add_attribute.data_type,
                    description: "".to_string(),
                    extra_data: add_attribute.extra_data.clone(),
                });
            }

            println!("Matched attribute store: '{}'", attribute_store);
        }
    }

    attribute_stores.into_values().collect()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;

    use minerva::{
        instance::{
            AddAttribute, AddAttributes, DeploymentConfig, InstanceConfig, RetentionConfig,
        },
        trend_store::{Trend, TrendStore, TrendStorePart},
    };

    use crate::commands::define::{define_attribute_stores, AttributeDefinition, TrendDefinition};

    use super::{define_trend_stores, TrendStorePartParameters};

    #[test]
    fn test_define_trend_stores() {
        let trend_definitions = vec![
            TrendDefinition {
                data_source: "my-test".to_string(),
                entity_type: "node".to_string(),
                granularity: Duration::from_secs(900),
                name: "tx".to_string(),
                data_type: minerva::meas_value::DataType::Int8,
                description: "".to_string(),
                part: "traffic".to_string(),
                entity_aggregation: "sum".to_string(),
                time_aggregation: "sum".to_string(),
                extra_data: json!("{}"),
            },
            TrendDefinition {
                data_source: "my-test".to_string(),
                entity_type: "node".to_string(),
                granularity: Duration::from_secs(900),
                name: "rx".to_string(),
                data_type: minerva::meas_value::DataType::Int8,
                description: "".to_string(),
                part: "traffic".to_string(),
                entity_aggregation: "sum".to_string(),
                time_aggregation: "sum".to_string(),
                extra_data: json!("{}"),
            },
        ];
        let current_trend_stores = vec![TrendStore {
            title: Some("test".to_string()),
            data_source: "my-test".to_string(),
            entity_type: "node".to_string(),
            granularity: Duration::from_secs(900),
            partition_size: Duration::from_secs(86400),
            retention_period: Duration::from_secs(86400 * 7),
            parts: vec![TrendStorePart {
                name: "my-test_node_traffic_15m".to_string(),
                trends: vec![Trend {
                    name: "tx".to_string(),
                    data_type: minerva::meas_value::DataType::Int8,
                    description: "".to_string(),
                    entity_aggregation: "sum".to_string(),
                    time_aggregation: "sum".to_string(),
                    extra_data: json!("{}"),
                }],
                generated_trends: vec![],
                has_alias_column: false,
            }],
        }];

        let params = TrendStorePartParameters {
            base_width: 3,
            max_row_width: 6,
            column_size: 15,
        };

        let instance_config = InstanceConfig {
            docker_image: None,
            deployment: Some(DeploymentConfig::default()),
            entity_aggregation_hints: Some(Vec::new()),
            entity_types: Vec::new(),
            old_data_stability_delay: Some(Duration::from_secs(3600 * 3)),
            old_data_threshold: Some(Duration::from_secs(3600 * 6)),
            retention: Some(vec![RetentionConfig {
                granularity: humantime::parse_duration("15m").unwrap(),
                retention_period: humantime::parse_duration("14d").unwrap(),
            }]),
            attribute_extraction: minerva::instance::AttributeExtraction {
                data_source: "my-test".to_string(),
                add_attributes: Some(vec![AddAttributes {
                    entity_type: "node".to_string(),
                    description: "some extra attributes for testing".to_string(),
                    attributes: vec![],
                }]),
            },
        };

        let new_trend_stores = define_trend_stores(
            &trend_definitions,
            &current_trend_stores,
            params,
            &instance_config,
        );

        assert_eq!(
            new_trend_stores.len(),
            1,
            "There should be 1 new trend store"
        );

        let first_trend_store = new_trend_stores.first().unwrap();

        assert_eq!(
            first_trend_store.parts.len(),
            2,
            "There should be 2 new trend store parts"
        );

        let mut it = first_trend_store.parts.iter();
        let first_trend_store_part = it.next().unwrap();

        assert_eq!(first_trend_store_part.name, "my-test_node_traffic1_15m");
        assert_eq!(first_trend_store_part.trends.len(), 1);
        assert_eq!(first_trend_store_part.trends.first().unwrap().name, "rx");

        let second_trend_store_part = it.next().unwrap();

        assert_eq!(second_trend_store_part.name, "my-test_node_traffic_15m");
        assert_eq!(second_trend_store_part.trends.len(), 1);
        assert_eq!(second_trend_store_part.trends.first().unwrap().name, "tx");
    }

    #[test]
    fn test_define_attribute_stores() {
        let mut attribute_definitions = vec![
            AttributeDefinition {
                data_source: "test_cm".to_string(),
                entity_type: "node".to_string(),
                name: "FrqB".to_string(),
                data_type: minerva::meas_value::DataType::Text,
                description: "Some frequency B".to_string(),
                extra_data: json!("{}"),
            },
            AttributeDefinition {
                data_source: "test_cm".to_string(),
                entity_type: "node".to_string(),
                name: "FrqA".to_string(),
                data_type: minerva::meas_value::DataType::Text,
                description: "Some frequency A".to_string(),
                extra_data: json!("{}"),
            },
        ];

        let instance_config = InstanceConfig {
            docker_image: None,
            deployment: Some(DeploymentConfig::default()),
            entity_aggregation_hints: Some(Vec::new()),
            entity_types: Vec::new(),
            old_data_stability_delay: Some(Duration::from_secs(3600 * 3)),
            old_data_threshold: Some(Duration::from_secs(3600 * 6)),
            retention: Some(vec![RetentionConfig {
                granularity: humantime::parse_duration("15m").unwrap(),
                retention_period: humantime::parse_duration("14d").unwrap(),
            }]),
            attribute_extraction: minerva::instance::AttributeExtraction {
                data_source: "test_cm".to_string(),
                add_attributes: Some(vec![AddAttributes {
                    entity_type: "node".to_string(),
                    description: "some extra attributes for testing".to_string(),
                    attributes: vec![AddAttribute {
                        name: "Pwr".to_string(),
                        data_type: minerva::meas_value::DataType::Integer,
                        example: "40".to_string(),
                        extra_data: serde_json::Value::Null,
                    }],
                }]),
            },
        };

        let attribute_stores =
            define_attribute_stores(&mut attribute_definitions, &instance_config);

        assert_eq!(attribute_stores.len(), 1);

        let attribute_store = &attribute_stores[0];

        // We expect the 2 attributes from the definition and 1 from the extra added attributes
        assert_eq!(attribute_store.attributes.len(), 3);

        assert_eq!(attribute_store.attributes[0].name, "FrqA");
        assert_eq!(attribute_store.attributes[1].name, "FrqB");
    }
}
