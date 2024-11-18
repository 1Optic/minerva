use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::ops::DerefMut;
use utoipa::ToSchema;

use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path, HttpResponse, Responder};
use chrono::{DateTime, Utc};

use minerva::entity_set::{
    load_entity_set, load_entity_sets, EntitySet, EntitySetError, NewEntitySet,
};
use minerva::error::DatabaseError;

use super::serviceerror::{ExtendedServiceError, ServiceError, ServiceErrorKind};
use crate::error::{Error, Success};

type PostgresName = String;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EntitySetData {
    pub name: PostgresName,
    pub group: Option<String>,
    pub entity_type: Option<String>,
    pub owner: String,
    pub description: Option<String>,
    pub entities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EntitySetDataFull {
    pub id: i32,
    pub name: PostgresName,
    pub group: String,
    pub entity_type: String,
    pub owner: String,
    pub description: String,
    pub entities: Vec<String>,
    pub created: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Id {
    pub id: i32,
}

impl EntitySetDataFull {
    fn entity_set(&self) -> EntitySet {
        EntitySet {
            id: self.id,
            name: self.name.to_string(),
            group: self.group.to_string(),
            entity_type: self.entity_type.to_string(),
            owner: self.owner.to_string(),
            description: self.description.to_string(),
            entities: self.entities.to_vec(),
            created: self.created.unwrap_or(Utc::now()),
            modified: self.modified.unwrap_or(Utc::now()),
        }
    }
}

impl EntitySetData {
    fn entity_set(&self) -> NewEntitySet {
        let group = match &self.group {
            None => "".to_string(),
            Some(value) => value.to_string(),
        };
        let entity_type = match &self.entity_type {
            None => "".to_string(),
            Some(value) => value.to_string(),
        };
        let description = match &self.description {
            None => "".to_string(),
            Some(value) => value.to_string(),
        };
        NewEntitySet {
            name: self.name.to_string(),
            group,
            entity_type,
            owner: self.owner.to_string(),
            description,
            entities: self.entities.to_vec(),
        }
    }
}

#[utoipa::path(
    get,
    path="/entitysets",
    responses(
    (status = 200, description = "List of existing entity sets", body = [EntitySet]),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[get("/entitysets")]
pub(super) async fn get_entity_sets(pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let mut manager = pool.get().await.map_err(|_| ServiceError {
        kind: ServiceErrorKind::PoolError,
        message: "".to_string(),
    })?;

    let client: &mut tokio_postgres::Client = manager.deref_mut().deref_mut();

    let data = load_entity_sets(client).await.map_err(|e| Error {
        code: 500,
        message: e.to_string(),
    })?;

    Ok(HttpResponse::Ok().json(data))
}

async fn change_entity_set_fn(
    pool: Data<Pool>,
    data: Json<EntitySetDataFull>,
) -> Result<HttpResponse, ExtendedServiceError> {
    let mut manager = pool.get().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), Value::String(e.to_string()));
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let mut tx = manager.transaction().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), Value::String(e.to_string()));
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let entity_set = data.entity_set();

    let result = entity_set.update(&mut tx).await;

    match result {
        Ok(_) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(Success {
                code: 200,
                message: "Entity set updated".into(),
            }))
        }
        Err(EntitySetError::NotFound(DatabaseError { msg: e, kind: _ })) => {
            let mut messages = Map::new();
            messages.insert(
                "id".to_string(),
                format!("Unable to find entity set with id {}", entity_set.id).into(),
            );
            messages.insert("general".to_string(), e.to_string().into());
            Ok(HttpResponse::NotFound().json(messages))
        }
        Err(EntitySetError::UnchangeableFields(fields)) => {
            let mut messages = Map::new();
            for field in fields {
                messages.insert(field, "Field cannot be changed".into());
            }
            Ok(HttpResponse::Conflict().json(messages))
        }
        Err(EntitySetError::DatabaseError(DatabaseError { msg: e, kind: _ })) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            Ok(HttpResponse::InternalServerError().json(messages))
        }
        Err(EntitySetError::MissingEntities(missing_entities)) => {
            let mut messages = Map::new();
            for entity in missing_entities {
                messages.insert(entity, "Entity does not exist".into());
            }
            Ok(HttpResponse::Conflict().json(messages))
        }
        Err(EntitySetError::EmptyEntitySet) => {
            let mut messages = Map::new();
            messages.insert("entities".to_string(), "Entity set cannot be empty".into());
            Ok(HttpResponse::BadRequest().json(messages))
        }
        Err(_) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), "Unexpected Error".into());
            Err(ExtendedServiceError {
                kind: ServiceErrorKind::InternalError,
                messages,
            })
        }
    }
}

#[utoipa::path(
    put,
    path="/entitysets",
    responses(
    (status = 200, description = "Changing entity set succeeded", body = Success),
    (status = 400, description = "Request could not be parsed", body = Error),
    (status = 409, description = "Changing entity set failed", body = Error),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[put("/entitysets")]
pub(super) async fn change_entity_set(
    pool: Data<Pool>,
    data: Json<EntitySetDataFull>,
) -> impl Responder {
    let result = change_entity_set_fn(pool, data).await;
    match result {
        Ok(res) => res,
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}

async fn create_entity_set_fn(
    pool: Data<Pool>,
    data: Json<EntitySetData>,
) -> Result<HttpResponse, ExtendedServiceError> {
    let mut manager = pool.get().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), Value::String(e.to_string()));
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let mut tx = manager.transaction().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), Value::String(e.to_string()));
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let entity_set = data.entity_set();

    let result = entity_set.create(&mut tx).await;

    match result {
        Ok(entity_set) => {
            tx.commit().await?;
            Ok(HttpResponse::Ok().json(Success {
                code: 200,
                message: format!("Entity set number {} created", &entity_set.id),
            }))
        }
        Err(EntitySetError::DatabaseError(DatabaseError { msg: e, kind: _ })) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            Ok(HttpResponse::InternalServerError().json(messages))
        }
        Err(EntitySetError::ExistingEntitySet(_name, _owner)) => {
            let mut messages = Map::new();
            messages.insert(
                "name".to_string(),
                "Entity set with name and owner already exists".into(),
            );
            Ok(HttpResponse::Conflict().json(messages))
        }
        Err(EntitySetError::EmptyEntitySet) => {
            let mut messages = Map::new();
            messages.insert("entities".to_string(), "Entity set cannot be empty".into());
            Ok(HttpResponse::BadRequest().json(messages))
        }
        Err(EntitySetError::MissingEntities(missing_entities)) => {
            let mut messages = Map::new();
            for entity in missing_entities {
                messages.insert(entity, "Entity does not exist".into());
            }
            Ok(HttpResponse::Conflict().json(messages))
        }
        Err(EntitySetError::IncorrectEntityType(_entity_type)) => {
            let mut messages = Map::new();
            messages.insert(
                "entity_type".to_string(),
                "Entity type with that name does not exist".into(),
            );
            Ok(HttpResponse::BadRequest().json(messages))
        }
        Err(_) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), "Unexpected Error".into());
            Err(ExtendedServiceError {
                kind: ServiceErrorKind::InternalError,
                messages,
            })
        }
    }
}

#[utoipa::path(
    post,
    path="/entitysets",
    responses(
    (status = 200, description = "Creating entity set succeeded", body = Success),
    (status = 400, description = "Request could not be parsed", body = Error),
    (status = 409, description = "Creating entity set failed", body = Error),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[post("/entitysets")]
pub(super) async fn create_entity_set(
    pool: Data<Pool>,
    data: Json<EntitySetData>,
) -> impl Responder {
    let result = create_entity_set_fn(pool, data).await;
    match result {
        Ok(res) => res,
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}

#[utoipa::path(
    delete,
    path="/entitysets",
    responses(
        (status = 200, description = "Deleting entity set succeeded", body=Success),
        (status = 400, description = "Request could not be parsed", body=Error),
        (status = 409, description = "Entity set does not exist", body=Error),
        (status = 500, description = "Database unreachable", body=Error),
    )
)]
#[delete("/entitysets")]
pub(super) async fn delete_entity_set_temp(pool: Data<Pool>, data: Json<Id>) -> impl Responder {
    let result = pool.get().await;
    match result {
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), Value::String(e.to_string()));
            HttpResponse::InternalServerError().json(messages)
        }
        Ok(mut manager) => {
            let client: &mut tokio_postgres::Client = manager.deref_mut().deref_mut();
            let preresult = load_entity_set(client, &data.id).await;
            match preresult {
                Ok(entityset) => {
                    let query =
                        "DELETE FROM attribute_history.minerva_entity_set WHERE entity_id = $1";
                    let result = client.execute(query, &[&entityset.id]).await;
                    match result {
                        Ok(_) => HttpResponse::Ok().json(Success {
                            code: 200,
                            message: format!("Entity set number {} deleted", &entityset.id),
                        }),
                        Err(e) => {
                            let mut messages = Map::new();
                            messages.insert("general".to_string(), Value::String(e.to_string()));
                            HttpResponse::InternalServerError().json(messages)
                        }
                    }
                }
                Err(e) => {
                    let mut messages = Map::new();
                    messages.insert("id".to_string(), Value::String(e.to_string()));
                    HttpResponse::Conflict().json(messages)
                }
            }
        }
    }
}

#[utoipa::path(
    delete,
    path="/entitysets/{id}",
    responses(
        (status = 200, description = "Deleting entity set succeeded", body=Success),
        (status = 400, description = "Request could not be parsed", body=Error),
        (status = 409, description = "Entity set does not exist", body=Error),
        (status = 500, description = "Database unreachable", body=Error),
    )
)]
#[delete("/entitysets/{id}")]
pub(super) async fn delete_entity_set(pool: Data<Pool>, id: Path<i32>) -> impl Responder {
    let result = pool.get().await;
    let es_id = id.into_inner();
    match result {
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), Value::String(e.to_string()));
            HttpResponse::InternalServerError().json(messages)
        }
        Ok(mut manager) => {
            let client: &mut tokio_postgres::Client = manager.deref_mut().deref_mut();
            let preresult = load_entity_set(client, &es_id).await;
            match preresult {
                Ok(_) => {
                    let query =
                        "DELETE FROM attribute_history.minerva_entity_set WHERE entity_id = $1";
                    let result = client.execute(query, &[&es_id]).await;
                    match result {
                        Ok(_) => HttpResponse::Ok().json(Success {
                            code: 200,
                            message: format!("Entity set number {} deleted", &es_id),
                        }),
                        Err(e) => {
                            let mut messages = Map::new();
                            messages.insert("general".to_string(), Value::String(e.to_string()));
                            HttpResponse::InternalServerError().json(messages)
                        }
                    }
                }
                Err(e) => {
                    let mut messages = Map::new();
                    messages.insert("id".to_string(), Value::String(e.to_string()));
                    HttpResponse::Conflict().json(messages)
                }
            }
        }
    }
}
