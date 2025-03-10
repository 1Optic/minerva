use deadpool_postgres::Pool;
use log::{debug, trace};
use std::time::Duration;

use actix_web::{get, post, put, web::Data, web::Path, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use serde_json::Map;
use utoipa::ToSchema;

use minerva::change::Change;
use minerva::error::DatabaseError;
use minerva::trigger::{
    list_triggers, load_thresholds_with_client, load_trigger, set_enabled, set_thresholds,
    AddTrigger, Threshold, TriggerError,
};
use minerva::trigger_template::{
    get_bare_template, get_template_from_id, list_templates, BareTemplate, ParameterValue,
    Template, TemplatedTrigger, TriggerTemplateError,
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
    pub data_type: Option<String>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TemplateData {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub sql_body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ShortTemplateData {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ParameterData {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TemplateInstanceDefinition {
    pub template_id: i32,
    pub parameters: Vec<ParameterData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TemplatedTriggerDefinition {
    pub name: String,
    pub description: Option<String>,
    pub thresholds: Vec<ThresholdData>,
    pub entity_type: String,
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    pub weight: i32,
    pub enabled: bool,
    pub template_instance: TemplateInstanceDefinition,
}

impl From<Threshold> for ThresholdData {
    fn from(threshold: Threshold) -> Self {
        ThresholdData {
            name: threshold.name,
            data_type: Some(threshold.data_type),
            value: threshold.value,
        }
    }
}

impl From<ThresholdData> for Threshold {
    fn from(threshold_data: ThresholdData) -> Self {
        Threshold {
            name: threshold_data.name,
            data_type: threshold_data.data_type.unwrap_or("numeric".to_string()),
            value: threshold_data.value,
        }
    }
}

impl From<BareTemplate> for TemplateData {
    fn from(template: BareTemplate) -> Self {
        TemplateData {
            id: template.id,
            name: template.name,
            body: template.body,
            sql_body: template.sql_body,
        }
    }
}

impl From<Template> for TemplateData {
    fn from(template: Template) -> Self {
        BareTemplate::from(template).into()
    }
}

impl From<TemplateData> for ShortTemplateData {
    fn from(template: TemplateData) -> Self {
        ShortTemplateData {
            id: template.id,
            name: template.name,
        }
    }
}

impl From<BareTemplate> for ShortTemplateData {
    fn from(template: BareTemplate) -> Self {
        TemplateData::from(template).into()
    }
}

impl From<Template> for ShortTemplateData {
    fn from(template: Template) -> Self {
        TemplateData::from(template).into()
    }
}

impl From<ParameterValue> for ParameterData {
    fn from(parm: ParameterValue) -> Self {
        ParameterData {
            name: parm.name,
            value: parm.value,
        }
    }
}

impl From<ParameterData> for ParameterValue {
    fn from(parm: ParameterData) -> Self {
        ParameterValue {
            name: parm.name,
            value: parm.value,
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

    let client: &mut tokio_postgres::Client = &mut manager;
    let triggerdata = list_triggers(client).await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;

    let mut result: Vec<TriggerData> = [].to_vec();

    for trigger in &triggerdata {
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
            thresholds: thresholds.into_iter().map(std::convert::Into::into).collect(),
        });
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

    let client: &mut tokio_postgres::Client = &mut manager;

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
                trigger.thresholds = data.thresholds.into_iter().map(std::convert::Into::into).collect();
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

#[utoipa::path(
    get,
    path="/templates",
    responses(
    (status = 200, description = "List of existing templates", body = [ShortTemplateData]),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[get("/templates")]
pub(super) async fn get_templates(pool: Data<Pool>) -> impl Responder {
    let try_manager = pool.get().await;

    match try_manager {
        Ok(mut manager) => {
            let client: &mut tokio_postgres::Client = &mut manager;
            let result = list_templates(client);
            match result.await {
                Ok(res) => HttpResponse::Ok().json(
                    res.into_iter()
                        .map(ShortTemplateData::from)
                        .collect::<Vec<ShortTemplateData>>(),
                ),
                Err(TriggerTemplateError::DatabaseError(e)) => {
                    let mut messages = Map::new();
                    messages.insert("general".to_string(), e.msg.into());
                    HttpResponse::InternalServerError().json(messages)
                }
                Err(_) => {
                    let mut messages = Map::new();
                    messages.insert("general".to_string(), "Unexpected error".into());
                    HttpResponse::InternalServerError().json(messages)
                }
            }
        }
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}

#[utoipa::path(
    get,
    path="/templates/{id}",
    responses(
        (status = 200, description = "Data of a template", body = TemplateData),
        (status = 404, description = "Template not found", body = Error),
        (status = 500, description = "Database unreachable or other error", body = Error),
    )
)]
#[get("/templates/{id}")]
pub(super) async fn get_template(pool: Data<Pool>, id: Path<i32>) -> impl Responder {
    let try_manager = pool.get().await;
    let trigger_id = id.into_inner();

    match try_manager {
        Ok(mut manager) => {
            let client: &mut tokio_postgres::Client = &mut manager;
            let result = get_bare_template(client, trigger_id);
            match result.await {
                Ok(template) => HttpResponse::Ok().json(TemplateData::from(template)),
                Err(TriggerTemplateError::DatabaseError(e)) => {
                    let mut messages = Map::new();
                    messages.insert("general".to_string(), e.msg.into());
                    HttpResponse::InternalServerError().json(messages)
                }
                Err(TriggerTemplateError::NoTemplate(back_id)) => {
                    let mut messages = Map::new();
                    messages.insert(
                        "id".to_string(),
                        format!("No template with id {}", &back_id).into(),
                    );
                    HttpResponse::NotFound().json(messages)
                }
                Err(_) => {
                    let mut messages = Map::new();
                    messages.insert("general".to_string(), "Unexpected error".into());
                    HttpResponse::InternalServerError().json(messages)
                }
            }
        }
        Err(e) => {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            HttpResponse::InternalServerError().json(messages)
        }
    }
}

async fn create_trigger_fn(
    pool: Data<Pool>,
    data: TemplatedTriggerDefinition,
) -> Result<HttpResponse, ExtendedServiceError> {
    trace!("Entering create trigger function");
    let mut manager = pool.get().await.map_err(|e| {
        let mut messages = Map::new();
        messages.insert("general".to_string(), e.to_string().into());
        ExtendedServiceError {
            kind: ServiceErrorKind::InternalError,
            messages,
        }
    })?;
    let client: &mut tokio_postgres::Client = &mut manager;
    trace!("Created client");

    let template = get_template_from_id(client, data.template_instance.template_id)
        .await
        .map_err(|e| match e {
            TriggerTemplateError::NoTemplate(_) => {
                let mut messages = Map::new();
                messages.insert("template_id".to_string(), "No template with this id".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::BadRequest,
                    messages,
                }
            }
            TriggerTemplateError::DatabaseError(e) => {
                let mut messages = Map::new();
                messages.insert("general".to_string(), e.msg.into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            }
            _ => {
                let mut messages = Map::new();
                messages.insert("general".to_string(), "Unexpected error".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            }
        })?;
    debug!("Got template {:?}", template);

    let mut transaction: tokio_postgres::Transaction<'_> =
        client.transaction().await.map_err(|e| {
            let mut messages = Map::new();
            messages.insert("general".to_string(), e.to_string().into());
            ExtendedServiceError {
                kind: ServiceErrorKind::InternalError,
                messages,
            }
        })?;

    let templated_trigger = TemplatedTrigger {
        template,
        name: data.name.clone(),
        description: data.description.clone(),
        parameters: data
            .template_instance
            .parameters
            .clone()
            .into_iter()
            .map(ParameterValue::from)
            .collect(),
        thresholds: data
            .thresholds
            .clone()
            .into_iter()
            .map(Threshold::from)
            .collect(),
        entity_type: data.entity_type.clone(),
        granularity: data.granularity,
        weight: data.weight,
        enabled: data.enabled,
    };
    debug!("Got trigger {:?}", templated_trigger);

    let trigger = templated_trigger
        .create_trigger(&mut transaction)
        .await
        .map_err(|e| match e {
            TriggerTemplateError::MissingParameter(parm) => {
                let mut messages = Map::new();
                messages.insert(parm, "Parameter missing".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::BadRequest,
                    messages,
                }
            }
            TriggerTemplateError::ExtraneousParameter(parm) => {
                let mut messages = Map::new();
                messages.insert(parm, "Parameter not defined in template".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::BadRequest,
                    messages,
                }
            }
            TriggerTemplateError::MissingThreshold(parm) => {
                let mut messages = Map::new();
                messages.insert(parm, "No threshold value defined".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::BadRequest,
                    messages,
                }
            }
            TriggerTemplateError::NotAThreshold(parm) => {
                let mut messages = Map::new();
                messages.insert(parm, "This parameter is not a threshold".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::BadRequest,
                    messages,
                }
            }
            TriggerTemplateError::DatabaseError(e2) => {
                let mut messages = Map::new();
                messages.insert("general".to_string(), e2.msg.into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            }
            TriggerTemplateError::TriggerError(TriggerError::DatabaseError(e2)) => {
                let mut messages = Map::new();
                messages.insert("general".to_string(), e2.msg.into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            }
            TriggerTemplateError::TriggerExists(name) => {
                let mut messages = Map::new();
                messages.insert(
                    "name".to_string(),
                    format!("A trigger named {name} already exists").into(),
                );
                ExtendedServiceError {
                    kind: ServiceErrorKind::Conflict,
                    messages,
                }
            }
            TriggerTemplateError::NoCounter(counter) => {
                let mut messages = Map::new();
                messages.insert(
                    "parameters".to_string(),
                    format!("Counter {counter} does not exist").into(),
                );
                ExtendedServiceError {
                    kind: ServiceErrorKind::NotFound,
                    messages,
                }
            }
            TriggerTemplateError::CounterNotUnique(counter) => {
                let mut messages = Map::new();
                messages.insert(
                    "parameters".to_string(),
                    format!("Counter {counter} cannot be uniquely identified").into(),
                );
                ExtendedServiceError {
                    kind: ServiceErrorKind::NotFound,
                    messages,
                }
            }
            _ => {
                let mut messages = Map::new();
                messages.insert("general".to_string(), "Unexpected Error".into());
                ExtendedServiceError {
                    kind: ServiceErrorKind::InternalError,
                    messages,
                }
            }
        })?;
    debug!("Created trigger {:?}", trigger);

    transaction.commit().await?;

    let change = AddTrigger {
        trigger,
        verify: false,
    };

    let message = change.apply(client).await?;

    debug!("Returned message {}", message);

    trace!("Transaction committed");

    Ok(HttpResponse::Ok().json(Success { code: 200, message }))
}

// curl -H "Content-Type: application/json" -X POST -d '{"name": "high_downtime", "description": "downtime higher than maximum", "thresholds": [{"name": "max_downtime", "data_type": "numeric", "value": "50"}], "entity_type": "v-cell", "granularity": "15m", "weight": 100, "enabled": true, "template_instance": {"template_id": 1, "parameters": [{"parameter": "counter", "value": "L.Cell.Unavail.Dur.Sys"}, {"parameter": "comparison", "value": ">"}, {"parameter": "value", "value": "max_downtime"}]}}' localhost:8000/triggers
#[utoipa::path(
    post,
    path="/triggers",
    responses(
    (status = 200, description = "Creating trigger succeeded", body = Success),
    (status = 400, description = "Request could not be parsed", body = Error),
    (status = 500, description = "Database unreachable", body = Error),
    )
)]
#[post("/triggers")]
pub(super) async fn create_trigger(pool: Data<Pool>, data: String) -> impl Responder {
    trace!("create_trigger is called");
    let preresult: Result<TemplatedTriggerDefinition, serde_json::Error> =
        serde_json::from_str(&data);
    match preresult {
        Ok(definition) => {
            trace!("create_trigger_function will be called");
            let result = create_trigger_fn(pool, definition);
            match result.await {
                Ok(res) => res,
                Err(e) => {
                    let mut messages = Map::new();
                    messages.insert("general".to_string(), e.to_string().into());
                    HttpResponse::InternalServerError().json(messages)
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().json(Error {
            code: 400,
            message: e.to_string(),
        }),
    }
}
