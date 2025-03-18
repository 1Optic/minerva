use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use clap::Parser;
use minerva::aggregation_generation::{granularity_to_partition_size, save_trend_store};
use minerva::instance::load_trend_stores_from;
use minerva::meas_value::DataType;
use minerva::trend_store::{Trend, TrendStore, TrendStorePart};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct DefineOpt {
    #[arg(help = "Root directory of Minerva instance to write to")]
    instance_root: PathBuf,
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

#[async_trait]
impl Cmd for DefineOpt {
    async fn run(&self) -> CmdResult {
        let trend_definitions: Vec<TrendDefinition> =
            serde_json::from_reader(std::io::stdin()).unwrap();

        let current_trend_stores: Vec<TrendStore> =
            load_trend_stores_from(&self.instance_root).collect();

        let trend_stores = define_trend_stores(
            &trend_definitions,
            &current_trend_stores,
            TrendStorePartParameters::default(),
        );

        for trend_store in &trend_stores {
            save_trend_store(&self.instance_root, trend_store).unwrap();
        }

        Ok(())
    }
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

struct TrendStorePartHolder {
    trend_store_key: TrendStoreKey,
    parts: HashMap<String, TrendStorePart>,
}

impl TrendStorePartHolder {
    pub fn new(trend_store_key: TrendStoreKey) -> Self {
        TrendStorePartHolder { trend_store_key, parts: HashMap::new() }
    }

    pub fn add_trend_to_existing_part(&mut self, part_name: &str, trend: Trend) {
        let part = self.parts.entry(part_name.to_string()).or_insert(TrendStorePart {
            name: part_name.to_string(),
            trends: Vec::new(),
            generated_trends: Vec::new(),
        });

        part.trends.push(trend);
    }

    pub fn add_trend(&mut self, params: &TrendStorePartParameters, trend_definition: &TrendDefinition) -> String {
        let default_part_name = trend_definition.get_default_part_name();

        let mut part = self.parts.entry(default_part_name.clone()).or_insert(TrendStorePart {
            name: default_part_name.clone(),
            trends: Vec::new(),
            generated_trends: Vec::new(),
        });

        let mut current_num = 1;

        while calc_row_size(params, part) > params.max_row_width {
            let next_part_name = trend_definition.get_numbered_part_name(current_num);

            println!("Adding to '{next_part_name}'");

            part = self.parts.entry(next_part_name.clone()).or_insert(TrendStorePart {
                name: next_part_name.clone(),
                trends: Vec::new(),
                generated_trends: Vec::new(),
            });

            current_num += 1;
        }

        part.trends.push(trend_definition.into());

        part.name.clone()
    }
}

fn calc_row_size(params: &TrendStorePartParameters, trend_store_part: &TrendStorePart) -> u16 {
    let num_columns = TryInto::<u16>::try_into(trend_store_part.trends.len()).unwrap();

    let result = params.base_width + num_columns * params.column_size;

    println!("row size of '{}'({} trends): {result}", trend_store_part.name, trend_store_part.trends.len());

    result
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
        let current_trend_store_part = current_trend_store
            .and_then(|t| {
                t.parts.iter().find(|p| {
                    p.trends
                        .iter()
                        .any(|trend| trend.name.eq(&trend_definition.name))
                })
            });

        match current_trend_store_part {
            Some(current_part) => {
                println!("curr: {}", current_part.name);

                // Check if the part name is suffixed with a digit, indicating that it is part of
                // a split part.
                let stripped_part_name = re.replace(&current_part.name, "").to_string();

                println!("stripped part name: {}", stripped_part_name);

                let part_holder = trend_store_parts
                    .entry(stripped_part_name.clone())
                    .or_insert(TrendStorePartHolder::new(trend_definition.get_trend_store_key()));

                part_holder.add_trend_to_existing_part(&current_part.name, trend_definition.into());
                println!("- Added '{}' to '{}'", trend_definition.name, current_part.name);
            }, 
            None => {
                remaining_trend_definitions.push(trend_definition);
            }
        }
    }

    // Now give the remaining trends a place

    for trend_definition in remaining_trend_definitions {
        let part_holder = trend_store_parts
            .entry(trend_definition.get_default_part_name())
            .or_insert(TrendStorePartHolder::new(trend_definition.get_trend_store_key()));

        let part_name = part_holder.add_trend(&params, trend_definition);
        println!("- Added '{}' to '{}'", trend_definition.name, part_name);
    }

    // Collect the trend store parts into trend stores

    let mut trend_stores: HashMap<TrendStoreKey, TrendStore> = HashMap::new();

    for part_holder in trend_store_parts.into_values() {
        let trend_store = trend_stores.entry(part_holder.trend_store_key.clone()).or_insert(TrendStore {
            title: Some("Raw data trend store generated from counter list".to_string()),
            data_source: part_holder.trend_store_key.data_source.clone(),
            entity_type: part_holder.trend_store_key.entity_type.clone(),
            granularity: part_holder.trend_store_key.granularity,
            partition_size: granularity_to_partition_size(part_holder.trend_store_key.granularity).unwrap(),
            retention_period: granularity_to_partition_size(part_holder.trend_store_key.granularity).unwrap(),
            parts: Vec::new(),
        });

        let mut parts: Vec<TrendStorePart> = part_holder.parts.into_values().collect();
        parts.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        trend_store.parts.extend(parts);
    }

    trend_stores.into_values().collect()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;

    use minerva::trend_store::{Trend, TrendStore, TrendStorePart};

    use crate::commands::define::TrendDefinition;

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
            }],
        }];

        let params = TrendStorePartParameters {
            base_width: 3,
            max_row_width: 6,
            column_size: 15,
        };

        let new_trend_stores =
            define_trend_stores(&trend_definitions, &current_trend_stores, params);

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
}
