use super::serviceerror::ServiceError;
use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Version {
    pub version: String,
}

#[utoipa::path(
    get,
    path="/version",
    responses(
    (status = 200, description = "version number", body = String),
    )
)]
#[get("/version")]
pub(super) async fn get_version() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(Version {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
