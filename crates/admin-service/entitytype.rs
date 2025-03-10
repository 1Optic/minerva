use deadpool_postgres::Pool;

use actix_web::{get, web::Data, web::Path, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EntityType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[utoipa::path(
    get,
    path="/entity-types",
    responses(
    (status = 200, description = "List all entity types", body = [EntityType]),
    (status = 500, description = "Unable to interact with database", body = Error)
    )
)]
#[get("/entity-types")]
pub(super) async fn get_entity_types(pool: Data<Pool>) -> impl Responder {
    let mut m: Vec<EntityType> = vec![];
    let result = pool.get().await;
    match result {
        Err(e) => HttpResponse::InternalServerError().json(Error {
            code: 500,
            message: e.to_string(),
        }),
        Ok(client) => {
            let query = client
                .query(
                    "SELECT id, name, description FROM directory.entity_type",
                    &[],
                )
                .await;
            match query {
                Err(e) => HttpResponse::InternalServerError().json(Error {
                    code: 500,
                    message: e.to_string(),
                }),
                Ok(result) => {
                    for row in result {
                        let entitytype = EntityType {
                            id: row.get(0),
                            name: row.get(1),
                            description: row.get(2),
                        };
                        m.push(entitytype);
                    }
                    HttpResponse::Ok().json(m)
                }
            }
        }
    }
}

#[utoipa::path(
    get,
    path="/entity-types/{id}",
    responses(
    (status = 200, description = "Get a specific entity type", body = EntityType),
    (status = 404, description = "Entity type not found", body = Error),
    (status = 500, description = "Problem interacting with database", body = Error),
    )
)]
#[get("/entity-types/{id}")]
pub(super) async fn get_entity_type(pool: Data<Pool>, id: Path<i32>) -> impl Responder {
    let et_id = id.into_inner();

    let result = pool.get().await;
    match result {
        Err(e) => HttpResponse::InternalServerError().json(Error {
            code: 500,
            message: e.to_string(),
        }),
        Ok(client) => {
            let query_result = client
                .query_one(
                    "SELECT id, name, description FROM directory.entity_type WHERE id=$1",
                    &[&et_id],
                )
                .await;

            match query_result {
                Ok(row) => {
                    let entitytype = EntityType {
                        id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                    };

                    HttpResponse::Ok().json(entitytype)
                }
                Err(_e) => HttpResponse::NotFound().json(Error {
                    code: 404,
                    message: format!("Entity type with id {} not found", &et_id),
                }),
            }
        }
    }
}
