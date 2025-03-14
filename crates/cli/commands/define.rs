use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use clap::Parser;
use minerva::aggregation_generation::granularity_to_partition_size;
use minerva::instance::load_trend_stores_from;
use minerva::meas_value::DataType;
use minerva::trend_store::{Trend, TrendStore, TrendStorePart};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct DefineOpt {
    #[arg(
        help = "Root directory of Minerva instance to write to"
    )]
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

#[async_trait]
impl Cmd for DefineOpt {
    async fn run(&self) -> CmdResult {
        let trend_definitions: Vec<TrendDefinition> =
            serde_json::from_reader(std::io::stdin()).unwrap();

        let current_trend_stores: Vec<TrendStore> = load_trend_stores_from(&self.instance_root).collect();

        define_trend_stores(&trend_definitions, &current_trend_stores, TrendStorePartParameters::default());

        Ok(())
    }
}

struct TrendStorePartParameters {
    pub base_width: u16,
    pub max_row_width: u16,
}

impl Default for TrendStorePartParameters {
    fn default() -> Self {
        TrendStorePartParameters {
            base_width: 36,
            max_row_width: 8160,
        }
    }
}

struct RegisterEntry {
    row_width: u16,
    part: TrendStorePart,
}

struct TrendStorePartRegister<'a> {
    current_trend_stores: &'a[TrendStore],
    name: String,
    parts: Vec<RegisterEntry>,
}

fn define_trend_stores(trend_definitions: &[TrendDefinition], current_trend_stores: &[TrendStore], params: TrendStorePartParameters) -> Vec<TrendStore> {
    let mut trend_store_parts: HashMap<_, TrendStorePartRegister> = HashMap::new();

    for trend_definition in trend_definitions {
        let trend_store_part_key = (
            trend_definition.data_source.clone(),
            trend_definition.entity_type.clone(),
            trend_definition.part.clone(),
            trend_definition.granularity,
        );

        let current_trend_store = current_trend_stores
            .iter()
            .find(|t| t.data_source.eq(&trend_definition.data_source) && t.entity_type.eq(&trend_definition.entity_type) && t.granularity.eq(&trend_definition.granularity));

        let current_trend_store_part_name = current_trend_store.map(|t| t.parts.iter().find(|p| p.trends.iter().any(|trend| trend.name.eq(&trend_definition.name))))

        println!(" -> {:?}", trend_store_part_key);

        let part_name = format!("{}_{}_{}_{}", trend_definition.data_source, trend_definition.entity_type, trend_definition.part, humantime::format_duration(trend_definition.granularity));

        let trend_store_part_register = trend_store_parts.entry(trend_store_part_key).or_insert(
            TrendStorePartRegister {
                current_trend_stores,
                name: trend_definition.part.clone(),
                parts: vec![
                    RegisterEntry {
                        row_width: params.base_width,
                        part: TrendStorePart {
                            name: part_name,
                            trends: Vec::new(),
                            generated_trends: Vec::new(),
                        }
                    }
                ]
            }
        );

        trend_store_part_register.parts.iter_mut().next().unwrap().part.trends.push(Trend {
            name: trend_definition.name.clone(),
            data_type: trend_definition.data_type,
            description: trend_definition.description.clone(),
            entity_aggregation: trend_definition.entity_aggregation.clone(),
            time_aggregation: trend_definition.time_aggregation.clone(),
            extra_data: trend_definition.extra_data.clone(),
        });

        println!("- {}", trend_definition.name);
    }

    let mut trend_stores: HashMap<_, TrendStore> = HashMap::new();

    for ((data_source, entity_type, part_name, granularity), parts_register) in trend_store_parts.into_iter() {
        let trend_store_key = (data_source.clone(), entity_type.clone(), granularity);

        let trend_store = trend_stores.entry(trend_store_key).or_insert(TrendStore {
            title: Some("Raw data trend store generated from counter list".to_string()),
            data_source: data_source.clone(),
            entity_type: entity_type.clone(),
            granularity,
            partition_size: granularity_to_partition_size(granularity)
                .unwrap(),
            retention_period: granularity_to_partition_size(granularity)
                .unwrap(),
            parts: Vec::new(),
        });

        for part_entry in parts_register.parts {
            trend_store.parts.push(part_entry.part);
        }
    }

    trend_stores.into_values().collect()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;

    use minerva::trend_store::TrendStore;

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
            }
        ];
        let current_trend_stores = vec![
            TrendStore {
                title: Some("test".to_string()),
                data_source: "my-test".to_string(),
                entity_type: "node".to_string(),
                granularity: Duration::from_secs(900),
                partition_size: Duration::from_secs(86400),
                retention_period: Duration::from_secs(86400 * 7),
                parts: vec![],
            }
        ];

        let params = TrendStorePartParameters { base_width: 3, max_row_width: 6 };

        let new_trend_stores = define_trend_stores(&trend_definitions, &current_trend_stores, params);

        assert_eq!(new_trend_stores.len(), 1, "There should be 1 new trend store");

        let first_trend_store = new_trend_stores.first().unwrap();

        assert_eq!(first_trend_store.parts.len(), 1, "There should be 1 new trend store part");

        let first_trend_store_part = first_trend_store.parts.first().unwrap();

        assert_eq!(first_trend_store_part.name, "my-test_node_traffic_15m");
    }
}
