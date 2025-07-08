use std::fmt;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use glob::glob;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient};

use crate::change::ChangeResult;

use super::change::Change;
use super::error::{ConfigurationError, Error, RuntimeError};

pub type EntityTypeName = String;

#[derive(Debug, Serialize, Deserialize, Clone, ToSql)]
pub struct EntityType {
    pub name: EntityTypeName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_alias: Option<String>,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntityType({})", &self.name)
    }
}

pub fn load_entity_type_from_file(path: &PathBuf) -> Result<EntityType, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open entity type definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    if path.extension() == Some(std::ffi::OsStr::new("yaml")) {
        let entity_type: EntityType = serde_yaml::from_reader(f).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not read entity type definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(entity_type)
    } else if path.extension() == Some(std::ffi::OsStr::new("json")) {
        let entity_type: EntityType = serde_json::from_reader(f).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not read entity type definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(entity_type)
    } else {
        return Err(ConfigurationError::from_msg(format!(
            "Unsupported entity type definition format '{}'",
            path.extension().unwrap().to_string_lossy()
        ))
        .into());
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddEntityType {
    pub entity_type: EntityType,
}

impl fmt::Display for AddEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddEntityType({})", &self.entity_type)
    }
}

#[async_trait]
impl Change for AddEntityType {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_entity_type(&mut tx, &self.entity_type)
            .await
            .map_err(|e| format!("Could not create entity type: {e}"))?;

        tx.commit().await?;
        Ok(format!("Created entity_type '{}'", &self.entity_type.name))
    }
}

impl From<EntityType> for AddEntityType {
    fn from(entity_type: EntityType) -> Self {
        AddEntityType { entity_type }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateEntityTypeError {
    #[error("{0}")]
    Database(String),
}

pub async fn create_entity_type<T: GenericClient>(
    client: &mut T,
    entity_type: &EntityType,
) -> Result<(), CreateEntityTypeError> {
    client
        .execute(
            "SELECT directory.create_entity_type($1, $2);",
            &[&entity_type.name, &entity_type.primary_alias],
        )
        .await
        .map(|_| ())
        .map_err(|e| CreateEntityTypeError::Database(format!("Error creating entity type: {e}")))
}

pub fn load_entity_types_from(minerva_instance_root: &Path) -> impl Iterator<Item = EntityType> {
    let yaml_paths = glob(&format!(
        "{}/entity-type/*.yaml",
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
            Ok(path) => match load_entity_type_from_file(&path) {
                Ok(trend_store) => Some(trend_store),
                Err(e) => {
                    println!("Error loading trend store definition: {e}");
                    None
                }
            },
            Err(_) => None,
        })
}

pub async fn load_entity_types<T: GenericClient + Send + Sync>(
    client: &mut T,
) -> Result<Vec<EntityType>, Error> {
    let query = "SELECT name, primary_alias FROM directory.entity_type";

    let rows = client.query(query, &[]).await?;

    let entity_types: Vec<EntityType> = rows
        .iter()
        .map(|row| EntityType {
            name: row.get(0),
            primary_alias: row.get(1),
        })
        .collect();

    Ok(entity_types)
}
