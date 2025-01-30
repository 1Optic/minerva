use std::time::Duration;

use humantime::format_duration;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row, Transaction};

use super::error::DatabaseError;
use crate::trigger::{
    trigger_exists, KPIDataColumn, Threshold, TrendStoreLink, Trigger, TriggerError,
};

#[derive(Debug)]
#[non_exhaustive]
pub enum TriggerTemplateError {
    DatabaseError(DatabaseError),
    TriggerError(TriggerError),
    MissingParameter(String),
    ExtraneousParameter(String),
    MissingThreshold(String),
    NotAThreshold(String),
    NoCounter(String),
    CounterNotUnique(String),
    TriggerExists(String),
    NoTemplate(String),
    UnexpectedError,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ParameterType {
    Default,
    Counter,
    ThresholdVariable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateParameter {
    pub name: String,
    pub parameter_type: ParameterType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub sql: String,
    pub parameters: Vec<TemplateParameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BareTemplate {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub sql_body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterValue {
    pub parameter: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplatedTrigger {
    pub template: Template,
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<ParameterValue>,
    pub thresholds: Vec<Threshold>,
    pub entity_type: String,
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    pub weight: i32,
    pub enabled: bool,
}

impl From<DatabaseError> for TriggerTemplateError {
    fn from(e: DatabaseError) -> TriggerTemplateError {
        TriggerTemplateError::DatabaseError(e)
    }
}

impl From<TriggerError> for TriggerTemplateError {
    fn from(e: TriggerError) -> TriggerTemplateError {
        match e {
            TriggerError::DatabaseError(e) => TriggerTemplateError::DatabaseError(e),
            te => TriggerTemplateError::TriggerError(te),
        }
    }
}

impl From<Template> for BareTemplate {
    fn from(template: Template) -> Self {
        BareTemplate {
            id: template.id,
            name: template.name,
            body: template.description,
            sql_body: template.sql,
        }
    }
}

impl Template {
    pub fn get_parameter(self, parameter_name: &str) -> Option<TemplateParameter> {
        self.parameters
            .into_iter()
            .find(|p| p.name == parameter_name)
    }
}

impl TemplatedTrigger {
    pub fn check_trigger(&self) -> Result<(), TriggerTemplateError> {
        if let Some(parm) = self.template.parameters.clone().into_iter().find(|p| {
            !self
                .parameters
                .clone()
                .into_iter()
                .any(|pp| pp.parameter == p.name)
        }) {
            return Err(TriggerTemplateError::MissingParameter(parm.name));
        };
        if let Some(parm) = self
            .parameters
            .clone()
            .into_iter()
            .find(|p| self.template.clone().get_parameter(&p.parameter).is_none())
        {
            return Err(TriggerTemplateError::ExtraneousParameter(parm.parameter));
        };
        let needed_thresholds = self.parameters.clone().into_iter().filter(|p| {
            self.template
                .clone()
                .get_parameter(&p.parameter)
                .unwrap()
                .parameter_type
                == ParameterType::ThresholdVariable
        });
        if let Some(threshold) = needed_thresholds.clone().find(|p| {
            !self
                .thresholds
                .clone()
                .into_iter()
                .any(|th| th.name == p.value)
        }) {
            return Err(TriggerTemplateError::MissingThreshold(threshold.value));
        };
        if let Some(threshold) = self
            .thresholds
            .clone()
            .into_iter()
            .find(|th| !needed_thresholds.clone().any(|nt| nt.value == th.name))
        {
            return Err(TriggerTemplateError::NotAThreshold(threshold.name));
        }
        Ok(())
    }

    pub fn adapted_parameter_name(&self, parameter: &ParameterValue) -> String {
        match self
            .template
            .parameters
            .clone()
            .into_iter()
            .find(|p| p.name == parameter.parameter)
        {
            Some(template_parameter) => match template_parameter.parameter_type {
                ParameterType::Counter => "\"".to_owned() + &parameter.value + "\"",
                _ => parameter.value.clone(),
            },
            None => parameter.value.clone(),
        }
    }

    pub async fn create_trigger(
        &self,
        client: &mut Transaction<'_>,
    ) -> Result<Trigger, TriggerTemplateError> {
        self.check_trigger()?;

        if trigger_exists(&self.name, client)
            .await
            .map_err(|e| DatabaseError::from_msg(format!("Error running check: {e}")))?
        {
            return Err(TriggerTemplateError::TriggerExists(self.name.clone()));
        };

        let mut sources: Vec<String> = vec![];
        let counters: Vec<ParameterValue> = self
            .parameters
            .clone()
            .into_iter()
            .filter(|p| {
                self.template
                    .clone()
                    .get_parameter(&p.parameter)
                    .unwrap()
                    .parameter_type
                    == ParameterType::Counter
            })
            .collect();
        for counter in &counters {
            let granularity_str: String = format_duration(self.granularity).to_string();

            let query = concat!(
                "SELECT tsp.name ",
                "FROM trigger.template t ",
                "JOIN trend_directory.table_trend tt ON tt.name = $1 ",
                "JOIN trend_directory.trend_store_part tsp ON tt.trend_store_part_id = tsp.id ",
                "JOIN trend_directory.trend_store ts ON tsp.trend_store_id = ts.id ",
                "JOIN directory.entity_type et ON ts.entity_type_id = et.id ",
                "WHERE t.name = $2 AND ts.granularity = $3::text::interval AND et.name = $4",
            );

            let tsprows = client
                .query(
                    query,
                    &[
                        &counter.value,
                        &self.template.name,
                        &granularity_str,
                        &self.entity_type,
                    ],
                )
                .await
                .map_err(|e| DatabaseError::from_msg(format!("Error running check: {e}")))?;
            match tsprows.len() {
                0 => {
                    return Err(TriggerTemplateError::NoCounter(counter.value.clone()));
                }
                1 => {}
                _ => {
                    return Err(TriggerTemplateError::CounterNotUnique(
                        counter.value.clone(),
                    ));
                }
            };

            let source: String = tsprows[0].get(0);
            if !sources.iter().any(|s| s == &source) {
                sources.push(source);
            };
        }

        let kpi_data = counters
            .clone()
            .into_iter()
            .map(|c| KPIDataColumn {
                name: c.value,
                data_type: "numeric".to_string(),
            })
            .collect();

        let mut kpi_function = concat!(
            "BEGIN\n",
            "  RETURN query EXECUTE $query$\n",
            "    SELECT\n",
            "      t1.entity_id,\n",
            "      $1",
        )
        .to_string();
        for counter in &counters {
            kpi_function.push_str(&format!(",\n      \"{}\"::numeric", counter.value));
        }
        let mut sourcecounter = 1;
        for source in &sources {
            if sourcecounter == 1 {
                kpi_function.push_str(&format!("\n    FROM trend.\"{}\" t1\n", source));
            } else {
                kpi_function.push_str(&format!(
                    "    JOIN trend.\"{}\" t{} ON t{}.timestamp = t1.timestamp AND t{}.entity_id = t1.entity_id\n",
                    source, sourcecounter, sourcecounter, sourcecounter
                ))
            };
            sourcecounter += 1;
        }
        sourcecounter = 1;
        for _ in &sources {
            if sourcecounter == 1 {
                kpi_function.push_str("    WHERE t1.timestamp = $1")
            } else {
                kpi_function.push_str(&format!(" AND t{}.timestamp = $1", sourcecounter))
            };
            sourcecounter += 1;
        }
        kpi_function.push_str("\n  $query$ USING $1;\nEND;");

        let mut condition = self.template.sql.clone();
        for parameter in &self.parameters {
            condition = condition.replace(
                &("{".to_owned() + &parameter.parameter + "}"),
                &self.adapted_parameter_name(parameter),
            );
        }

        let mut data_code = format!(
            "SELECT json_build_object(\n  '{}', \"{}\".name",
            self.entity_type, self.entity_type
        );
        for counter in &counters {
            data_code.push_str(&format!(
                ",\n  '{}', $1.\"{}\"",
                counter.value, counter.value
            ));
        }
        data_code.push_str(&format!(
            "\n)\nFROM entity.{} et\nWHERE et.id = $1.entity_id",
            self.entity_type
        ));

        let description = match &self.description {
            Some(value) => value.to_string(),
            None => {
                let mut description_from_template = self.template.description.clone();
                for parameter in &self.parameters {
                    description_from_template = description_from_template.replace(
                        &("{".to_owned() + &parameter.parameter + "}"),
                        &parameter.value,
                    );
                }
                description_from_template
            }
        };

        Ok(Trigger {
            name: self.name.clone(),
            kpi_data,
            kpi_function,
            thresholds: self.thresholds.clone(),
            condition,
            weight: self.weight.to_string(),
            notification: "SELECT 'obsolete'".to_string(),
            tags: vec![],
            fingerprint: "SELECT 'obsolete'".to_string(),
            notification_store: "template-trigger-notification".to_string(),
            data: data_code,
            trend_store_links: sources
                .iter()
                .map(|source| TrendStoreLink {
                    part_name: source.to_string(),
                    mapping_function: "mapping_id".to_string(),
                })
                .collect(),
            mapping_functions: vec![],
            description,
            granularity: self.granularity,
            enabled: self.enabled,
        })
    }
}

pub async fn list_templates(conn: &mut Client) -> Result<Vec<BareTemplate>, TriggerTemplateError> {
    let query = concat!(
        "SELECT id, name, description_body, sql_body ",
        "FROM trigger.template"
    );

    let result = conn
        .query(query, &[])
        .await
        .map_err(|e| TriggerTemplateError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;

    Ok(result
        .into_iter()
        .map(|row: Row| BareTemplate {
            id: row.get(0),
            name: row.get(1),
            body: row.get(2),
            sql_body: row.get(3),
        })
        .collect())
}

pub async fn get_bare_template(
    conn: &mut Client,
    id: i32,
) -> Result<BareTemplate, TriggerTemplateError> {
    let query = concat!(
        "SELECT id, name, description_body, sql_body ",
        "FROM trigger.template ",
        "WHERE id = $1",
    );

    let rows = conn
        .query(query, &[&id])
        .await
        .map_err(|e| TriggerTemplateError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;

    match rows.len() {
        1 => Ok(BareTemplate {
            id: rows[0].get(0),
            name: rows[0].get(1),
            body: rows[0].get(2),
            sql_body: rows[0].get(3),
        }),
        0 => Err(TriggerTemplateError::NoTemplate(id.to_string())),
        _ => Err(TriggerTemplateError::UnexpectedError),
    }
}

pub async fn get_template_from_id(
    conn: &mut Client,
    id: i32,
) -> Result<Template, TriggerTemplateError> {
    let bare_template = get_bare_template(conn, id).await?;

    let query = concat!(
        "SELECT name, is_variable, is_source_name ",
        "FROM trigger.template_parameter WHERE template_id = $1",
    );

    let rows = conn
        .query(query, &[&id])
        .await
        .map_err(|e| TriggerTemplateError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;

    let parameters: Vec<TemplateParameter> = rows
        .into_iter()
        .map(|row| TemplateParameter {
            name: row.get(0),
            parameter_type: if row.get(1) {
                ParameterType::ThresholdVariable
            } else if row.get(2) {
                ParameterType::Counter
            } else {
                ParameterType::Default
            },
        })
        .collect();

    Ok(Template {
        id: bare_template.id,
        name: bare_template.name,
        description: bare_template.body,
        sql: bare_template.sql_body,
        parameters,
    })
}
