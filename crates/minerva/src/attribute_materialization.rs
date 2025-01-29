use std::fmt;
use std::path::Path;

use async_trait::async_trait;
use glob::glob;
use postgres_protocol::escape::escape_identifier;
use serde::{Deserialize, Serialize};
use tokio_postgres::{GenericClient, Transaction};

use super::change::{Change, ChangeResult};
use super::error::{Error, RuntimeError};
use crate::attribute_store::load_attribute_names;
use crate::attribute_store::materialize_curr_ptr::{
    materialize_curr_ptr_by_name, MaterializeCurrPtrError, MaterializeCurrPtrResult,
};

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

    pub fn view_name(&self) -> String {
        attribute_materialization_view_name(&self.attribute_store)
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
            Err(e) => Err(format!("Error creating view: {e}")),
        }
    }

    pub async fn define_materialization<T: GenericClient + Send + Sync>(
        &self,
        client: &mut T,
    ) -> Result<(), String> {
        let query = "INSERT INTO attribute_directory.sampled_view_materialization(attribute_store_id, src_view) VALUES ($1, $2)";
        let attribute_store_name = format!(
            "{}_{}",
            self.attribute_store.data_source, self.attribute_store.entity_type
        );
        let attribute_store_ref = AttributeStoreRef::from_name(client, &attribute_store_name)
            .await
            .map_err(|e| format!("{e}"))?;
        let src_view = format!(
            "{}.{}",
            MATERIALIZATION_VIEW_SCHEMA,
            &escape_identifier(&attribute_materialization_view_name(&self.attribute_store))
        );

        client
            .execute(query, &[&attribute_store_ref.id, &src_view])
            .await
            .map_err(|e| format!("Could not insert record for attribute materialization: {e}"))?;

        Ok(())
    }

    pub async fn materialize_attribute<T: GenericClient + Send + Sync>(
        &self,
        client: &T,
    ) -> Result<MaterializeAttributeResult, AttributeMaterializeError> {
        let attribute_store =
            AttributeStoreRef::from_name(client, &self.attribute_store.to_string())
                .await
                .map_err(|e| {
                    AttributeMaterializeError::Unexpected(format!(
                        "Could not load attribute store reference: {e}"
                    ))
                })?;

        let mut attribute_names = load_attribute_names(client, attribute_store.id)
            .await
            .map_err(AttributeMaterializeError::Unexpected)?;
        let mut columns: Vec<String> = vec!["entity_id", "timestamp"]
            .into_iter()
            .map(String::from)
            .collect();
        columns.append(&mut attribute_names);
        let columns_part = columns
            .iter()
            .map(|name| escape_identifier(name))
            .collect::<Vec<String>>()
            .join(",");

        let query = format!(
            "TRUNCATE TABLE attribute_staging.\"{}\"",
            attribute_store.name
        );

        client.execute(&query, &[]).await.map_err(|e| {
            AttributeMaterializeError::Unexpected(format!("Could not truncate staging table: {e}"))
        })?;

        let query = format!(
            "INSERT INTO attribute_staging.\"{}\"({}) SELECT {} FROM attribute.\"{}\"",
            attribute_store.name,
            columns_part,
            columns_part,
            self.view_name()
        );

        let staged_record_count = client.execute(&query, &[]).await.map_err(|e| {
            AttributeMaterializeError::Unexpected(format!(
                "Could not materialize attribute data: {e}"
            ))
        })?;
        let query = format!(
            "INSERT INTO attribute_history.\"{}\"({}) SELECT {} FROM attribute_staging.\"{}\"",
            attribute_store.name, columns_part, columns_part, attribute_store.name
        );

        let record_count = client.execute(&query, &[]).await.map_err(|e| {
            AttributeMaterializeError::Unexpected(format!(
                "Could not materialize attribute data: {e}"
            ))
        })?;

        let query = "SELECT attribute_directory.mark_modified(ast.id) FROM attribute_directory.attribute_store ast WHERE ast::text = $1";

        client
            .execute(query, &[&attribute_store.name])
            .await
            .map_err(|e| {
                AttributeMaterializeError::Unexpected(format!(
                    "Could not mark attribute store modified: {e}"
                ))
            })?;

        let curr_ptr_result = materialize_curr_ptr_by_name(client, &attribute_store.name).await?;

        Ok(MaterializeAttributeResult {
            staged_record_count,
            materialized_record_count: record_count,
            materialized_curr_ptr: curr_ptr_result,
        })
    }
}

pub fn attribute_materialization_view_name(
    target_attribute_store: &AttributeMaterializationTarget,
) -> String {
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
                    println!(
                        "Error loading attribute materialization '{}': {}",
                        &path.display(),
                        e
                    );
                    None
                }
            },
            Err(_) => None,
        })
}

pub fn attribute_materialization_from_config(
    path: &std::path::PathBuf,
) -> Result<AttributeMaterialization, String> {
    let f =
        std::fs::File::open(path).map_err(|e| format!("could not open definition file: {e}"))?;
    let deserialize_result: Result<AttributeMaterialization, serde_yaml::Error> =
        serde_yaml::from_reader(f);

    match deserialize_result {
        Ok(materialization) => Ok(materialization),
        Err(e) => Err(format!("could not deserialize materialization: {e}")),
    }
}

pub async fn load_attribute_materialization_by_id<T: GenericClient + Send + Sync>(
    conn: &T,
    attribute_materialization_id: i32,
) -> Result<AttributeMaterialization, String> {
    let query = concat!(
        "SELECT ds.name, et.name, pg_get_viewdef(src_view) ",
        "FROM attribute_directory.sampled_view_materialization svm ",
        "JOIN attribute_directory.attribute_store ast ON ast.id = svm.attribute_store_id ",
        "JOIN directory.data_source AS ds ON ds.id = ast.data_source_id ",
        "JOIN directory.entity_type AS et ON et.id = ast.entity_type_id ",
        "WHERE svm.id = $1"
    );

    let rows = conn
        .query(query, &[&attribute_materialization_id])
        .await
        .map_err(|e| format!("Could not load attribute materialization: {e}"))?;

    if rows.is_empty() {
        return Err(format!(
            "No materialization found matching Id {attribute_materialization_id}"
        ));
    }

    let row = rows.first().unwrap();

    Ok(AttributeMaterialization {
        attribute_store: AttributeMaterializationTarget {
            data_source: row.get(0),
            entity_type: row.get(1),
        },
        query: row.get(2),
    })
}

pub async fn load_attribute_materialization_by_name<T: GenericClient + Send + Sync>(
    conn: &T,
    name: &str,
) -> Result<AttributeMaterialization, String> {
    let query = concat!(
        "SELECT ds.name, et.name, pg_get_viewdef(src_view) ",
        "FROM attribute_directory.sampled_view_materialization svm ",
        "JOIN attribute_directory.attribute_store ast ON ast.id = svm.attribute_store_id ",
        "JOIN directory.data_source AS ds ON ds.id = ast.data_source_id ",
        "JOIN directory.entity_type AS et ON et.id = ast.entity_type_id ",
        "WHERE svm::text = $1"
    );

    let rows = conn
        .query(query, &[&name])
        .await
        .map_err(|e| format!("Could not load attribute materialization: {e}"))?;

    if rows.is_empty() {
        return Err(format!("No materialization found matching name '{name}'"));
    }

    let row = rows.first().unwrap();

    Ok(AttributeMaterialization {
        attribute_store: AttributeMaterializationTarget {
            data_source: row.get(0),
            entity_type: row.get(1),
        },
        query: row.get(2),
    })
}

pub async fn load_attribute_materializations<T: GenericClient + Send + Sync>(
    conn: &T,
) -> Result<Vec<AttributeMaterialization>, String> {
    let query = concat!(
        "SELECT ds.name, et.name, pg_get_viewdef(src_view) ",
        "FROM attribute_directory.sampled_view_materialization svm ",
        "JOIN attribute_directory.attribute_store ast ON ast.id = svm.attribute_store_id ",
        "JOIN directory.data_source AS ds ON ds.id = ast.data_source_id ",
        "JOIN directory.entity_type AS et ON et.id = ast.entity_type_id"
    );

    let rows = conn
        .query(query, &[])
        .await
        .map_err(|e| format!("Could not load attribute materializations: {e}"))?;

    let attribute_materializations = rows
        .iter()
        .map(|row| AttributeMaterialization {
            attribute_store: AttributeMaterializationTarget {
                data_source: row.get(0),
                entity_type: row.get(1),
            },
            query: row.get(2),
        })
        .collect();

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

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum AttributeMaterializeError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Could not materialize curr ptr data: {0}")]
    CurrPtrMaterialization(#[from] MaterializeCurrPtrError),
}

#[derive(thiserror::Error, Debug)]
pub enum AttributeStoreRefError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("No attribute store matching name '{0}'")]
    NoAttributeStoreWithName(String),
    #[error("No attribute store matching id '{0}'")]
    NoAttributeStoreWithId(i32),
}

pub struct AttributeStoreRef {
    pub id: i32,
    pub name: String,
}

impl Display for AttributeStoreRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'({})", self.name, self.id)
    }
}

impl AttributeStoreRef {
    pub async fn from_name<T: GenericClient + Send + Sync>(
        client: &T,
        name: &str,
    ) -> Result<AttributeStoreRef, AttributeStoreRefError> {
        let query = "SELECT id FROM attribute_directory.attribute_store ast WHERE ast::text = $1";
        let rows = client
            .query(query, &[&name])
            .await
            .map_err(|e| AttributeStoreRefError::Unexpected(format!("{e}")))?;

        if rows.is_empty() {
            return Err(AttributeStoreRefError::NoAttributeStoreWithName(
                name.to_string(),
            ));
        }

        let row = rows.first().unwrap();

        let attribute_store_id: i32 = row.get(0);

        let attribute_store_ref = AttributeStoreRef {
            id: attribute_store_id,
            name: name.to_string(),
        };

        Ok(attribute_store_ref)
    }

    pub async fn from_id<T: GenericClient + Send + Sync>(
        client: &T,
        attribute_store_id: i32,
    ) -> Result<AttributeStoreRef, AttributeStoreRefError> {
        let query = "SELECT ast::text FROM attribute_directory.attribute_store ast WHERE id = $1";
        let rows = client
            .query(query, &[&attribute_store_id])
            .await
            .map_err(|e| AttributeStoreRefError::Unexpected(format!("{e}")))?;

        if rows.is_empty() {
            return Err(AttributeStoreRefError::NoAttributeStoreWithId(
                attribute_store_id,
            ));
        }

        let row = rows.first().unwrap();

        let name: String = row.get(0);

        let attribute_store_ref = AttributeStoreRef {
            id: attribute_store_id,
            name,
        };

        Ok(attribute_store_ref)
    }
}

pub struct MaterializeAttributeResult {
    pub staged_record_count: u64,
    pub materialized_record_count: u64,
    pub materialized_curr_ptr: MaterializeCurrPtrResult,
}
