use std::fmt::{self, Display};
use std::path::PathBuf;

use async_trait::async_trait;
use postgres_protocol::escape::escape_identifier;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_postgres::{Client, GenericClient, Transaction};

use crate::change::{ChangeResult, Changed};
use crate::error::postgres_error_to_string;

use super::change::Change;
use super::error::{ConfigurationError, DatabaseError, Error, RuntimeError};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
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
        Err(ConfigurationError::from_msg(format!(
            "Unsupported relation definition format '{}'",
            path.extension().unwrap().to_string_lossy()
        ))
        .into())
    }
}

pub async fn load_relation_from_db<T: GenericClient + Send + Sync>(
    conn: &mut T,
    name: &str,
) -> Result<Relation, String> {
    let query = concat!(
        "select ",
        "relname, ",
        "pg_get_viewdef(ev_class) ",
        "from pg_rewrite r ",
        "join pg_class c on c.oid = ev_class ",
        "join pg_namespace nsp on nsp.oid = c.relnamespace ",
        "where nspname = 'relation_def' and relname = $1"
    );

    let rows = conn.query(query, &[&name]).await.unwrap();

    if rows.is_empty() {
        return Err(format!("No such relation '{name}'"));
    }

    let row = rows.first().unwrap();

    let relation = Relation {
        name: row.get(0),
        query: row.get(1),
    };

    Ok(relation)
}

pub async fn load_relations_from_db<T: GenericClient + Send + Sync>(
    conn: &mut T,
) -> Result<Vec<Relation>, String> {
    let query = concat!(
        "select ",
        "relname, ",
        "pg_get_viewdef(ev_class) ",
        "from pg_rewrite r ",
        "join pg_class c on c.oid = ev_class ",
        "join pg_namespace nsp on nsp.oid = c.relnamespace ",
        "where nspname = 'relation_def'"
    );

    let rows = conn.query(query, &[]).await.unwrap();

    let relations = rows
        .iter()
        .map(|row| Relation {
            name: row.get(0),
            query: row.get(1),
        })
        .collect();

    Ok(relations)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddRelation {
    pub relation: Relation,
}

impl fmt::Display for AddRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AddRelation({})", &self.relation)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for AddRelation {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        create_relation(&mut tx, &self.relation)
            .await
            .map_err(|e| format!("Could not create relation '{}': {e}", self.relation.name))?;

        tx.commit().await?;

        Ok(Box::new(AddedRelation {
            relation_name: self.relation.name.clone(),
        }))
    }
}

impl From<Relation> for AddRelation {
    fn from(relation: Relation) -> Self {
        AddRelation { relation }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AddedRelation {
    pub relation_name: String,
}

impl Display for AddedRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Added relation '{}'", &self.relation_name)
    }
}

#[typetag::serde]
impl Changed for AddedRelation {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UpdateRelation {
    pub relation: Relation,
}

impl fmt::Display for UpdateRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UpdateRelation({})", &self.relation)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for UpdateRelation {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let tx = client.transaction().await?;

        let query = format!(
            "CREATE OR REPLACE VIEW relation_def.{} AS {}",
            escape_identifier(&self.relation.name),
            self.relation.query
        );

        tx.query(&query, &[])
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error updating relation view: {e}")))?;

        tx.commit().await?;

        Ok(Box::new(UpdatedRelation {
            relation_name: self.relation.name.clone(),
        }))
    }
}

impl From<Relation> for UpdateRelation {
    fn from(relation: Relation) -> Self {
        UpdateRelation { relation }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UpdatedRelation {
    pub relation_name: String,
}

impl Display for UpdatedRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Updated relation {}", &self.relation_name)
    }
}

#[typetag::serde]
impl Changed for UpdatedRelation {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
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

#[derive(thiserror::Error, Debug)]
pub enum RemoveRelationError {
    #[error("{0}")]
    Database(String),
    #[error("No such relation registration")]
    NoSuchRegistration,
}

impl RemoveRelationError {
    fn from_postgres_error(msg: &str, e: tokio_postgres::Error) -> RemoveRelationError {
        RemoveRelationError::Database(format!("{msg}: {}", postgres_error_to_string(e)))
    }
}

pub async fn remove_relation<T: GenericClient>(
    client: &mut T,
    relation_name: &str,
) -> Result<(), RemoveRelationError> {
    let query = format!(
        "DROP TABLE IF EXISTS relation.{}",
        escape_identifier(relation_name),
    );

    client.query(&query, &[]).await.map_err(|e| {
        RemoveRelationError::from_postgres_error("Error dropping relation table", e)
    })?;

    let query = format!(
        "DROP VIEW IF EXISTS relation_def.{}",
        escape_identifier(relation_name)
    );

    client
        .query(&query, &[])
        .await
        .map_err(|e| RemoveRelationError::from_postgres_error("Error dropping relation view", e))?;

    let query = "DELETE FROM relation_directory.type WHERE name = $1";

    let delete_count = client
        .execute(query, &[&relation_name])
        .await
        .map_err(|e| RemoveRelationError::from_postgres_error("Error unregistering relation", e))?;

    if delete_count == 0 {
        return Err(RemoveRelationError::NoSuchRegistration);
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemoveRelation {
    pub relation_name: String,
}

impl fmt::Display for RemoveRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RemoveRelation({})", &self.relation_name)
    }
}

#[async_trait]
#[typetag::serde]
impl Change for RemoveRelation {
    async fn apply(&self, client: &mut Client) -> ChangeResult {
        let mut tx = client.transaction().await?;

        let relation = load_relation_from_db(&mut tx, &self.relation_name).await?;

        remove_relation(&mut tx, &self.relation_name)
            .await
            .map_err(|e| format!("Could not remove relation '{}': {e}", self.relation_name))?;

        tx.commit().await?;

        Ok(Box::new(RemovedRelation { relation }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RemovedRelation {
    pub relation: Relation,
}

impl Display for RemovedRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Removed relation '{}'", &self.relation.name)
    }
}

#[typetag::serde]
impl Changed for RemovedRelation {
    fn revert(&self) -> Option<Box<dyn Change>> {
        None
    }
}
