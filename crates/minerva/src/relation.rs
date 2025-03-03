use std::fmt;
use std::path::PathBuf;

use async_trait::async_trait;
use postgres_protocol::escape::escape_identifier;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_postgres::{Client, GenericClient, Transaction};

use crate::change::ChangeResult;

use super::change::Change;
use super::error::{ConfigurationError, DatabaseError, Error, RuntimeError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub name: String,
    pub query: String,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Relation({})", &self.name)
    }
}

pub fn load_relation_from_file(path: &PathBuf) -> Result<Relation, Error> {
    let f = std::fs::File::open(path).map_err(|e| {
        ConfigurationError::from_msg(format!(
            "Could not open relation definition file '{}': {}",
            path.display(),
            e
        ))
    })?;

    if path.extension() == Some(std::ffi::OsStr::new("yaml")) {
        let relation: Relation = serde_yaml::from_reader(f).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not read relation definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(relation)
    } else if path.extension() == Some(std::ffi::OsStr::new("json")) {
        let relation: Relation = serde_json::from_reader(f).map_err(|e| {
            RuntimeError::from_msg(format!(
                "Could not read relation definition from file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Ok(relation)
    } else {
        return Err(ConfigurationError::from_msg(format!(
            "Unsupported relation definition format '{}'",
            path.extension().unwrap().to_string_lossy()
        ))
        .into());
    }
}

pub struct AddRelation {
    pub relation: Relation,
}

impl fmt::Display for AddRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddRelation({})", &self.relation)
    }
}

#[async_trait]
impl Change for AddRelation {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_relation(&mut tx, &self.relation)
            .await
            .map_err(|e| format!("Could not create relation '{}': {e}", self.relation.name))?;

        tx.commit().await?;

        Ok(format!("Added relation '{}'", &self.relation))
    }
}

impl From<Relation> for AddRelation {
    fn from(relation: Relation) -> Self {
        AddRelation { relation }
    }
}

pub struct UpdateRelation {
    pub relation: Relation,
}

impl fmt::Display for UpdateRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UpdateRelation({})", &self.relation)
    }
}

#[async_trait]
impl Change for UpdateRelation {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = format!(
            "CREATE OR REPLACE VIEW relation_def.\"{}\" AS {}",
            self.relation.name, self.relation.query
        );

        tx.query(&query, &[])
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error updating relation view: {e}")))?;

        tx.commit().await?;

        Ok(format!("Updated relation {}", &self.relation))
    }
}

impl From<Relation> for UpdateRelation {
    fn from(relation: Relation) -> Self {
        UpdateRelation { relation }
    }
}

#[derive(Error, Debug)]
pub enum MaterializeRelationError {
    #[error("Could not delete current relations: {source}")]
    Delete {
        #[source]
        source: tokio_postgres::Error,
    },
    #[error("Could not insert new relations: {source}")]
    Insert {
        #[source]
        source: tokio_postgres::Error,
    },
}

pub struct MaterializeRelationResult {
    pub deleted_count: u64,
    pub inserted_count: u64,
}

pub async fn materialize_relation(
    tx: &mut Transaction<'_>,
    name: &str,
) -> Result<MaterializeRelationResult, MaterializeRelationError> {
    let delete_query = format!("DELETE FROM relation.{}", escape_identifier(name));

    let deleted_count = tx
        .execute(&delete_query, &[])
        .await
        .map_err(|e| MaterializeRelationError::Delete { source: e })?;

    let insert_query = format!(
        "INSERT INTO relation.{}(source_id, target_id) SELECT source_id, target_id FROM relation_def.{}",
        escape_identifier(name),
        escape_identifier(name)
    );

    let inserted_count = tx
        .execute(&insert_query, &[])
        .await
        .map_err(|e| MaterializeRelationError::Insert { source: e })?;

    Ok(MaterializeRelationResult {
        deleted_count,
        inserted_count,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum CreateRelationError {
    #[error("{0}")]
    Database(String),
}

pub async fn create_relation<T: GenericClient>(
    client: &mut T,
    relation: &Relation,
) -> Result<(), CreateRelationError> {
    let query = format!(
        "CREATE TABLE relation.\"{}\"(source_id integer, target_id integer)",
        relation.name
    );
    client.query(&query, &[]).await.map_err(|e| {
        CreateRelationError::Database(format!("Error creating relation table: {e}"))
    })?;

    let query = format!(
        "CREATE VIEW relation_def.\"{}\" AS {}",
        relation.name, relation.query
    );

    client
        .query(&query, &[])
        .await
        .map_err(|e| CreateRelationError::Database(format!("Error creating relation view: {e}")))?;

    let query = format!(
        "CREATE UNIQUE INDEX ON relation.\"{}\"(source_id, target_id)",
        relation.name
    );

    client.query(&query, &[]).await.map_err(|e| {
        CreateRelationError::Database(format!("Error creating index on relation table: {e}"))
    })?;

    let query = format!("CREATE INDEX ON relation.\"{}\"(target_id)", relation.name);

    client.query(&query, &[]).await.map_err(|e| {
        CreateRelationError::Database(format!("Error creating index on relation table: {e}"))
    })?;

    // Make the table available on each of the Citus nodes.
    let query = format!(
        "SELECT create_reference_table('relation.\"{}\"')",
        relation.name
    );

    client.query(&query, &[]).await.map_err(|e| {
        CreateRelationError::Database(format!(
            "Error converting relation table to reference table: {e}"
        ))
    })?;

    let query = "SELECT relation_directory.register_type($1)";

    client
        .query_one(query, &[&relation.name])
        .await
        .map_err(|e| CreateRelationError::Database(format!("Error registering relation: {e}")))?;

    Ok(())
}
