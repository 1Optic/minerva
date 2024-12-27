use deadpool_postgres::Pool;

use actix_web::{get, web::Data, web::Path, HttpResponse};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::error::Error;
use super::serviceerror::{ServiceError, ServiceErrorKind};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DataSource {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[utoipa::path(
    get,
    path="/data-sources",
    responses(
    (status = 200, description = "List all data sources", body = [DataSource]),
    (status = 500, description = "Problem interacting with database", body = Error)
    )
)]
#[get("/data-sources")]
pub(super) async fn get_data_sources(pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let client = pool.get().await.map_err(|_| ServiceError {
        kind: ServiceErrorKind::PoolError,
        message: "".to_string(),
    })?;

    let data_sources: Vec<DataSource> = client
        .query(
            "SELECT id, name, description FROM directory.data_source",
            &[],
        )
        .await
        .map_err(|e| Error {
            code: 500,
            message: e.to_string(),
        })
        .map(|rows| {
            rows.iter()
                .map(|row| DataSource {
                    id: row.get(0),
                    name: row.get(1),
                    description: row.get(2),
                })
                .collect()
        })?;

    Ok(HttpResponse::Ok().json(data_sources))
}

#[utoipa::path(
    get,
    path="/data-sources/{id}",
    responses(
    (status = 200, description = "Get a specific data source", body = DataSource),
    (status = 404, description = "Data source not found", body = Error),
    (status = 500, description = "Problem interacting with database", body = Error),
    )
)]
#[get("/data-sources/{id}")]
pub(super) async fn get_data_source(
    pool: Data<Pool>,
    id: Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let ds_id = id.into_inner();

    let client = pool.get().await.map_err(|_| ServiceError {
        kind: ServiceErrorKind::PoolError,
        message: "".to_string(),
    })?;

    let data_source = client
        .query_one(
            "SELECT id, name, description FROM directory.data_source WHERE id=$1",
            &[&ds_id],
        )
        .await
        .map_err(|_| Error {
            code: 404,
            message: format!("Data source with id {} not found", &ds_id),
        })
        .map(|row| DataSource {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        })?;

    Ok(HttpResponse::Ok().json(data_source))
}
