use async_trait::async_trait;
use postgres_protocol::escape::escape_identifier;
use std::fmt::{self, Display};
use std::{io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, GenericClient};

use crate::change::Changed;

use super::change::{Change, ChangeResult};
use super::error::{ConfigurationError, DatabaseError, Error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualEntity {
    pub name: String,
    pub sql: String,
}

impl fmt::Display for VirtualEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VirtualEntity({})", &self.name)
    }
}

pub fn load_virtual_entity_from_yaml_file(path: &PathBuf) -> Result<VirtualEntity, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open virtual entity definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    serde_yaml::from_reader(f).map_err(|e| {
        Error::Runtime(crate::error::RuntimeError::from_msg(format!(
            "Could not read virtual entity definition from file '{}': {}",
            path.display(),
            e
        )))
    })
}

pub fn load_virtual_entity_from_file(path: &PathBuf) -> Result<VirtualEntity, Error> {
    let mut f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open virtual entity definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let mut sql = String::new();

    f.read_to_string(&mut sql).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not read virtual entity definition file: {e}"
        ))
    })?;

    let name = path.file_name().unwrap().to_string_lossy().to_string();

    let virtual_entity = VirtualEntity { name, sql };

    Ok(virtual_entity)
}

pub async fn load_virtual_entity_from_db<T: GenericClient + Send + Sync>(
    conn: &mut T,
    name: &str,
) -> Result<VirtualEntity, String> {
    let query = concat!(
        "select ",
        "ev_class::regclass::text as name, ",
        "pg_get_viewdef(ev_class) as view_def ",
        "from pg_rewrite r ",
        "join pg_class c on c.oid = ev_class ",
        "join pg_namespace nsp on nsp.oid = c.relnamespace ",
        "where nspname = 'virtual_entity' and relname = $1"
    );

    let rows = conn.query(query, &[&name]).await.unwrap();

    if rows.is_empty() {
        return Err(format!("No such virtual entity '{name}'"));
    }

    let row = rows.first().unwrap();

    let virtual_entity = VirtualEntity {
        name: row.get(0),
        sql: row.get(1),
    };

    Ok(virtual_entity)
}

pub async fn load_virtual_entities_from_db<T: GenericClient + Send + Sync>(
    conn: &mut T,
) -> Result<Vec<VirtualEntity>, String> {
    let query = concat!(
        "select ",
        "c.relname as name, ",
        "pg_get_viewdef(ev_class) as view_def ",
        "from pg_rewrite r ",
        "join pg_class c on c.oid = ev_class ",
        "join pg_namespace nsp on nsp.oid = c.relnamespace ",
        "where nspname = 'virtual_entity'"
    );

    let rows = conn.query(query, &[]).await.unwrap();

    let virtual_entities = rows
        .iter()
        .map(|row| VirtualEntity {
            name: row.get(0),
            sql: row.get(1),
        })
        .collect();

    Ok(virtual_entities)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddVirtualEntity {
    pub virtual_entity: VirtualEntity,
}

impl fmt::Display for AddVirtualEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddVirtualEntity({})", &self.virtual_entity)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddVirtualEntity {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = format!(
            "CREATE OR REPLACE VIEW virtual_entity.{} AS {}",
            escape_identifier(&self.virtual_entity.name),
            self.virtual_entity.sql
        );

        tx.execute(&query, &[]).await.map_err(|e| {
            DatabaseError::from_msg(format!(
                "Error creating virtual entity '{}': {e}",
                &self.virtual_entity.name
            ))
        })?;

        tx.commit().await?;

        Ok(Box::new(AddedVirtualEntity {
            virtual_entity: self.virtual_entity.name.clone(),
        }))
    }
}

impl From<VirtualEntity> for AddVirtualEntity {
    fn from(virtual_entity: VirtualEntity) -> Self {
        AddVirtualEntity { virtual_entity }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedVirtualEntity {
    pub virtual_entity: String,
}

impl Display for AddedVirtualEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Added virtual entity {}", &self.virtual_entity)
    }
}

#[typetag::serde]
impl Changed for AddedVirtualEntity {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(RemoveVirtualEntity {
            name: self.virtual_entity.clone(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveVirtualEntity {
    pub name: String,
}

impl fmt::Display for RemoveVirtualEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoveVirtualEntity({})", &self.name)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveVirtualEntity {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let virtual_entity = load_virtual_entity_from_db(&mut tx, &self.name).await?;

        let query = format!("DROP VIEW virtual_entity.{}", escape_identifier(&self.name),);

        tx.execute(&query, &[]).await.map_err(|e| {
            DatabaseError::from_msg(format!(
                "Error removing virtual entity '{}': {e}",
                &self.name
            ))
        })?;

        tx.commit().await?;

        Ok(Box::new(RemovedVirtualEntity { virtual_entity }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedVirtualEntity {
    pub virtual_entity: VirtualEntity,
}

impl Display for RemovedVirtualEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Removed virtual entity {}", &self.virtual_entity)
    }
}

#[typetag::serde]
impl Changed for RemovedVirtualEntity {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(AddVirtualEntity {
            virtual_entity: self.virtual_entity.clone(),
        }))
    }
}
