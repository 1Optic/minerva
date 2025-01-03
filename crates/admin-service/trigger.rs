use deadpool_postgres::Pool;
use std::ops::DerefMut;

use actix_web::{get, put, web::Data, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use serde_json::Map;
use utoipa::ToSchema;

use minerva::error::DatabaseError;
use minerva::trigger::{
    list_triggers, load_thresholds_with_client, load_trigger, set_enabled, set_thresholds,
    Threshold, TriggerError,
};

use super::serviceerror::{ExtendedServiceError, ServiceErrorKind};
use crate::error::{Error, Success};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TriggerData {
    name: String,
    enabled: bool,
    description: String,
    thresholds: Vec<ThresholdData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ThresholdData {
    pub name: String,
    pub data_type: String,
    pub value: String,
}

impl From<Threshold> for ThresholdData {
    fn from(threshold: Threshold) -> Self {
        ThresholdData {
            name: threshold.name,
            data_type: threshold.data_type,
            value: threshold.value,
        }
    }
}

impl From<ThresholdData> for Threshold {
    fn from(threshold_data: ThresholdData) -> Self {
        Threshold {
            name: threshold_data.name,
            data_type: threshold_data.data_type,
            value: threshold_data.value,
        }
    }
}

async fn get_triggers_fn(pool: Data<Pool>) -> Result<HttpResponse, ExtendedServiceError> {
    let mut manager = pool.get().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let client: &mut tokio_postgres::Client = manager.deref_mut().deref_mut();
    let triggerdata = list_triggers(client).await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let mut result: Vec<TriggerData> = [].to_vec();

    for trigger in triggerdata.iter() {
        let thresholds = load_thresholds_with_client(client, &trigger.name)
            .await
            .map_err(|e| {
                let mut messages = Map::new();
                messages.insert("general".to_string(), e.to_string().into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            })?;

        result.push(TriggerData {
            name: trigger.name.clone(),
            enabled: trigger.enabled,
            description: trigger.description.clone(),
            thresholds: thresholds.into_iter().map(|t| t.into()).collect(),
        })
    }

    Ok(HttpResponse::Ok().json(result))
}

#[utoipa::path(
    get,
    path="/triggers",
    responses(
    (status = 200, description = "List of existing triggers", body = [TriggerData]),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[get("/triggers")]
pub(super) async fn get_triggers(pool: Data<Pool>) -> impl Responder {
    let result = get_triggers_fn(pool);
    match result.await {
        Ok(res) => res,
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}

async fn change_thresholds_fn(
    pool: Data<Pool>,
    post: String,
) -> Result<HttpResponse, ExtendedServiceError> {
    let data: TriggerData = serde_json::from_str(&post).map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let mut manager = pool.get().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let client: &mut tokio_postgres::Client = manager.deref_mut().deref_mut();

    let mut transaction = client.transaction().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let result = load_trigger(&mut transaction, &data.name).await;

    match result {
        Err(TriggerError::DatabaseError(DatabaseError { msg, kind: _ })) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), msg.into());
            Ok(HttpResponse::InternalServerError().json(messages))
        }
        Err(TriggerError::NotFound(_)) => {
            let mut messages = Map::new();
            messages.insert(
                "name".to_string(),
                "Trigger does not exist".to_string().into(),
            );
            Ok(HttpResponse::NotFound().json(messages))
        }
        Err(TriggerError::GranularityError(granularity)) => {
            let mut messages = Map::new();
            messages.insert(
                "general".to_string(),
                format!("Unable to parse granularity {}", &granularity).into(),
            );
            Ok(HttpResponse::BadRequest().json(messages))
        }
        Err(TriggerError::FunctionError(function)) => {
            let mut messages = Map::new();
            messages.insert(
                "general".to_string(),
                format!("Unable to load function {}", &function).into(),
            );
            Ok(HttpResponse::InternalServerError().json(messages))
        }
        Ok(mut trigger) => {
            let mut reports = Map::new();

            for threshold in &data.thresholds {
                match trigger
                    .thresholds
                    .iter()
                    .find(|th| th.name == threshold.name)
                {
                    Some(_) => {}
                    None => {
                        reports.insert(threshold.name.clone(), "This field does not exist".into());
                    }
                }
            }

            for threshold in &trigger.thresholds {
                match data.thresholds.iter().find(|th| th.name == threshold.name) {
                    Some(_) => {}
                    None => {
                        reports.insert(threshold.name.clone(), "This field is required".into());
                    }
                }
            }

            if !reports.is_empty() {
                Ok(HttpResponse::Conflict().json(reports))
            } else {
                trigger.thresholds = data.thresholds.into_iter().map(|t| t.into()).collect();
                trigger.enabled = data.enabled;
                trigger.description = data.description;

                set_thresholds(&trigger, &mut transaction)
                    .await
                    .map_err(|e| Error {
                        code: 409,
                        message: e.to_string(),
                    })?;

                set_enabled(&mut transaction, &trigger.name, data.enabled)
                    .await
                    .map_err(|e| Error {
                        code: 409,
                        message: e.to_string(),
                    })?;

                transaction.commit().await.map_err(|e| Error {
                    code: 409,
                    message: e.to_string(),
                })?;

                Ok(HttpResponse::Ok().json(Success {
                    code: 200,
                    message: "trigger updated".to_string(),
                }))
            }
        }
    }
}

// curl -H "Content-Type: application/json" -X PUT -d '{"name":"average-output","entity_type":"Cell","data_type":"numeric","enabled":true,"source_trends":["L.Thrp.bits.UL.NsaDc"],"definition":"public.safe_division(SUM(\"L.Thrp.bits.UL.NsaDc\"),1000::numeric)","description":{"type": "ratio", "numerator": [{"type": "trend", "value": "L.Thrp.bits.UL.NsaDC"}], "denominator": [{"type": "constant", "value": "1000"}]}}' localhost:8000/triggers
#[utoipa::path(
    put,
    path="/triggers",
    responses(
    (status = 200, description = "Updated trigger", body = Success),
    (status = 400, description = "Input format incorrect", body = Error),
    (status = 404, description = "Trigger not found", body = Error),
    (status = 409, description = "Update failed", body = Error),
    (status = 500, description = "General error", body = Error)
    )
)]
#[put("/triggers")]
pub(super) async fn change_thresholds(pool: Data<Pool>, post: String) -> impl Responder {
    let result = change_thresholds_fn(pool, post);
    match result.await {
        Ok(res) => res,
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}
