use std::collections::HashMap;
use std::future::Future;

use postgres_protocol::escape::escape_identifier;
use quick_cache::sync::Cache;
use thiserror::Error;
use tokio_postgres::GenericClient;

use super::entity_type::EntityTypeName;

type EntityName = String;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: i64,
    pub name: String,
    pub alias: Option<String>,
}

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

pub trait EntityMapping {
    fn uses_alias_column<T: GenericClient + Sync>(
        &self,
        entity_type: &EntityTypeName,
        client: &T,
    ) -> impl Future<Output = Result<bool, EntityMappingError>> + Send;

    fn names_to_entity_ids<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> impl Future<Output = Result<Vec<i64>, EntityMappingError>> + Send;

    fn names_to_aliases<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> impl Future<Output = Result<Vec<Option<String>>, EntityMappingError>> + Send;

    fn names_to_entities<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> impl Future<Output = Result<Vec<Entity>, EntityMappingError>> + Send;
}

pub struct DbEntityMapping {}

impl EntityMapping for DbEntityMapping {
    async fn uses_alias_column<T: GenericClient + Sync>(
        &self,
        entity_type: &EntityTypeName,
        client: &T,
    ) -> Result<bool, EntityMappingError> {
        let query = "SELECT primary_alias IS NOT NULL FROM directory.entity_type WHERE name = $1";
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
        names: &[EntityName],
    ) -> Result<Vec<i64>, EntityMappingError> {
        let mut entity_ids: HashMap<String, i64> = HashMap::new();

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
            let entity_id_value: Option<i64> =
                row.try_get(1).map_err(EntityMappingError::DatabaseError)?;
            let entity_id: i64 = match entity_id_value {
                Some(entity_id) => entity_id,
                None => create_entity(client, entity_type, &name).await?,
            };

            entity_ids.insert(name, entity_id);
        }

        names
            .iter()
            .map(|name| -> Result<i64, EntityMappingError> {
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
        names: &[EntityName],
    ) -> Result<Vec<Option<String>>, EntityMappingError> {
        let entities = self.names_to_entities(client, entity_type, names).await?;
        entities.iter().map(|e| Ok(e.alias.clone())).collect()
    }

    async fn names_to_entities<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> Result<Vec<Entity>, EntityMappingError> {
        let mut entities: HashMap<String, Entity> = HashMap::new();

        let primary_alias_query = "SELECT primary_alias FROM directory.entity_type WHERE name = $1";
        let primary_alias_result = client
            .query_one(primary_alias_query, &[&entity_type])
            .await
            .map_err(EntityMappingError::DatabaseError)?;

        let primary_alias: bool = primary_alias_result.get(0);

        let query = if primary_alias {
            format!(
                "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
                SELECT l.name, e.id, e.primary_alias FROM lookup_list l \
                LEFT JOIN entity.{} e ON l.name = e.name ",
                escape_identifier(entity_type)
            )
        } else {
            format!(
                "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
                SELECT l.name, e.id AS primary_alias FROM lookup_list l \
                LEFT JOIN entity.{} e ON l.name = e.name ",
                escape_identifier(entity_type)
            )
        };

        let rows = client
            .query(&query, &[&names])
            .await
            .map_err(EntityMappingError::DatabaseError)?;

        for row in rows {
            let name: String = row.get(0);
            let entity_id_value: Option<i64> =
                row.try_get(1).map_err(EntityMappingError::DatabaseError)?;
            let entity: Entity = match entity_id_value {
                Some(entity_id) => {
                    let alias: Option<String> = if primary_alias {
                        Some(
                            row.try_get::<usize, String>(2)
                                .map_err(EntityMappingError::DatabaseError)?,
                        )
                    } else {
                        None
                    };
                    Entity {
                        id: entity_id,
                        name: name.clone(),
                        alias,
                    }
                }
                None => create_entity_with_alias(client, entity_type, &name).await?,
            };

            entities.insert(name, entity);
        }

        Ok(entities.into_values().collect())
    }
}

pub struct CachingEntityMapping {
    id_cache: Cache<(EntityTypeName, EntityName), i64>,
    alias_cache: Cache<(EntityTypeName, EntityName), Option<String>>,
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
    async fn uses_alias_column<T: GenericClient + Sync>(
        &self,
        entity_type: &EntityTypeName,
        client: &T,
    ) -> Result<bool, EntityMappingError> {
        if self.primary_alias_cache.get(entity_type).is_none() {
            let query =
                "SELECT primary_alias IS NOT NULL FROM directory.entity_type WHERE name = $1";
            let result = client
                .query_one(query, &[&entity_type])
                .await
                .map_err(EntityMappingError::DatabaseError)?;
            let primary_alias: bool = result.get(0);

            self.primary_alias_cache
                .insert(entity_type.to_string(), primary_alias);
        };
        self.primary_alias_cache
            .get(entity_type)
            .ok_or(EntityMappingError::CacheError)
    }

    async fn names_to_entity_ids<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> Result<Vec<i64>, EntityMappingError> {
        let mut entity_ids: HashMap<String, i64> = HashMap::new();

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
                let entity_id_value: Option<i64> =
                    row.try_get(1).map_err(EntityMappingError::DatabaseError)?;
                let entity_id: i64 = match entity_id_value {
                    Some(entity_id) => entity_id,
                    None => create_entity(client, entity_type, &name).await?,
                };

                self.id_cache
                    .insert((entity_type.to_string(), name.clone()), entity_id);

                entity_ids.insert(name, entity_id);
            }
        }

        names
            .iter()
            .map(|name| -> Result<i64, EntityMappingError> {
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
        names: &[EntityName],
    ) -> Result<Vec<Option<String>>, EntityMappingError> {
        match &self.uses_alias_column(entity_type, client).await? {
            true => {
                // Ensure that all entities actually exist in the database
                self.names_to_entity_ids(client, entity_type, names).await?;

                let mut aliases: HashMap<String, String> = HashMap::new();

                let query = format!(
                    "WITH lookup_list AS (SELECT unnest($1::text[]) AS name) \
                    SELECT l.name, e.primary_alias FROM lookup_list l \
                    LEFT JOIN entity.{} e ON l.name = e.name ",
                    escape_identifier(entity_type)
                );

                let mut names_list: Vec<&str> = Vec::new();

                for name in names {
                    if let Some(Some(alias)) = self
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
                            .insert((entity_type.to_string(), name.clone()), Some(alias.clone()));

                        aliases.insert(name, alias);
                    }
                }

                names
                    .iter()
                    .map(|name| -> Result<Option<String>, EntityMappingError> {
                        Ok(aliases.get(name).cloned())
                    })
                    .collect()
            }
            false => names
                .iter()
                .map(|_| -> Result<Option<String>, EntityMappingError> { Ok(None) })
                .collect(),
        }
    }

    async fn names_to_entities<T: GenericClient + Sync>(
        &self,
        client: &T,
        entity_type: &EntityTypeName,
        names: &[EntityName],
    ) -> Result<Vec<Entity>, EntityMappingError> {
        let mut entities: HashMap<String, Entity> = HashMap::new();

        let mut names_list: Vec<String> = Vec::new();

        for name in names {
            if let Some(entity) = self
                .id_cache
                .get(&(entity_type.to_string(), String::from(name)))
            {
                if let Some(alias) = self
                    .alias_cache
                    .get(&(entity_type.to_string(), String::from(name)))
                {
                    let entity = Entity {
                        id: entity,
                        name: name.clone(),
                        alias: alias.clone(),
                    };
                    entities.insert(name.clone(), entity);
                    continue;
                } else {
                    // If we have the ID but not the alias, we need to fetch the full entity from the database
                    names_list.push(name.to_string());
                }
            } else {
                names_list.push(name.to_string());
            }
        }

        // Only lookup in the database if there is anything left to lookup
        if !names_list.is_empty() {
            let query = "WITH data as (
                SELECT entity.get_existing_entities(et, $1) AS entity \
                FROM directory.entity_type et WHERE et.name = $2) \
                SELECT (data.entity).id::integer, (data.entity).name, (data.entity).alias FROM data"
                .to_string();

            let rows = client
                .query(&query, &[&names_list, &entity_type])
                .await
                .map_err(EntityMappingError::DatabaseError)?;

            for row in rows {
                let id: i64 = row.get(0);
                let name: String = row.get(1);
                let alias: Option<String> = row.get(2);
                entities.insert(name.clone(), Entity { id, name, alias });
            }

            let missing_entities: Vec<&str> = names_list
                .iter()
                .filter(|name| !entities.contains_key(*name))
                .map(String::as_str)
                .collect();

            if !missing_entities.is_empty() {
                let query = "WITH lookup_list AS (SELECT unnest($1::text[]) AS name), \
                    data AS (SELECT entity.get_entity(et, l.name) AS entity FROM lookup_list l, directory.entity_type et WHERE et.name = $2)
                    SELECT (data.entity).id::integer, (data.entity).name, (data.entity).alias FROM data".to_string();

                let rows = client
                    .query(&query, &[&missing_entities, &entity_type])
                    .await
                    .map_err(EntityMappingError::DatabaseError)?;

                for row in rows {
                    let id: i64 = row.get(0);
                    let name: String = row.get(1);
                    let alias: Option<String> = row.get(2);
                    entities.insert(name.clone(), Entity { id, name, alias });
                }
            }
        }

        names
            .iter()
            .map(|name| -> Result<Entity, EntityMappingError> {
                entities
                    .get(name)
                    .cloned()
                    .ok_or(EntityMappingError::UnmappedEntityError)
            })
            .collect()
    }
}

async fn create_entity<T: GenericClient>(
    client: &T,
    entity_type_table: &str,
    name: &str,
) -> Result<i64, EntityMappingError> {
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

async fn create_entity_with_alias<T: GenericClient>(
    client: &T,
    entity_type_table: &str,
    name: &str,
) -> Result<Entity, EntityMappingError> {
    let query = format!(
        "INSERT INTO entity.{}(name, primary_alias) VALUES($1, $2) ON CONFLICT(name) DO UPDATE SET name=EXCLUDED.name RETURNING id, primary_alias",
        escape_identifier(entity_type_table)
    );

    let primary_alias = format!("alias_for_{}", name);

    let rows = client
        .query(&query, &[&name, &primary_alias])
        .await
        .map_err(EntityMappingError::DatabaseError)?;

    match rows.first() {
        Some(row) => {
            let id: i64 = row
                .try_get(0)
                .map_err(EntityMappingError::EntityCreationError)?;
            let alias: String = row
                .try_get(1)
                .map_err(EntityMappingError::EntityCreationError)?;
            Ok(Entity {
                id,
                name: name.to_string(),
                alias: Some(alias),
            })
        }
        None => Err(EntityMappingError::EntityInsertError),
    }
}
