use std::path::Path;
use std::fmt;

use serde::{Deserialize, Serialize};
use glob::glob;
use postgres_protocol::escape::escape_identifier;
use tokio_postgres::{GenericClient, Transaction};
use async_trait::async_trait;

use crate::attribute_store::materialize::AttributeStoreRef;

use super::change::{Change, ChangeResult};
use super::error::{Error, RuntimeError};

pub const MATERIALIZATION_VIEW_SCHEMA: &str = "attribute";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeMaterializationTarget {
    pub data_source: String,
    pub entity_type: String,
}

impl fmt::Display for AttributeMaterializationTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", &self.data_source, &self.entity_type)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeMaterialization {
    pub attribute_store: AttributeMaterializationTarget,
    pub query: String,
}

impl fmt::Display for AttributeMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AttributeMaterialization('{}')", &self.attribute_store)
    }
}

impl AttributeMaterialization {
    pub async fn create<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), String> {
        self.create_view(client).await?;
        self.define_materialization(client).await?;
        Ok(())
    }

    pub async fn create_view<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), String> {
        let query = format!(
            "CREATE VIEW {}.{} AS {}",
            MATERIALIZATION_VIEW_SCHEMA,
            &escape_identifier(&attribute_materialization_view_name(&self.attribute_store)),
            self.query,
        );

        match client.execute(query.as_str(), &[]).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error creating view: {e}"
            )),
        }
    }

    pub async fn define_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), String> {
        let query = "INSERT INTO attribute_directory.sampled_view_materialization(attribute_store_id, src_view) VALUES ($1, $2)";
        let attribute_store_name = format!("{}_{}", self.attribute_store.data_source, self.attribute_store.entity_type);
        let attribute_store_ref = AttributeStoreRef::from_name(client, &attribute_store_name).await.map_err(|e|format!("{e}"))?;
        let src_view = format!("{}.{}", MATERIALIZATION_VIEW_SCHEMA, &escape_identifier(&attribute_materialization_view_name(&self.attribute_store)));
        
        client.execute(query, &[&attribute_store_ref.id, &src_view]).await.map_err(|e| format!("Could not insert record for attribute materialization: {e}"))?; 

        Ok(())
    }
}

pub fn attribute_materialization_view_name(target_attribute_store: &AttributeMaterializationTarget) -> String {
    format!("_{}", target_attribute_store)
}

pub fn load_attribute_materializations_from(
    minerva_instance_root: &Path,
) -> impl Iterator<Item = AttributeMaterialization> {
    let glob_path = format!(
        "{}/attribute/materialization/*.yaml",
        minerva_instance_root.to_string_lossy()
    );

    glob(&glob_path)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| match entry {
            Ok(path) => match attribute_materialization_from_config(&path) {
                Ok(materialization) => Some(materialization),
                Err(e) => {
                    println!("Error loading attribute materialization '{}': {}", &path.display(), e);
                    None
                }
            },
            Err(_) => None,
        })
}

pub fn attribute_materialization_from_config(
    path: &std::path::PathBuf,
) -> Result<AttributeMaterialization, String> {
    let f = std::fs::File::open(path).map_err(|e| format!("could not open definition file: {e}"))?;
    let deserialize_result: Result<AttributeMaterialization, serde_yaml::Error> =
        serde_yaml::from_reader(f);

    match deserialize_result {
        Ok(materialization) => Ok(materialization),
        Err(e) => Err(format!("could not deserialize materialization: {e}")),
    }
}

pub async fn load_attribute_materializations<T: GenericClient + Send + Sync>(
    conn: &mut T,
) -> Result<Vec<AttributeMaterialization>, String> {
    let query = concat!(
        "SELECT ds.name, et.name, src_view ",
        "FROM attribute_directory.sampled_view_materialization svm ",
        "JOIN attribute_directory.attribute_store ast ON ast.id = svm.attribute_store_id ",
        "JOIN directory.data_source AS ds ON ds.id = ast.data_source_id ",
        "JOIN directory.entity_type AS et ON et.id = ast.entity_type_id"
    );
    
    let rows = conn.query(query, &[]).await.map_err(|e| format!("Could not load attribute materializations: {e}"))?;
    
    let attribute_materializations = rows.iter().map(|row| AttributeMaterialization { attribute_store: AttributeMaterializationTarget {data_source: row.get(0), entity_type: row.get(1)}, query: row.get(2)}).collect();

    Ok(attribute_materializations)
}

pub struct AddAttributeMaterialization {
    pub attribute_materialization: AttributeMaterialization,
}

impl fmt::Display for AddAttributeMaterialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddAttributeMaterialization({})",
            &self.attribute_materialization
        )
    }
}

#[async_trait]
impl Change for AddAttributeMaterialization {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        match self.attribute_materialization.create(client).await {
            Ok(_) => Ok(format!(
                "Added attribute materialization '{}'",
                &self.attribute_materialization
            )),
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "Error adding attribute materialization '{}': {}",
                    &self.attribute_materialization, e
                ),
            })),
        }
    }
}

impl From<AttributeMaterialization> for AddAttributeMaterialization {
    fn from(attribute_materialization: AttributeMaterialization) -> Self {
        AddAttributeMaterialization {
            attribute_materialization,
        }
    }
}
