use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use erased_serde::{Serialize, Serializer};
use glob::glob;
use log::error;

use serde_json::Value;
use std::time::Duration;
use tokio_postgres::Client;

use crate::attribute_materialization::AddAttributeMaterialization;
use crate::changes::trend_store::{RemoveTrendStore, RemoveTrendStorePart};
use crate::entity_type::{load_entity_types, load_entity_types_from, AddEntityType, EntityType};
use crate::error::RuntimeError;
use crate::graph::GraphNode;
use crate::meas_value::DataType;
use crate::relation::{load_relations_from_db, RemoveRelation};
use crate::trend_materialization::{RemoveTrendMaterialization, TrendMaterializationSource};
use crate::virtual_entity::{load_virtual_entities_from_db, RemoveVirtualEntity};

use super::attribute_materialization::{
    load_attribute_materializations, load_attribute_materializations_from, AttributeMaterialization,
};
use super::attribute_store::{
    load_attribute_stores, AddAttributeStore, AttributeStore, AttributeStoreDiffOptions,
};
use super::change::Change;
use super::changes::trend_store::AddTrendStore;
use super::entity_set::{load_entity_sets, EntitySet};
use super::error::Error;
use super::notification_store::{
    load_notification_stores, AddNotificationStore, NotificationStore,
};
use super::relation::{load_relation_from_file, AddRelation, Relation};
use super::trend_materialization::{
    load_materializations, load_materializations_from, AddTrendMaterialization,
    TrendMaterialization,
};
use super::trend_store::{
    load_trend_store_from_file, load_trend_stores, TrendStore, TrendStoreDiffOptions,
};
use super::trigger::{load_trigger_from_file, load_triggers, AddTrigger, Trigger};
use super::virtual_entity::{
    load_virtual_entity_from_file, load_virtual_entity_from_yaml_file, AddVirtualEntity,
    VirtualEntity,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum AggregationType {
    #[serde(rename = "VIEW")]
    View,
    #[serde(rename = "VIEW_MATERIALIZATION")]
    ViewMaterialization,
    #[serde(rename = "FUNCTION_MATERIALIZATION")]
    FunctionMaterialization,
    #[serde(rename = "SUPPRESS")]
    Suppress,
}

pub struct DiffOptions {
    pub ignore_trend_extra_data: bool,
    pub ignore_trend_data_type: bool,
    pub ignore_deletions: bool,
    pub instance_ignores: Vec<DeploymentIgnore>,
    pub stage_deletions: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EntityAggregationHint {
    pub relation: String,
    pub materialization_type: AggregationType,
    pub prefix: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InstanceDockerImage {
    pub image_name: String,
    pub image_tag: String,
    pub path: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RetentionConfig {
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    #[serde(with = "humantime_serde")]
    pub retention_period: Duration,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PartitionSizeConfig {
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    #[serde(with = "humantime_serde")]
    pub partition_size: Duration,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum TrendChange {
    DataType,
    ExtraData,
    Remove,
    Add,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DeploymentIgnoreTrend {
    pub change: TrendChange,
    pub trend_store_part: String,
    pub trend_match_regex: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DeploymentIgnore {
    Trend(DeploymentIgnoreTrend),
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DeploymentConfig {
    pub ignore: Vec<DeploymentIgnore>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddAttribute {
    pub name: String,
    pub data_type: DataType,
    pub example: String,
    #[serde(default)]
    pub extra_data: Value,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AddAttributes {
    pub entity_type: String,
    pub description: String,
    pub attributes: Vec<AddAttribute>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct AttributeExtraction {
    pub data_source: String,
    pub add_attributes: Option<Vec<AddAttributes>>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct InstanceConfig {
    pub docker_image: Option<InstanceDockerImage>,
    pub deployment: Option<DeploymentConfig>,
    pub entity_aggregation_hints: Option<Vec<EntityAggregationHint>>,
    pub entity_types: Vec<String>,
    pub retention: Option<Vec<RetentionConfig>>,
    pub partition_size: Option<Vec<PartitionSizeConfig>>,
    #[serde(with = "humantime_serde::option", default = "default_duration_none")]
    pub old_data_threshold: Option<Duration>,
    #[serde(with = "humantime_serde::option")]
    pub old_data_stability_delay: Option<Duration>,
    pub attribute_extraction: AttributeExtraction,
}

fn default_duration_none() -> Option<Duration> {
    None
}

impl InstanceConfig {
    pub fn granularity_to_retention(&self, granularity: Duration) -> Option<Duration> {
        match &self.retention {
            Some(l) => l
                .iter()
                .find(|retention_config| retention_config.granularity.eq(&granularity))
                .map(|c| c.retention_period),
            None => None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InstanceConfigLoadError {
    #[error("No such config file '{0}'")]
    NoSuchFile(String),
    #[error("Could not open file: {0}")]
    FileOpen(#[from] std::io::Error),
    #[error("Could not deserialize config from file '{0}': {1}")]
    Deserialize(String, String),
}

pub fn load_instance_config(
    instance_root: &Path,
) -> Result<InstanceConfig, InstanceConfigLoadError> {
    let config_file_path = PathBuf::from_iter([instance_root, &PathBuf::from("config.json")]);

    if !config_file_path.is_file() {
        return Ok(InstanceConfig::default());
    }

    let config_file = std::fs::File::open(config_file_path.clone())?;
    let reader = BufReader::new(config_file);
    let image_config: InstanceConfig = serde_json::from_reader(reader).map_err(|e| {
        InstanceConfigLoadError::Deserialize(
            config_file_path.to_string_lossy().to_string(),
            format!("{e}"),
        )
    })?;

    Ok(image_config)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MinervaInstance {
    pub entity_types: Vec<EntityType>,
    pub trend_stores: Vec<TrendStore>,
    pub attribute_stores: Vec<AttributeStore>,
    pub notification_stores: Vec<NotificationStore>,
    pub virtual_entities: Vec<VirtualEntity>,
    pub relations: Vec<Relation>,
    pub trend_materializations: Vec<TrendMaterialization>,
    pub attribute_materializations: Vec<AttributeMaterialization>,
    pub triggers: Vec<Trigger>,
    pub entity_sets: Vec<EntitySet>,
}

pub async fn initialize_from<K, V>(
    instance_root: &Path,
    client: &mut Client,
    env: &[(K, V)],
) -> Result<(), Error>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    let minerva_instance = MinervaInstance::load_from(instance_root)?;

    initialize_custom(
        client,
        &format!("{}/custom/pre-init/**/*", instance_root.to_string_lossy()),
        env,
    )
    .await;

    initialize_entity_types(client, &minerva_instance.entity_types).await?;

    initialize_attribute_stores(client, &minerva_instance.attribute_stores).await?;

    initialize_trend_stores(client, &minerva_instance.trend_stores).await?;

    initialize_custom(
        client,
        &format!(
            "{}/custom/pre-notification-init/**/*",
            instance_root.to_string_lossy()
        ),
        env,
    )
    .await;

    initialize_notification_stores(client, &minerva_instance.notification_stores).await?;

    initialize_virtual_entities(client, &minerva_instance.virtual_entities).await?;

    initialize_custom(
        client,
        &format!(
            "{}/custom/pre-relation-init/**/*",
            instance_root.to_string_lossy()
        ),
        env,
    )
    .await;

    initialize_relations(client, &minerva_instance.relations).await?;

    initialize_custom(
        client,
        &format!(
            "{}/custom/pre-materialization-init/**/*",
            instance_root.to_string_lossy()
        ),
        env,
    )
    .await;

    initialize_trend_materializations(client, &minerva_instance.trend_materializations).await?;

    initialize_attribute_materializations(client, &minerva_instance.attribute_materializations)
        .await?;

    initialize_custom(
        client,
        &format!(
            "{}/custom/pre-trigger-init/**/*",
            instance_root.to_string_lossy()
        ),
        env,
    )
    .await;

    initialize_triggers(client, &minerva_instance.triggers).await?;

    initialize_custom(
        client,
        &format!("{}/custom/post-init/**/*", instance_root.to_string_lossy()),
        env,
    )
    .await;

    Ok(())
}

impl MinervaInstance {
    pub async fn load_from_db(client: &mut Client) -> Result<MinervaInstance, Error> {
        let entity_types = load_entity_types(client).await?;

        let attribute_stores = load_attribute_stores(client).await?;

        let trend_stores = load_trend_stores(client).await?;

        let notification_stores = load_notification_stores(client).await?;

        let virtual_entities = load_virtual_entities_from_db(client).await?;

        let relations = load_relations_from_db(client).await?;

        let trend_materializations = load_materializations(client)
            .await
            .map_err(|e| RuntimeError::from_msg(format!("Could not load materializations: {e}")))?;

        let attribute_materializations = load_attribute_materializations(client).await?;

        let triggers = load_triggers(client)
            .await
            .map_err(super::trigger::TriggerError::to_database_error)?;

        let entity_sets = load_entity_sets(client).await?;

        Ok(MinervaInstance {
            entity_types,
            trend_stores,
            attribute_stores,
            notification_stores,
            virtual_entities,
            relations,
            trend_materializations,
            attribute_materializations,
            triggers,
            entity_sets,
        })
    }

    pub fn load_from(minerva_instance_root: &Path) -> Result<MinervaInstance, String> {
        let entity_types = load_entity_types_from(minerva_instance_root).collect();
        let trend_stores = load_trend_stores_from(minerva_instance_root).collect();
        let notification_stores = load_notification_stores_from(minerva_instance_root).collect();
        let attribute_stores = load_attribute_stores_from(minerva_instance_root)
            .collect::<Result<Vec<AttributeStore>, String>>()?;
        let virtual_entities = load_virtual_entities_from(minerva_instance_root).collect();
        let relations = load_relations_from(minerva_instance_root).collect();
        let trend_materializations = load_materializations_from(minerva_instance_root).collect();
        let attribute_materializations =
            load_attribute_materializations_from(minerva_instance_root).collect();
        let triggers = load_triggers_from(minerva_instance_root).collect();
        let entity_sets: Vec<EntitySet> = vec![];

        Ok(MinervaInstance {
            entity_types,
            trend_stores,
            attribute_stores,
            notification_stores,
            virtual_entities,
            relations,
            trend_materializations,
            attribute_materializations,
            triggers,
            entity_sets,
        })
    }

    pub fn dependency_graph(&self) -> petgraph::Graph<GraphNode, String> {
        let mut graph = petgraph::Graph::new();

        let mut table_node_map: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

        for entity_type in &self.entity_types {
            let table_name = format!("entity.{}", entity_type.name);
            let node = GraphNode::Table(table_name.clone());
            let node_idx = graph.add_node(node);

            table_node_map.insert(table_name, node_idx);
        }

        for trend_store in &self.trend_stores {
            for trend_store_part in &trend_store.parts {
                let node = GraphNode::TrendStorePart(trend_store_part.name.clone());
                let node_idx = graph.add_node(node);

                table_node_map.insert(format!("trend.{}", &trend_store_part.name), node_idx);
            }
        }

        for attribute_store in &self.attribute_stores {
            let attribute_store_name = format!(
                "{}_{}",
                attribute_store.data_source, attribute_store.entity_type
            );
            let node = GraphNode::AttributeStore(attribute_store_name.clone());
            let node_idx = graph.add_node(node);

            table_node_map.insert(format!("attribute.{}", &attribute_store_name), node_idx);
        }

        for relation in &self.relations {
            let relation_node_index = graph.add_node(GraphNode::Relation(relation.name.clone()));

            table_node_map.insert(format!("relation.{}", &relation.name), relation_node_index);

            // Parse the SQL with the relation definition to find what tables it has as
            // dependencies
            match pg_query::parse(&relation.query) {
                Err(e) => {
                    error!("Could not parse SQL of relation '{}': {e}", relation.name);
                }
                Ok(parse_result) => {
                    for table_name in parse_result.tables() {
                        let source_index = match table_node_map.get(table_name.as_str()) {
                            None => {
                                let node = GraphNode::Table(table_name.clone());
                                let table_node_index = graph.add_node(node);
                                table_node_map.insert(table_name, table_node_index);
                                table_node_index
                            }
                            Some(index) => *index,
                        };

                        graph.add_edge(relation_node_index, source_index, "".to_string());
                    }
                }
            }
        }

        for trend_materialization in &self.trend_materializations {
            match trend_materialization {
                TrendMaterialization::View(m) => {
                    let materialization_node_idx = graph.add_node(
                        GraphNode::TrendViewMaterialization(m.target_trend_store_part.clone()),
                    );
                    let table_name = format!("trend.{}", m.target_trend_store_part);
                    let source_index = table_node_map.get(&table_name).unwrap();
                    graph.add_edge(*source_index, materialization_node_idx, "".to_string());

                    for source in &m.sources {
                        match source {
                            TrendMaterializationSource::Trend(trend_source) => {
                                let table_name = format!("trend.{}", trend_source.trend_store_part);
                                let target_index = table_node_map.get(&table_name).unwrap();
                                graph.add_edge(
                                    materialization_node_idx,
                                    *target_index,
                                    "".to_string(),
                                );
                            }
                            TrendMaterializationSource::Attribute(attribute_source) => {
                                let table_name =
                                    format!("attribute.{}", attribute_source.attribute_store);
                                match table_node_map.get(&table_name) {
                                    Some(target_index) => {
                                        graph.add_edge(
                                            materialization_node_idx,
                                            *target_index,
                                            "".to_string(),
                                        );
                                    }
                                    None => {
                                        println!(
                                            "Could not find attribute source table '{table_name}'"
                                        );
                                    }
                                }
                            }
                            TrendMaterializationSource::Relation(relation_source) => {
                                let table_name = format!("relation.{}", relation_source.relation);
                                match table_node_map.get(&table_name) {
                                    Some(target_index) => {
                                        graph.add_edge(
                                            materialization_node_idx,
                                            *target_index,
                                            "".to_string(),
                                        );
                                    }
                                    None => {
                                        println!(
                                            "Could not find relation source table '{table_name}'"
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                TrendMaterialization::Function(m) => {
                    let materialization_node_idx =
                        graph.add_node(GraphNode::TrendFunctionMaterialization(m.name()));
                    let table_name = format!("trend.{}", m.name());
                    let source_index = table_node_map.get(&table_name).unwrap();
                    graph.add_edge(*source_index, materialization_node_idx, "".to_string());

                    for source in &m.sources {
                        match source {
                            TrendMaterializationSource::Trend(trend_source) => {
                                let table_name = format!("trend.{}", trend_source.trend_store_part);
                                let target_index = table_node_map.get(&table_name).unwrap();
                                graph.add_edge(
                                    materialization_node_idx,
                                    *target_index,
                                    "".to_string(),
                                );
                            }
                            TrendMaterializationSource::Attribute(attribute_source) => {
                                let table_name =
                                    format!("attribute.{}", attribute_source.attribute_store);
                                match table_node_map.get(&table_name) {
                                    Some(target_index) => {
                                        graph.add_edge(
                                            materialization_node_idx,
                                            *target_index,
                                            "".to_string(),
                                        );
                                    }
                                    None => {
                                        println!(
                                            "Could not find attribute source table '{table_name}'"
                                        );
                                    }
                                }
                            }
                            TrendMaterializationSource::Relation(relation_source) => {
                                let table_name = format!("relation.{}", relation_source.relation);
                                match table_node_map.get(&table_name) {
                                    Some(target_index) => {
                                        graph.add_edge(
                                            materialization_node_idx,
                                            *target_index,
                                            "".to_string(),
                                        );
                                    }
                                    None => {
                                        println!(
                                            "Could not find relation source table '{table_name}'"
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for attribute_materialization in &self.attribute_materializations {
            let attribute_materialization_node_index =
                graph.add_node(GraphNode::AttributeMaterialization(
                    attribute_materialization.attribute_store.to_string(),
                ));

            let table_name = format!("attribute.{}", attribute_materialization.attribute_store);

            match table_node_map.get(&table_name) {
                Some(attribute_store_index) => {
                    graph.add_edge(
                        *attribute_store_index,
                        attribute_materialization_node_index,
                        "".to_string(),
                    );
                }
                None => {
                    error!(
                        "Could not find attribute store '{table_name}' for attribute materialization '{attribute_materialization}'"
                    );
                }
            }

            // Parse the SQL with the relation definition to find what tables it has as
            // dependencies
            match pg_query::parse(&attribute_materialization.query) {
                Err(e) => {
                    error!(
                        "Could not parse SQL of attribute materialization '{attribute_materialization}': {e}",
                    );
                }
                Ok(parse_result) => {
                    for table_name in parse_result.tables() {
                        let source_index = match table_node_map.get(table_name.as_str()) {
                            None => {
                                let node = GraphNode::Table(table_name.clone());
                                let table_node_index = graph.add_node(node);
                                table_node_map.insert(table_name, table_node_index);
                                table_node_index
                            }
                            Some(index) => *index,
                        };

                        graph.add_edge(
                            attribute_materialization_node_index,
                            source_index,
                            "".to_string(),
                        );
                    }
                }
            }
        }

        for virtual_entity in &self.virtual_entities {
            let virtual_entity_node_index =
                graph.add_node(GraphNode::VirtualEntity(virtual_entity.name.clone()));

            let table_name = format!("entity.{}", virtual_entity.name);

            match table_node_map.get(&table_name) {
                Some(entity_index) => {
                    graph.add_edge(*entity_index, virtual_entity_node_index, "".to_string());
                }
                None => {
                    error!(
                        "Could not find entity table '{}' for virtual entity '{}'",
                        table_name, virtual_entity.name
                    );
                }
            }

            // Parse the SQL with the relation definition to find what tables it has as
            // dependencies
            match pg_query::parse(&virtual_entity.sql) {
                Err(e) => {
                    error!(
                        "Could not parse SQL of virtual entity '{}': {e}",
                        virtual_entity.name
                    );
                }
                Ok(parse_result) => {
                    for table_name in parse_result.tables() {
                        let source_index = match table_node_map.get(table_name.as_str()) {
                            None => {
                                let node = GraphNode::Table(table_name.clone());
                                let table_node_index = graph.add_node(node);
                                table_node_map.insert(table_name, table_node_index);
                                table_node_index
                            }
                            Some(index) => *index,
                        };

                        graph.add_edge(virtual_entity_node_index, source_index, "".to_string());
                    }
                }
            }
        }

        graph
    }

    #[must_use]
    pub fn diff(
        &self,
        other: &MinervaInstance,
        options: DiffOptions,
    ) -> Vec<Box<dyn Change + Send + 'static>> {
        let mut changes: Vec<Box<dyn Change + Send>> = Vec::new();

        for other_entity_type in &other.entity_types {
            match self
                .entity_types
                .iter()
                .find(|my_entity_type| my_entity_type.name == other_entity_type.name)
            {
                Some(my_entity_type) => {
                    changes.append(&mut my_entity_type.diff(other_entity_type));
                }
                None => {
                    changes.push(Box::new(AddEntityType {
                        entity_type: other_entity_type.clone(),
                    }));
                }
            }
        }

        // Check for changes in trend stores
        for other_trend_store in &other.trend_stores {
            match self.trend_stores.iter().find(|my_trend_store| {
                my_trend_store.data_source == other_trend_store.data_source
                    && my_trend_store.entity_type == other_trend_store.entity_type
                    && my_trend_store.granularity == other_trend_store.granularity
            }) {
                Some(my_trend_store) => {
                    let diff_options = TrendStoreDiffOptions {
                        ignore_trend_extra_data: options.ignore_trend_extra_data,
                        ignore_trend_data_type: options.ignore_trend_data_type,
                        ignore_deletions: options.ignore_deletions,
                        instance_ignores: options.instance_ignores.clone(),
                        stage_deletions: options.stage_deletions,
                    };

                    changes.append(&mut my_trend_store.diff(other_trend_store, diff_options));
                }
                None => {
                    changes.push(Box::new(AddTrendStore {
                        trend_store: other_trend_store.clone(),
                    }));
                }
            }
        }

        // Check for trend stores to remove
        for my_trend_store in &self.trend_stores {
            match other.trend_stores.iter().find(|other_trend_store| {
                my_trend_store.data_source == other_trend_store.data_source
                    && my_trend_store.entity_type == other_trend_store.entity_type
                    && my_trend_store.granularity == other_trend_store.granularity
            }) {
                Some(_) => {
                    // Ok, still there
                }
                None => {
                    // No longer defined, so remove it
                    for trend_store_part in &my_trend_store.parts {
                        changes.push(Box::new(RemoveTrendStorePart {
                            name: trend_store_part.name.clone(),
                        }));
                    }

                    changes.push(Box::new(RemoveTrendStore {
                        trend_store: my_trend_store.into(),
                    }));
                }
            }
        }

        // Check for changes in attribute stores
        for other_attribute_store in &other.attribute_stores {
            match self.attribute_stores.iter().find(|my_attribute_store| {
                my_attribute_store.data_source == other_attribute_store.data_source
                    && my_attribute_store.entity_type.to_lowercase()
                        == other_attribute_store.entity_type.to_lowercase()
            }) {
                Some(my_attribute_store) => {
                    let attribute_store_diff_options = AttributeStoreDiffOptions {
                        ignore_deletions: options.ignore_deletions,
                    };
                    changes.append(
                        &mut my_attribute_store
                            .diff(other_attribute_store, attribute_store_diff_options),
                    );
                }
                None => {
                    changes.push(Box::new(AddAttributeStore {
                        attribute_store: other_attribute_store.clone(),
                    }));
                }
            }
        }

        // Check for changes in notification stores
        for other_notification_store in &other.notification_stores {
            match self.notification_stores.iter().find(|my_attribute_store| {
                my_attribute_store.data_source == other_notification_store.data_source
            }) {
                Some(my_attribute_store) => {
                    changes.append(&mut my_attribute_store.diff(other_notification_store));
                }
                None => {
                    changes.push(Box::new(AddNotificationStore {
                        notification_store: other_notification_store.clone(),
                    }));
                }
            }
        }

        // Check for changes in trend materializations
        for other_trend_materialization in &other.trend_materializations {
            match self
                .trend_materializations
                .iter()
                .find(|my_trend_materialization| {
                    my_trend_materialization.name() == other_trend_materialization.name()
                }) {
                Some(my_trend_materialization) => {
                    changes.append(&mut my_trend_materialization.diff(other_trend_materialization));
                }
                None => changes.push(Box::new(AddTrendMaterialization::from(
                    other_trend_materialization.clone(),
                ))),
            }
        }

        for my_trend_materialization in &self.trend_materializations {
            if !other
                .trend_materializations
                .iter()
                .any(|other_trend_materialization| {
                    other_trend_materialization.name() == my_trend_materialization.name()
                })
            {
                changes.push(Box::new(RemoveTrendMaterialization {
                    name: my_trend_materialization.name().to_string(),
                }))
            }
        }

        // Check for virtual entities to add
        for other_virtual_entity in &other.virtual_entities {
            match self
                .virtual_entities
                .iter()
                .find(|my_virtual_entity| my_virtual_entity.name == other_virtual_entity.name)
            {
                Some(_my_virtual_entity) => {
                    // changes.append(my_virtual_entity.diff(&other_virtual_entity));
                }
                None => {
                    changes.push(Box::new(AddVirtualEntity {
                        virtual_entity: other_virtual_entity.clone(),
                    }));
                }
            }
        }

        // Check for virtual entities to remove
        for my_virtual_entity in &self.virtual_entities {
            if !other
                .virtual_entities
                .iter()
                .any(|other_virtual_entity| other_virtual_entity.name == my_virtual_entity.name)
            {
                changes.push(Box::new(RemoveVirtualEntity {
                    name: my_virtual_entity.name.clone(),
                }))
            }
        }

        // Check for relations to add
        for other_relation in &other.relations {
            match self
                .relations
                .iter()
                .find(|my_relation| my_relation.name == other_relation.name)
            {
                Some(_my_relation) => {}
                None => {
                    changes.push(Box::new(AddRelation {
                        relation: other_relation.clone(),
                    }));
                }
            }
        }

        // Check for relations to remove
        for my_relation in &self.relations {
            if !other
                .relations
                .iter()
                .any(|other_relation| other_relation.name == my_relation.name)
            {
                changes.push(Box::new(RemoveRelation {
                    relation_name: my_relation.name.clone(),
                }))
            }
        }

        changes
    }

    pub async fn update(
        &self,
        client: &mut Client,
        other: &MinervaInstance,
        diff_options: DiffOptions,
    ) -> Result<(), Error> {
        let changes = self.diff(other, diff_options);

        println!("Applying changes:");

        for change in changes {
            println!("* {change}");

            match change.apply(client).await {
                Ok(message) => println!("> {}", &message),
                Err(err) => println!("! Error applying change: {}", &err),
            }
        }

        Ok(())
    }
}

pub async fn dump(client: &mut Client) {
    let minerva_instance: MinervaInstance = match MinervaInstance::load_from_db(client).await {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error loading instance from database: {e}");
            return;
        }
    };

    let json = &mut serde_json::Serializer::pretty(std::io::stdout());
    let mut serializer = Box::new(<dyn Serializer>::erase(json));
    let _ = minerva_instance.erased_serialize(&mut serializer);
}

fn load_attribute_stores_from(
    minerva_instance_root: &Path,
) -> impl Iterator<Item = Result<AttributeStore, String>> {
    let glob_path = format!(
        "{}/attribute/*.yaml",
        minerva_instance_root.to_string_lossy()
    );

    glob(&glob_path)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => {
                let f = std::fs::File::open(path.clone()).unwrap();
                let attribute_store = serde_yaml::from_reader(f).map_err(|e| {
                    format!(
                        "Could not load attribute store definition '{}': {e}",
                        path.to_string_lossy()
                    )
                });

                Some(attribute_store)
            }
            Err(_) => None,
        })
}

async fn initialize_entity_types(
    client: &mut Client,
    entity_types: &Vec<EntityType>,
) -> Result<(), Error> {
    for entity_type in entity_types {
        let change = AddEntityType {
            entity_type: entity_type.clone(),
        };

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

async fn initialize_attribute_stores(
    client: &mut Client,
    attribute_stores: &Vec<AttributeStore>,
) -> Result<(), Error> {
    for attribute_store in attribute_stores {
        let change = AddAttributeStore {
            attribute_store: attribute_store.clone(),
        };

        //let mut tx = client.transaction().await?;

        //tx.execute(
        //    "SET LOCAL citus.multi_shard_modify_mode TO 'sequential'",
        //    &[],
        //)
        //.await?;

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

fn load_notification_stores_from(
    minerva_instance_root: &Path,
) -> impl Iterator<Item = NotificationStore> {
    let glob_path = format!(
        "{}/notification/*.yaml",
        minerva_instance_root.to_string_lossy()
    );

    glob(&glob_path)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => {
                let f = std::fs::File::open(path).unwrap();
                let notification_store: NotificationStore = serde_yaml::from_reader(f).unwrap();

                Some(notification_store)
            }
            Err(_) => None,
        })
}

async fn initialize_notification_stores(
    client: &mut Client,
    notification_stores: &Vec<NotificationStore>,
) -> Result<(), Error> {
    for notification_store in notification_stores {
        let change = AddNotificationStore {
            notification_store: notification_store.clone(),
        };

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("Error creating notification store: {e}");
            }
        }
    }

    Ok(())
}

pub fn load_trend_stores_from(minerva_instance_root: &Path) -> impl Iterator<Item = TrendStore> {
    let yaml_paths = glob(&format!(
        "{}/trend/*.yaml",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    let json_paths = glob(&format!(
        "{}/trend/*.json",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    yaml_paths
        .chain(json_paths)
        .filter_map(|entry| match entry {
            Ok(path) => match load_trend_store_from_file(&path) {
                Ok(trend_store) => Some(trend_store),
                Err(e) => {
                    println!("Error loading trend store definition: {e}");
                    None
                }
            },
            Err(_) => None,
        })
}

async fn initialize_trend_stores(
    client: &mut Client,
    trend_stores: &Vec<TrendStore>,
) -> Result<(), Error> {
    for trend_store in trend_stores {
        let change = AddTrendStore {
            trend_store: trend_store.clone(),
        };

        //let mut tx = client.transaction().await?;

        //tx.execute(
        //    "SET LOCAL citus.multi_shard_modify_mode TO 'sequential'",
        //    &[],
        //)
        //.await?;

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

fn load_triggers_from(minerva_instance_root: &Path) -> impl Iterator<Item = Trigger> {
    let yaml_paths = glob(&format!(
        "{}/trigger/*.yaml",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    let json_paths = glob(&format!(
        "{}/trigger/*.json",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    yaml_paths
        .chain(json_paths)
        .filter_map(|entry| match entry {
            Ok(path) => match load_trigger_from_file(&path) {
                Ok(trend_store) => Some(trend_store),
                Err(e) => {
                    println!("Error loading trend store definition: {e}");
                    None
                }
            },
            Err(_) => None,
        })
}

fn load_virtual_entities_from(minerva_instance_root: &Path) -> impl Iterator<Item = VirtualEntity> {
    let sql_paths = glob(&format!(
        "{}/virtual-entity/*.sql",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    let from_sql_definitions = sql_paths.filter_map(|entry| match entry {
        Ok(path) => match load_virtual_entity_from_file(&path) {
            Ok(virtual_entity) => Some(virtual_entity),
            Err(e) => {
                println!("Error loading virtual entity definition: {e}");
                None
            }
        },
        Err(_) => None,
    });

    let yaml_paths = glob(&format!(
        "{}/virtual-entity/*.yaml",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    let from_yaml_definitions = yaml_paths.filter_map(|entry| match entry {
        Ok(path) => match load_virtual_entity_from_yaml_file(&path) {
            Ok(virtual_entity) => Some(virtual_entity),
            Err(e) => {
                println!("Error loading virtual entity definition: {e}");
                None
            }
        },
        Err(_) => None,
    });

    from_sql_definitions.chain(from_yaml_definitions)
}

fn load_relations_from(minerva_instance_root: &Path) -> impl Iterator<Item = Relation> {
    let yaml_paths = glob(&format!(
        "{}/relation/*.yaml",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    let json_paths = glob(&format!(
        "{}/relation/*.json",
        minerva_instance_root.to_string_lossy()
    ))
    .expect("Failed to read glob pattern");

    yaml_paths
        .chain(json_paths)
        .filter_map(|entry| match entry {
            Ok(path) => match load_relation_from_file(&path) {
                Ok(trend_store) => Some(trend_store),
                Err(e) => {
                    println!("Error loading relation definition: {e}");
                    None
                }
            },
            Err(_) => None,
        })
}

async fn initialize_virtual_entities(
    client: &mut Client,
    virtual_entities: &Vec<VirtualEntity>,
) -> Result<(), Error> {
    for virtual_entity in virtual_entities {
        let change: AddVirtualEntity = AddVirtualEntity::from(virtual_entity.clone());

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

async fn initialize_relations(client: &mut Client, relations: &Vec<Relation>) -> Result<(), Error> {
    for relation in relations {
        let change: AddRelation = AddRelation::from(relation.clone());

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

async fn initialize_attribute_materializations(
    client: &mut Client,
    attribute_materializations: &Vec<AttributeMaterialization>,
) -> Result<(), Error> {
    for materialization in attribute_materializations {
        let change = AddAttributeMaterialization::from(materialization.clone());

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
    Ok(())
}

async fn initialize_trend_materializations(
    client: &mut Client,
    trend_materializations: &Vec<TrendMaterialization>,
) -> Result<(), Error> {
    for materialization in trend_materializations {
        let change = AddTrendMaterialization::from(materialization.clone());

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }

    Ok(())
}

async fn initialize_triggers(client: &mut Client, triggers: &Vec<Trigger>) -> Result<(), Error> {
    for trigger in triggers {
        let change = AddTrigger {
            trigger: trigger.clone(),
            verify: false,
        };

        match change.apply(client).await {
            Ok(message) => {
                println!("{message}");
            }
            Err(e) => {
                println!("Error creating trigger '{}': {}", trigger.name, e);
            }
        }
    }

    Ok(())
}

async fn load_sql(client: &mut Client, path: &PathBuf) -> Result<(), String> {
    let mut f = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!(
                "Could not open sql file '{}': {}",
                &path.to_string_lossy(),
                e
            ))
        }
    };

    let mut sql = String::new();

    if let Err(e) = f.read_to_string(&mut sql) {
        return Err(format!(
            "Could not read virtual entity definition file: {e}"
        ));
    }

    if let Err(e) = client.batch_execute(&sql).await {
        return Err(format!("Error creating relation materialized view: {e}"));
    }

    Ok(())
}

fn load_psql(path: &PathBuf) -> Result<String, String> {
    let cmd = Command::new("psql").arg("-f").arg(path).output();

    match cmd {
        Ok(output) => match output.status.success() {
            true => {
                let stdout = std::str::from_utf8(&output.stderr).unwrap();

                Ok(stdout.into())
            }
            false => {
                let stderr = std::str::from_utf8(&output.stderr).unwrap();

                Err(stderr.into())
            }
        },
        Err(e) => Err(format!("Could not run psql command: {e}")),
    }
}

fn execute_custom<I, K, V>(path: &PathBuf, env: I) -> Result<String, String>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut cmd = Command::new(path);

    cmd.envs(env);

    let cmd_result = cmd.output();

    match cmd_result {
        Ok(output) => match output.status.success() {
            true => {
                let stdout = std::str::from_utf8(&output.stderr).unwrap();

                Ok(stdout.into())
            }
            false => {
                let stderr = std::str::from_utf8(&output.stderr).unwrap();

                Err(stderr.into())
            }
        },
        Err(e) => Err(format!("Could not run command: {e}")),
    }
}

async fn initialize_custom<'a, K, V>(client: &'a mut Client, glob_pattern: &'a str, env: &[(K, V)])
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    let paths = glob(glob_pattern).expect("Failed to read glob pattern");

    let envs = env.iter().collect::<Vec<_>>();

    for entry in paths {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    println!("Directory '{}'", &path.to_string_lossy());
                } else {
                    match path.extension() {
                        Some(ext) => {
                            let ext_str = ext.to_str().unwrap_or("");
                            match ext_str {
                                "sql" => match load_sql(client, &path).await {
                                    Ok(()) => {
                                        println!("Executed sql '{}'", &path.to_string_lossy())
                                    }
                                    Err(e) => {
                                        println!(
                                            "Error executing sql '{}': {}",
                                            &path.to_string_lossy(),
                                            e
                                        );
                                    }
                                },
                                "psql" => match load_psql(&path) {
                                    Ok(msg) => {
                                        println!(
                                            "Executed '{}' with psql: {}",
                                            &path.to_string_lossy(),
                                            msg
                                        );
                                    }
                                    Err(e) => {
                                        println!(
                                            "Error executing '{}' with psql: {}",
                                            &path.to_string_lossy(),
                                            e
                                        );
                                    }
                                },
                                _ => {
                                    let metadata_result = path.metadata();

                                    match metadata_result {
                                        Err(e) => {
                                            println!(
                                                "Error retrieving meta data for '{}': {}",
                                                &path.to_string_lossy(),
                                                e
                                            );
                                        }
                                        Ok(metadata) => {
                                            if (metadata.permissions().mode() & 0o111) != 0 {
                                                let env_pairs: Vec<(&str, &str)> = envs
                                                    .iter()
                                                    .map(|(k, v)| (k.as_ref(), v.as_ref()))
                                                    .collect();
                                                match execute_custom(&path, env_pairs) {
                                                    Ok(msg) => {
                                                        println!(
                                                            "Executed '{}': {}",
                                                            &path.to_string_lossy(),
                                                            msg
                                                        );
                                                    }
                                                    Err(e) => {
                                                        println!(
                                                            "Error executing '{}': {}",
                                                            &path.to_string_lossy(),
                                                            e
                                                        );
                                                    }
                                                }
                                            } else {
                                                println!(
                                                    "Skipping non-executable file '{}'",
                                                    path.to_string_lossy()
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            println!(
                                "A file without an extension should not have matched the glob patterns",
                            );
                        }
                    }
                }
            }
            Err(_) => println!("No path"),
        }
    }
}
