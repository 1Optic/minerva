use async_trait::async_trait;
use std::fmt;
use std::{io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

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

    serde_yaml::from_reader(f).map_err(|e| Error::Runtime(crate::error::RuntimeError::from_msg(format!("Could not read virtual entity definition from file '{}': {}", path.display(), e))))
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
impl Change for AddVirtualEntity {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        tx.batch_execute(&self.virtual_entity.sql)
            .await
            .map_err(|e| {
                DatabaseError::from_msg(format!(
                    "Error creating virtual entity '{}': {e}",
                    &self.virtual_entity.name
                ))
            })?;

        tx.commit().await?;

        Ok(format!("Added virtual entity {}", &self.virtual_entity))
    }
}

impl From<VirtualEntity> for AddVirtualEntity {
    fn from(virtual_entity: VirtualEntity) -> Self {
        AddVirtualEntity { virtual_entity }
    }
}
