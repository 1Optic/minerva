use std::collections::HashMap;
use std::fmt;
use std::future::Future;

use async_trait::async_trait;
use postgres_protocol::escape::escape_identifier;
use quick_cache::sync::Cache;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient};

use super::change::{Change, ChangeResult};

#[derive(Error, Debug)]
pub enum EntityMappingError {
    #[error("Database error: {0}")]
    DatabaseError(tokio_postgres::Error),
    #[error("Could not create entity: {0}")]
    EntityCreationError(tokio_postgres::Error),
    #[error("Could not insert entity")]
    EntityInsertError,
    #[error("Could not map entity")]
    UnmappedEntityError,
    #[error("Value unexpectedly not found in cache")]
    CacheError,
}

type EntityTypeName = String;
type EntityName = String;

#[derive(Debug, Serialize, Deserialize, Clone, ToSql)]
pub struct EntityType {
    name: EntityTypeName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    primary_alias: Option<String>,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntityType({})", &self.name)
    }
}

pub trait EntityMapping {
    fn uses_alias_column<T: GenericClient + Sync>(
        &self, entity_type: &EntityTypeName, client: &T
    ) -> impl Future<Output = Result<bool, EntityMappingError>> + Send;

    fn names_to_entity_ids<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> impl Future<Output = Result<Vec<i32>, EntityMappingError>> + Send;

    fn names_to_aliases<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> impl Future<Output = Result<Vec<Option<String>>, EntityMappingError>> + Send;
}

pub struct DbEntityMapping {}

impl EntityMapping for DbEntityMapping {
    async fn uses_alias_column<T: GenericClient + Sync>(&self, entity_type: &EntityTypeName, client: &T) -> Result<bool, EntityMappingError> {
        let query = "SELECT primary_alias FROM directory.entity_type WHERE name = $1";
        let query_result = client
            .query_one(query, &[&entity_type])
            .await
            .map_err(EntityMappingError::DatabaseError)?;
        let result: bool = query_result.get(0);
        Ok(result)
    }

    async fn names_to_entity_ids<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> Result<Vec<i32>, EntityMappingError> {
        let mut entity_ids: HashMap<String, i32> = HashMap::new();

        let query = format!(
            "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
            SELECT l.name, e.id FROM lookup_list l \
            LEFT JOIN entity.{} e ON l.name = e.name ",
            escape_identifier(entity_type)
        );

        let rows = client
            .query(&query, &[&names])
            .await
            .map_err(EntityMappingError::DatabaseError)?;

        for row in rows {
            let name: String = row.get(0);
            let entity_id_value: Option<i32> =
                row.try_get(1).map_err(EntityMappingError::DatabaseError)?;
            let entity_id: i32 = match entity_id_value {
                Some(entity_id) => entity_id,
                None => create_entity(client, entity_type, &name).await?,
            };

            entity_ids.insert(name, entity_id);
        }

        names
            .into_iter()
            .map(|name| -> Result<i32, EntityMappingError> {
                entity_ids
                    .get(name)
                    .copied()
                    .ok_or(EntityMappingError::UnmappedEntityError)
            })
            .collect()
    }

    async fn names_to_aliases<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> Result<Vec<Option<String>>, EntityMappingError> {

        let query = "SELECT primary_alias FROM directory.entity_type WHERE name = $1";
        let result = client
            .query_one(query, &[&entity_type])
            .await
            .map_err(EntityMappingError::DatabaseError)?;
        
        let has_primary_alias: bool = result.get(0);
        match has_primary_alias {
            true => {
                // Ensure that all entities actually exist in the database
                self.names_to_entity_ids(client, entity_type, &names).await?;

                let mut aliases: HashMap<String, Option<String>> = HashMap::new();
                let query = format!(
                    "WITH lookup_list AS (SELECT unnest($1::integer[]) AS id) \
                    SELECT l.name, e.primary_alias FROM lookup_list l \
                    LEFT JOIN entity.{} e ON l.name = e.name ",
                    escape_identifier(entity_type)
                );

                let rows = client
                    .query(&query, &[&names])
                    .await
                    .map_err(EntityMappingError::DatabaseError)?;

                for row in rows {
                    let name: String = row.get(0);
                    let alias: Option<String> =
                        Some(row.try_get::<usize, String>(1).map_err(EntityMappingError::DatabaseError)?);
                    aliases.insert(name, alias);
                }

                names
                    .into_iter()
                    .map(|name| -> Result<Option<String>, EntityMappingError> {
                        aliases
                            .get(name)
                            .cloned()
                            .ok_or(EntityMappingError::UnmappedEntityError)
                    })
                    .collect()
            },
            false => {
                names.iter().map(|_| -> Result<Option<String>, EntityMappingError> {Ok(None)}).collect()
            }
        }

    }
}

pub struct CachingEntityMapping {
    id_cache: Cache<(EntityTypeName, EntityName), i32>,
    alias_cache: Cache<(EntityTypeName, EntityName), String>,
    primary_alias_cache: Cache<EntityTypeName, bool>,
}

impl CachingEntityMapping {
    #[must_use]
    pub fn new(size: usize) -> Self {
        CachingEntityMapping {
            id_cache: Cache::new(size),
            alias_cache: Cache::new(size),
            primary_alias_cache: Cache::new(size),
        }
    }
}

impl EntityMapping for CachingEntityMapping {
    async fn uses_alias_column<T: GenericClient + Sync>(&self, entity_type: &EntityTypeName, client: &T) -> Result<bool, EntityMappingError> {
        if self.primary_alias_cache.get(entity_type) == None {
            let query = "SELECT primary_alias FROM directory.entity_type WHERE name = $1";
            let result = client
                .query_one(query, &[&entity_type])
                .await
                .map_err(EntityMappingError::DatabaseError)?;
            let primary_alias: bool = result.get(0);
            
            self.primary_alias_cache.insert(entity_type.to_string(), primary_alias);
        };
        self.primary_alias_cache.get(entity_type).ok_or(EntityMappingError::CacheError)
    }

    async fn names_to_entity_ids<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> Result<Vec<i32>, EntityMappingError> {
        let mut entity_ids: HashMap<String, i32> = HashMap::new();

        let query = format!(
            "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
            SELECT l.name, e.id FROM lookup_list l \
            LEFT JOIN entity.{} e ON l.name = e.name ",
            escape_identifier(entity_type)
        );

        let mut names_list: Vec<&str> = Vec::new();

        for name in names {
            if let Some(entity_id) = self
                .id_cache
                .get(&(entity_type.to_string(), String::from(name)))
            {
                entity_ids.insert(name.clone(), entity_id);
            } else {
                names_list.push(name.as_ref());
            }
        }

        // Only lookup in the database if there is anything left to lookup
        if !names_list.is_empty() {
            let rows = client
                .query(&query, &[&names_list])
                .await
                .map_err(EntityMappingError::DatabaseError)?;

            for row in rows {
                let name: String = row.get(0);
                let entity_id_value: Option<i32> =
                    row.try_get(1).map_err(EntityMappingError::DatabaseError)?;
                let entity_id: i32 = match entity_id_value {
                    Some(entity_id) => entity_id,
                    None => create_entity(client, entity_type, &name).await?,
                };

                self.id_cache
                    .insert((entity_type.to_string(), name.clone()), entity_id);

                entity_ids.insert(name, entity_id);
            }
        }

        names
            .into_iter()
            .map(|name| -> Result<i32, EntityMappingError> {
                entity_ids
                    .get(name)
                    .copied()
                    .ok_or(EntityMappingError::UnmappedEntityError)
            })
            .collect()
    }

    async fn names_to_aliases<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &Vec<EntityName>,
    ) -> Result<Vec<Option<String>>, EntityMappingError> {
        match &self.uses_alias_column(entity_type, client).await? {
            true => {
                // Ensure that all entities actually exist in the database
                self.names_to_entity_ids(client, entity_type, &names).await?;

                let mut aliases: HashMap<String, String> = HashMap::new();

                let query = format!(
                    "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
                    SELECT l.name, e.primary_alias FROM lookup_list l \
                    LEFT JOIN entity.{} e ON l.name = e.name ",
                    escape_identifier(entity_type)
                );

                let mut names_list: Vec<&str> = Vec::new();

                for name in names {
                    if let Some(alias) = self
                        .alias_cache
                        .get(&(entity_type.to_string(), String::from(name)))
                    {
                        aliases.insert(name.clone(), alias);
                    } else {
                        names_list.push(name.as_ref());
                    }
                }

                // Only lookup in the database if there is anything left to lookup
                if !names_list.is_empty() {
                    let rows = client
                        .query(&query, &[&names_list])
                        .await
                        .map_err(EntityMappingError::DatabaseError)?;

                    for row in rows {
                        let name: String = row.get(0);
                        let alias: String = row.get(1);

                        self.alias_cache
                            .insert((entity_type.to_string(), name.clone()), alias.clone());

                        aliases.insert(name, alias);

                    }
                }

                names
                    .into_iter()
                    .map(|name| -> Result<Option<String>, EntityMappingError> {
                        Ok(aliases
                            .get(name)
                            .cloned())
                    })
                    .collect()
            },
            false => names.iter().map(|_| -> Result<Option<String>, EntityMappingError> {Ok(None)}).collect()
        }

    }
}

async fn create_entity<T: GenericClient>(
    client: &T,
    entity_type_table: &str,
    name: &str,
) -> Result<i32, EntityMappingError> {
    let query = format!(
        "INSERT INTO entity.{}(name) VALUES($1) ON CONFLICT(name) DO UPDATE SET name=EXCLUDED.name RETURNING id",
        escape_identifier(entity_type_table)
    );

    let rows = client
        .query(&query, &[&name])
        .await
        .map_err(EntityMappingError::DatabaseError)?;

    match rows.first() {
        Some(row) => row
            .try_get(0)
            .map_err(EntityMappingError::EntityCreationError),
        None => Err(EntityMappingError::EntityInsertError),
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
        let tx = client.transaction().await?;

        // TODO: This has not been implemented yet

        Ok(format!("Created entity_type '{}'", &self.entity_type.name))
    }
}
