use std::fmt;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use tokio_postgres::{Client, Transaction};

use async_trait::async_trait;

use super::change::{Change, ChangeResult};
use super::error::{DatabaseError, DatabaseErrorKind, Error, RuntimeError};

type PostgresName = String;
use log::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntitySet {
    pub id: i32,
    pub name: PostgresName,
    pub group: String,
    pub entity_type: String,
    pub owner: String,
    pub description: String,
    pub entities: Vec<String>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEntitySet {
    pub name: PostgresName,
    pub group: String,
    pub entity_type: String,
    pub owner: String,
    pub description: String,
    pub entities: Vec<String>,
}

pub enum EntitySetError {
    DatabaseError(DatabaseError),
    NotFound(DatabaseError),
    ExistingEntitySet(String, String),
    EmptyEntitySet,
    MissingEntities(Vec<String>),
    UnchangeableFields(Vec<String>),
}

impl From<DatabaseError> for EntitySetError {
    fn from(e: DatabaseError) -> EntitySetError {
        EntitySetError::DatabaseError(e)
    }
}

impl From<String> for EntitySetError {
    fn from(e: String) -> EntitySetError {
        EntitySetError::DatabaseError(DatabaseError {
            msg: e,
            kind: DatabaseErrorKind::Default,
        })
    }
}

impl fmt::Display for EntitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntitySet({}:{})", &self.owner, &self.name,)
    }
}

impl fmt::Display for NewEntitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntitySet({}:{})", &self.owner, &self.name,)
    }
}

async fn get_entity_set_members(conn: &mut Client, id: i32) -> Result<Vec<String>, String> {
    let query = "SELECT relation_directory.get_entity_set_members($1)";
    let row = conn
        .query_one(query, &[&id])
        .await
        .map_err(|e| format!("{e}"))?;
    Ok(row.get(0))
}

pub async fn load_entity_sets(conn: &mut Client) -> Result<Vec<EntitySet>, String> {
    let query = concat!(
        "SELECT name, \"group\", source_entity_type, owner, description, ",
        "id, first_appearance, modified ",
        "FROM attribute.minerva_entity_set es"
    );

    let rows = conn
        .query(query, &[])
        .await
        .map_err(|e| format!("Error loading entity sets: {e}"))?;

    let mut entity_sets: Vec<EntitySet> = vec![];

    for row in rows {
        let entities = get_entity_set_members(conn, row.get(5))
            .await
            .map_err(|e| format!("Error loading entity set content: {e}"))?;

        entity_sets.push(EntitySet {
            id: row.get(5),
            name: row.get(0),
            group: row.get(1),
            entity_type: row.get(2),
            owner: row.get(3),
            description: row.try_get(4).unwrap_or("".into()),
            entities,
            created: row.get(6),
            modified: row.get(7),
        })
    }

    Ok(entity_sets)
}

pub async fn load_entity_set(
    conn: &mut Client,
    owner: &str,
    name: &str,
) -> Result<EntitySet, String> {
    let query = concat!(
        "SELECT name, \"group\", source_entity_type, owner, description, ",
        "entity_set.get_entity_set_members(es.id), first_appearance, modified, id ",
        "FROM attribute.minerva_entity_set es ",
        "WHERE es.owner = $1 AND es.name = $2"
    );

    let row = conn
        .query_one(query, &[&owner, &name])
        .await
        .map_err(|e| format!("Could not load entity set {owner}:{name}: {e}"))?;

    let entity_set = EntitySet {
        id: row.get(8),
        name: row.get(0),
        group: row.get(1),
        entity_type: row.get(2),
        owner: row.get(3),
        description: row.try_get(4).unwrap_or("".into()),
        entities: row.get(5),
        created: row.get(6),
        modified: row.get(7),
    };

    Ok(entity_set)
}

impl EntitySet {
    pub async fn update(&self, conn: &mut Transaction<'_>) -> Result<EntitySet, EntitySetError> {
        let row = conn
            .query_one(
                "SELECT name, owner, source_entity_type FROM attribute.minerva_entity_set WHERE id = $1",
                &[&self.id],
            )
            .await
            .map_err(|e| EntitySetError::NotFound(DatabaseError::from_msg(e.to_string())))?;

        let mut incorrect_fields: Vec<String> = vec![];
        let foundname: String = row.get(0);
        if self.name != foundname {
            incorrect_fields.push("name".to_string())
        };
        let foundowner: String = row.get(1);
        if self.owner != foundowner {
            incorrect_fields.push("owner".to_string())
        };
        let foundentitytype: String = row.get(2);
        if self.entity_type != foundentitytype {
            incorrect_fields.push("entity_type".to_string())
        };
        if incorrect_fields.is_empty() {
            match self.entities.len() {
                0 => Err(EntitySetError::EmptyEntitySet),
                _ => {
                    let entitieslist = self.entities.join("', '");

                    let query = format!(
                        concat!("SELECT relation_directory.change_set_entities_guarded({}, ARRAY['{}'])"),
                        self.id.to_string(),
                        entitieslist
                    );
                    let row = conn.query_one(&query, &[]).await.map_err(|e| {
                        EntitySetError::DatabaseError(DatabaseError::from_msg(e.to_string()))
                    })?;

                    let missing_entities: Vec<String> = row.get(0);
                    if missing_entities.is_empty() {
                        let query = concat!(
                            "INSERT INTO attribute_staging.minerva_entity_set ",
                            "(entity_id, timestamp, name, fullname, \"group\", source_entity_type, owner, description, last_update) ",
                            "VALUES ($1, now(), $2, $3, $4, $5, $6, $7, CURRENT_DATE::text)",
                        );

                        conn.execute(
                            query,
                            &[
                                &self.id,
                                &self.name,
                                &format!("{}__{}", &self.name, &self.owner),
                                &self.group,
                                &self.entity_type,
                                &self.owner,
                                &self.description,
                            ],
                        )
                        .await
                        .map_err(|e| {
                            EntitySetError::DatabaseError(DatabaseError::from_msg(e.to_string()))
                        })?;

                        let query = "SELECT attribute_directory.transfer_staged(at) FROM attribute_directory.attribute_store at WHERE id = $1";
                        conn.execute(query, &[&self.id]).await.map_err(|e| {
                            EntitySetError::DatabaseError(DatabaseError::from_msg(e.to_string()))
                        })?;

                        let newdata = conn.query_one(
                            "SELECT name, \"group\", source_entity_type, owner, description, first_appearance, modified FROM attribute.minerva_entity_set es WHERE id = $1",
                                &[&self.id,])
                            .await
                            .map_err(|e| EntitySetError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;
                        let changed_entity_set = EntitySet {
                            id: self.id,
                            name: newdata.get(0),
                            group: newdata.get(1),
                            entity_type: newdata.get(2),
                            owner: newdata.get(3),
                            description: newdata.get(4),
                            entities: self.entities.to_vec(),
                            created: newdata.get(5),
                            modified: newdata.get(5),
                        };
                        Ok(changed_entity_set)
                    } else {
                        Err(EntitySetError::MissingEntities(missing_entities))
                    }
                }
            }
        } else {
            Err(EntitySetError::UnchangeableFields(incorrect_fields))
        }
    }
}

pub struct ChangeEntitySet {
    pub entity_set: EntitySet,
    pub entities: Vec<String>,
}

impl fmt::Display for ChangeEntitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChangeEntitySet({}:{})",
            &self.entity_set.owner, &self.entity_set.name
        )
    }
}

#[async_trait]
impl Change for ChangeEntitySet {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        let result = self.entity_set.update(client).await;
        match result {
            Ok(_) => Ok("Entity set updated".to_string()),
            Err(EntitySetError::DatabaseError(err)) => Err(Error::Database(err)),
            Err(EntitySetError::ExistingEntitySet(name, owner)) => {
                Err(Error::Database(DatabaseError {
                    msg: format!(
                        "An entity set with name {} and owner {} already exists.",
                        &name, &owner,
                    ),
                    kind: DatabaseErrorKind::UniqueViolation,
                }))
            }
            Err(EntitySetError::EmptyEntitySet) => Err(Error::Runtime(RuntimeError::from_msg(
                "Entity sets cannot be empty".to_string(),
            ))),
            Err(EntitySetError::MissingEntities(missing_entities)) => {
                Err(Error::Runtime(RuntimeError::from_msg(format!(
                    "The following entities do not exist: {}",
                    missing_entities.join(", ")
                ))))
            }
            Err(_) => Err(Error::Database(DatabaseError {
                msg: "Unexpected Error".to_string(),
                kind: DatabaseErrorKind::Default,
            })),
        }
    }
}

impl NewEntitySet {
    pub async fn create(&self, conn: &mut Transaction<'_>) -> Result<EntitySet, EntitySetError> {
        let row = conn
            .query_one(
                "SELECT relation_directory.entity_set_exists($1, $2)",
                &[&self.owner, &self.name],
            )
            .await
            .map_err(|e| EntitySetError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;

        match row.get(0) {
            true => Err(EntitySetError::ExistingEntitySet(
                self.name.clone(),
                self.owner.clone(),
            )),
            false => match self.entities.len() {
                0 => Err(EntitySetError::EmptyEntitySet),
                _ => {
                    let entitieslist = self.entities.join("', '");
                    let query = format!(
                        concat!(
                            "SELECT relation_directory.create_entity_set_guarded(",
                            "$1, $2, $3, $4, $5, ARRAY['{}'])"
                        ),
                        entitieslist
                    );

                    info!(
                        "SELECT relation_directory.create_entity_set_guarded('{}', '{}', '{}', '{}', '{}', ARRAY['{}'])",                       
                        &self.name,
                        &self.group,
                        &self.entity_type,
                        &self.owner,
                        &self.description,
                        entitieslist
                    );

                    let row = conn
                        .query_one(
                            &query,
                            &[
                                &self.name,
                                &self.group,
                                &self.entity_type,
                                &self.owner,
                                &self.description,
                            ],
                        )
                        .await
                        .map_err(|e| e.to_string())?;

                    let missing_entities: Vec<String> = row.get(0);

                    if missing_entities.is_empty() {
                        let iddata = conn.query_one(
                            "SELECT id, first_appearance, modified FROM attribute.minerva_entity_set es WHERE name = $1 AND owner = $2",
                                &[&self.name, &self.owner,])
                            .await
                            .map_err(|e| EntitySetError::DatabaseError(DatabaseError{msg: e.to_string(), kind: DatabaseErrorKind::Default}))?;
                        let created_entity_set = EntitySet {
                            id: iddata.get(0),
                            name: self.name.clone(),
                            group: self.group.clone(),
                            entity_type: self.entity_type.clone(),
                            owner: self.owner.clone(),
                            description: self.description.clone(),
                            entities: self.entities.to_vec(),
                            created: iddata.get(1),
                            modified: iddata.get(2),
                        };
                        Ok(created_entity_set)
                    } else {
                        Err(EntitySetError::MissingEntities(missing_entities))
                    }
                }
            },
        }
    }
}

pub struct CreateEntitySet {
    pub entity_set: NewEntitySet,
}

impl fmt::Display for CreateEntitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CreateEntitySet({}:{})",
            &self.entity_set.owner, &self.entity_set.name
        )
    }
}

#[async_trait]
impl Change for CreateEntitySet {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult {
        let result = self.entity_set.create(client).await;
        match result {
            Ok(entity_set) => Ok(format!("Entity set number {} created", &entity_set.id)),
            Err(EntitySetError::DatabaseError(err)) => Err(Error::Database(err)),
            Err(EntitySetError::ExistingEntitySet(name, owner)) => {
                Err(Error::Database(DatabaseError {
                    msg: format!(
                        "An entity set with name {} and owner {} already exists.",
                        &name, &owner,
                    ),
                    kind: DatabaseErrorKind::UniqueViolation,
                }))
            }
            Err(EntitySetError::EmptyEntitySet) => Err(Error::Runtime(RuntimeError::from_msg(
                "Entity sets cannot be empty".to_string(),
            ))),
            Err(EntitySetError::MissingEntities(missing_entities)) => {
                Err(Error::Runtime(RuntimeError::from_msg(format!(
                    "The following entities do not exist: {}",
                    missing_entities.join(", ")
                ))))
            }
            Err(_) => Err(Error::Runtime(RuntimeError::from_msg(
                "Unexpected Error".to_string(),
            ))),
        }
    }
}
