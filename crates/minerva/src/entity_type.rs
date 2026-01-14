use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use glob::glob;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient};

use crate::change::{ChangeResult, Changed};

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

impl EntityType {
    pub fn diff(&self, other: &EntityType) -> Vec<Box<dyn Change + Send>> {
        match &self.primary_alias {
            None => match &other.primary_alias {
                None => Vec::new(),
                Some(other_primary_alias) => vec![Box::new(AddPrimaryAlias {
                    entity_type: self.name.clone(),
                    primary_alias: other_primary_alias.to_string(),
                })],
            },
            Some(my_primary_alias) => match &other.primary_alias {
                None => vec![Box::new(RemovePrimaryAlias {
                    entity_type: self.name.clone(),
                })],
                Some(other_primary_alias) => {
                    if other_primary_alias != my_primary_alias {
                        vec![Box::new(ChangePrimaryAlias {
                            entity_type: self.name.clone(),
                            primary_alias: other_primary_alias.to_string(),
                        })]
                    } else {
                        Vec::new()
                    }
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddPrimaryAlias {
    pub entity_type: String,
    pub primary_alias: String,
}

impl fmt::Display for AddPrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "AddPrimaryAlias({}, {}):",
            &self.entity_type, &self.primary_alias
        )?;

        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddPrimaryAlias {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let transaction = client.transaction().await?;

        transaction
            .execute(
                "UPDATE directory.entity_type SET primary_alias = $1 WHERE name = $2",
                &[&self.primary_alias, &self.entity_type],
            )
            .await?;

        let query = format!("ALTER TABLE entity.\"{}\" ADD COLUMN primary_alias text GENERATED ALWAYS AS ({}) STORED", &self.entity_type, &self.primary_alias);

        transaction.execute(&query, &[]).await?;

        transaction.commit().await?;

        Ok(Box::new(AddedPrimaryAlias {
            entity_type: self.entity_type.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedPrimaryAlias {
    pub entity_type: String,
}

impl Display for AddedPrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Added primary alias to entity type '{}'",
            self.entity_type
        )
    }
}

#[typetag::serde]
impl Changed for AddedPrimaryAlias {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemovePrimaryAlias {
            entity_type: self.entity_type.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovePrimaryAlias {
    pub entity_type: String,
}

impl fmt::Display for RemovePrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RemovePrimaryAlias({}):", &self.entity_type)?;

        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemovePrimaryAlias {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let transaction = client.transaction().await?;

        let row = transaction
            .query_one(
                "SELECT primary_alias FROM directory.entity_type WHERE name = $1",
                &[&self.entity_type],
            )
            .await?;

        let primary_alias: String = row.get(0);

        transaction
            .execute(
                "UPDATE directory.entity_type SET primary_alias = NULL WHERE name = $1",
                &[&self.entity_type],
            )
            .await?;

        let query = format!(
            "ALTER TABLE entity.\"{}\" DROP COLUMN primary_alias",
            &self.entity_type
        );

        transaction.execute(&query, &[]).await?;

        transaction.commit().await?;

        Ok(Box::new(RemovedPrimaryAlias {
            entity_type: self.entity_type.clone(),
            primary_alias,
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedPrimaryAlias {
    pub entity_type: String,
    pub primary_alias: String,
}

impl Display for RemovedPrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Removed primary alias from entity type '{}'",
            self.entity_type
        )
    }
}

#[typetag::serde]
impl Changed for RemovedPrimaryAlias {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddPrimaryAlias {
            entity_type: self.entity_type.clone(),
            primary_alias: self.primary_alias.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangePrimaryAlias {
    pub entity_type: String,
    pub primary_alias: String,
}

impl fmt::Display for ChangePrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ChangePrimaryAlias({}, {}):",
            &self.entity_type, &self.primary_alias
        )?;

        Ok(())
    }
}

#[async_trait]
#[typetag::serde]
impl Change for ChangePrimaryAlias {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let transaction = client.transaction().await?;

        transaction
            .execute(
                "UPDATE directory.entity_type SET primary_alias = $1 WHERE name = $2",
                &[&self.primary_alias, &self.entity_type],
            )
            .await?;

        let query = format!(
            "ALTER TABLE entity.\"{}\" ALTER COLUMN primary_alias SET EXPRESSION AS ({})",
            &self.entity_type, &self.primary_alias
        );

        transaction.execute(&query, &[]).await?;

        transaction.commit().await?;

        Ok(Box::new(ChangedPrimaryAlias {
            entity_type: self.entity_type.clone(),
            new_primary_alias: self.primary_alias.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ChangedPrimaryAlias {
    pub entity_type: String,
    pub new_primary_alias: String,
}

impl Display for ChangedPrimaryAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Changed primary alias of entity type '{}'",
            self.entity_type
        )
    }
}

#[typetag::serde]
impl Changed for ChangedPrimaryAlias {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
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
        Err(ConfigurationError::from_msg(format!(
            "Unsupported entity type definition format '{}'",
            path.extension().unwrap().to_string_lossy()
        ))
        .into())
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
#[typetag::serde]
impl Change for AddEntityType {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_entity_type(&mut tx, &self.entity_type)
            .await
            .map_err(|e| format!("Could not create entity type: {e}"))?;

        tx.commit().await?;

        Ok(Box::new(AddedEntityType {
            entity_type: self.entity_type.name.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedEntityType {
    pub entity_type: String,
}

impl Display for AddedEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Created entity_type '{}'", &self.entity_type)
    }
}

#[typetag::serde]
impl Changed for AddedEntityType {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
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
