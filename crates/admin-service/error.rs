use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ExtendedError {
    pub code: i32,
    pub messages: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Success {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreationSuccess {
    pub code: i32,
    pub message: String,
    pub id: Option<i32>,
}

#[derive(Display, From, Debug)]
pub struct BadRequest {
    pub message: String,
}
