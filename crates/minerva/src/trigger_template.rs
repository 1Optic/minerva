use std::{collections::HashMap, time::Duration};

use humantime::format_duration;
use postgres_protocol::escape::escape_identifier;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row, Transaction};

use super::error::DatabaseError;
use crate::trigger::{
    trigger_exists, KPIDataColumn, Threshold, TrendStoreLink, Trigger, TriggerError,
};

const DEFAULT_THRESHOLD_DATA_TYPE: &str = "numeric";
const DEFAULT_THRESHOLD_VALUE: &str = "0";

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
    LookbackVariable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub has_lookback: bool,
    pub lookback_parameter: Option<String>,
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
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedParameterValue {
    pub name: String,
    pub value: String,
    pub data_type: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplatedTrigger {
    pub template: Template,
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<ExtendedParameterValue>,
    pub thresholds: Option<Vec<Threshold>>,
    pub entity_type: String,
    #[serde(with = "humantime_serde")]
    pub granularity: Duration,
    pub weight: i32,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullTemplatedTrigger {
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

impl From<ExtendedParameterValue> for ParameterValue {
    fn from(epv: ExtendedParameterValue) -> Self {
        ParameterValue {
            name: epv.name,
            value: epv.value,
        }
    }
}

impl From<ParameterValue> for ExtendedParameterValue {
    fn from(pv: ParameterValue) -> Self {
        ExtendedParameterValue {
            name: pv.name,
            value: pv.value,
            default_value: None,
            data_type: None,
        }
    }
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

impl From<FullTemplatedTrigger> for TemplatedTrigger {
    fn from(full: FullTemplatedTrigger) -> Self {
        TemplatedTrigger {
            template: full.template,
            name: full.name,
            description: full.description,
            parameters: full.parameters.into_iter().map(|p| p.into()).collect(),
            thresholds: Some(full.thresholds),
            entity_type: full.entity_type,
            granularity: full.granularity,
            weight: full.weight,
            enabled: full.enabled,
        }
    }
}

impl From<TemplatedTrigger> for FullTemplatedTrigger {
    fn from(template: TemplatedTrigger) -> Self {
        let any_thresholds = match template.thresholds.clone() {
            Some(thresholds) => !thresholds.is_empty(),
            None => false,  
        };
        if any_thresholds {
            FullTemplatedTrigger {
                template: template.template,
                name: template.name,
                description: template.description,
                parameters: template.parameters.into_iter().map(|p| p.into()).collect(),
                thresholds: template.thresholds.unwrap(),
                entity_type: template.entity_type,
                granularity: template.granularity,
                weight: template.weight,
                enabled: template.enabled,
            }
        } else {
            let thresholds: Vec<Threshold> = template
                .parameters
                .iter()
                .filter_map(|p| {
                    let parameter = template.template.clone().get_parameter(&p.name);
                    let ptype = match &parameter {
                        Some(param) => param.parameter_type.clone(),
                        None => panic!("Parameter {} not found in template", p.name),
                    };
                    if ptype == ParameterType::ThresholdVariable
                        || ptype == ParameterType::LookbackVariable
                    {
                        Some(Threshold {
                            name: p.value.clone(),
                            data_type: p
                                .data_type
                                .clone()
                                .unwrap_or(DEFAULT_THRESHOLD_DATA_TYPE.to_string()),
                            value: p
                                .default_value
                                .clone()
                                .unwrap_or(DEFAULT_THRESHOLD_VALUE.to_string()),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            FullTemplatedTrigger {
                template: template.template,
                name: template.name,
                description: template.description,
                parameters: template.parameters.into_iter().map(|p| p.into()).collect(),
                thresholds,
                entity_type: template.entity_type,
                granularity: template.granularity,
                weight: template.weight,
                enabled: template.enabled,
            }
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
    #[must_use]
    pub fn get_parameter(self, parameter_name: &str) -> Option<TemplateParameter> {
        self.parameters
            .into_iter()
            .find(|p| p.name == parameter_name)
    }
}

impl FullTemplatedTrigger {
    pub fn check_trigger(&self) -> Result<(), TriggerTemplateError> {
        if let Some(parm) = self.template.parameters.clone().into_iter().find(|p| {
            !self
                .parameters
                .clone()
                .into_iter()
                .any(|pp| pp.name == p.name)
        }) {
            return Err(TriggerTemplateError::MissingParameter(parm.name));
        };
        if let Some(parm) = self
            .parameters
            .clone()
            .into_iter()
            .find(|p| self.template.clone().get_parameter(&p.name).is_none())
        {
            return Err(TriggerTemplateError::ExtraneousParameter(parm.name));
        };
        let needed_thresholds = self.parameters.clone().into_iter().filter(|p| {
            let ptype = self
                .template
                .clone()
                .get_parameter(&p.name)
                .unwrap()
                .parameter_type;
            ptype == ParameterType::ThresholdVariable || ptype == ParameterType::LookbackVariable
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

    #[must_use]
    pub fn adapted_parameter_name(&self, parameter: &ParameterValue) -> String {
        match self
            .template
            .parameters
            .clone()
            .into_iter()
            .find(|p| p.name == parameter.name)
        {
            Some(template_parameter) => match template_parameter.parameter_type {
                ParameterType::Counter => "\"".to_owned() + &parameter.value + "\"",
                _ => parameter.value.clone(),
            },
            None => parameter.value.clone(),
        }
    }

    pub fn adapted_extended_parameter_name(
        &self,
        parameter: &ParameterValue,
        extension: &str,
    ) -> String {
        match self
            .template
            .parameters
            .clone()
            .into_iter()
            .find(|p| p.name == parameter.name)
        {
            Some(template_parameter) => match template_parameter.parameter_type {
                ParameterType::Counter => {
                    "\"".to_owned() + &parameter.value + "_" + extension + "\""
                }
                _ => parameter.value.clone() + "_" + extension,
            },
            None => parameter.value.clone() + "_" + extension,
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
                    .get_parameter(&p.name)
                    .unwrap()
                    .parameter_type
                    == ParameterType::Counter
            })
            .collect();
        let lookback_counters: Vec<ParameterValue> = self
            .parameters
            .clone()
            .into_iter()
            .filter(|p| {
                let tp = self.template.clone().get_parameter(&p.name).unwrap();
                tp.parameter_type == ParameterType::Counter && tp.has_lookback
            })
            .collect();
        let mut counter_source: HashMap<String, String> = HashMap::new();

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
                sources.push(source.to_string());
                counter_source.insert(counter.value.clone(), source);
            };
        }

        let mut kpi_data: Vec<KPIDataColumn> = counters
            .clone()
            .into_iter()
            .map(|c| KPIDataColumn {
                name: c.value,
                data_type: "numeric".to_string(),
            })
            .collect();

        for counter in &lookback_counters {
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_count",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_avg",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_max",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_min",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_sum",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_stddev",
                data_type: "numeric".to_string(),
            });
            kpi_data.push(KPIDataColumn {
                name: counter.value.clone() + "_var",
                data_type: "numeric".to_string(),
            });
        }

        let mut kpi_function =
            concat!("\nBEGIN\n", "  RETURN query EXECUTE $query$\n",).to_string();

        let mut first = true;

        for counter in &lookback_counters {
            let lookback_param = self
                .template
                .clone()
                .get_parameter(&counter.name)
                .unwrap()
                .lookback_parameter
                .clone()
                .unwrap_or("lookback".to_string());
            let mut lookback = "1".to_string();
            if let Some(lookback_param_name) = self
                .parameters
                .clone()
                .into_iter()
                .find(|p| p.name == lookback_param)
            {
                if let Some(lookback_param) = self
                    .thresholds
                    .clone()
                    .into_iter()
                    .find(|t| t.name == lookback_param_name.value)
                {
                    lookback = lookback_param.value.clone();
                }
            }

            if first {
                first = false;
            } else {
                kpi_function.push_str(",\n");
            }
            kpi_function.push_str(&format!(
                "\
    WITH \"{}_past_data\" AS (\n\
      SELECT\n\
        entity_id,\n\
        count(*) AS {}_count,\n\
        avg({}) AS {}_avg,\n\
        max({}) AS {}_max,\n\
        min({}) AS {}_min,\n\
        sum({}) AS {}_sum,\n\
        stddev_samp({}) AS {}_stddev,\n\
        var_samp({}) AS {}_var\n\
    FROM trend.\"{}\"\n\
    WHERE timestamp < $1 AND timestamp >= $1 - '{}d'::interval\n\
    GROUP BY entity_id\n\
    )",
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter_source.get(&counter.value).unwrap(),
                lookback
            ));
        }

        kpi_function.push_str(concat!("    SELECT\n", "      t1.entity_id,\n", "      $1",));
        for counter in &counters {
            kpi_function.push_str(&format!(",\n      \"{}\"::numeric", counter.value));
        }
        for counter in &lookback_counters {
            kpi_function.push_str(&format!(
                "\
,\n      \"{}_past_data\".\"{}_count\"::numeric\
,\n      \"{}_past_data\".\"{}_avg\"::numeric\
,\n      \"{}_past_data\".\"{}_max\"::numeric\
,\n      \"{}_past_data\".\"{}_min\"::numeric\
,\n      \"{}_past_data\".\"{}_sum\"::numeric\
,\n      \"{}_past_data\".\"{}_stddev\"::numeric\
,\n      \"{}_past_data\".\"{}_var\"::numeric",
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
                counter.value,
            ));
        }

        let mut sourcecounter = 1;
        for source in &sources {
            if sourcecounter == 1 {
                kpi_function.push_str(&format!("\n    FROM trend.\"{source}\" t1\n"));
            } else {
                kpi_function.push_str(&format!(
                    "    JOIN trend.{} t{} ON t{}.timestamp = t1.timestamp AND t{}.entity_id = $1\n",
                    escape_identifier(source), sourcecounter, sourcecounter, sourcecounter
                ));
            };
            sourcecounter += 1;
        }
        for counter in &lookback_counters {
            kpi_function.push_str(&format!(
                "    JOIN \"{}_past_data\" ON \"{}_past_data\".entity_id = t1.entity_id\n",
                counter.value, counter.value
            ));
        }
        sourcecounter = 1;
        for _ in &sources {
            if sourcecounter == 1 {
                kpi_function.push_str("    WHERE t1.timestamp = $1");
            } else {
                kpi_function.push_str(&format!(" AND t{sourcecounter}.timestamp = $1"));
            };
            sourcecounter += 1;
        }
        kpi_function.push_str("\n  $query$ USING $1;\nEND;");

        let mut condition = self.template.sql.clone();
        for parameter in &self.parameters {
            condition = condition.replace(
                &("{".to_owned() + &parameter.name + "}"),
                &self.adapted_parameter_name(parameter),
            );
            if self
                .template
                .parameters
                .clone()
                .into_iter()
                .any(|p| p.name == parameter.name && p.has_lookback)
            {
                condition = condition
                    .replace(
                        &("{".to_owned() + &parameter.name + "_count}"),
                        &self.adapted_extended_parameter_name(parameter, "count"),
                    )
                    .replace(
                        &("{".to_owned() + &parameter.name + "_avg}"),
                        &self.adapted_extended_parameter_name(parameter, "avg"),
                    )
                    .replace(
                        &("{".to_owned() + &parameter.name + "_max}"),
                        &self.adapted_extended_parameter_name(parameter, "max"),
                    )
                    .replace(
                        &("{".to_owned() + &parameter.name + "_min}"),
                        &self.adapted_extended_parameter_name(parameter, "min"),
                    )
                    .replace(
                        &("{".to_owned() + &parameter.name + "_sum}"),
                        &self.adapted_extended_parameter_name(parameter, "sum"),
                    )
                    .replace(
                        &("{".to_owned() + &parameter.name + "_stddev}"),
                        &self.adapted_extended_parameter_name(parameter, "stddev"),
                    )
            };
        }

        let mut data_code = format!(
            "SELECT json_build_object(\n  '{}', {}.name",
            self.entity_type,
            escape_identifier(&self.entity_type)
        );
        for counter in &counters {
            data_code.push_str(&format!(
                ",\n  '{}', $1.{}",
                counter.value,
                escape_identifier(&counter.value)
            ));
        }
        data_code.push_str(&format!(
            "\n)\nFROM entity.{} et\nWHERE et.id = $1.entity_id",
            escape_identifier(&self.entity_type)
        ));

        let description = if let Some(value) = &self.description {
            value.to_string()
        } else {
            let mut description_from_template = self.template.description.clone();
            for parameter in &self.parameters {
                description_from_template = description_from_template
                    .replace(&("{".to_owned() + &parameter.name + "}"), &parameter.value);
            }
            description_from_template
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
        "SELECT name, is_variable, is_source_name, has_lookback, lookback_parameter ",
        "FROM trigger.template_parameter WHERE template_id = $1",
    );

    let rows = conn
        .query(query, &[&id])
        .await
        .map_err(|e| TriggerTemplateError::DatabaseError(DatabaseError::from_msg(e.to_string())))?;

    let mut parameters: Vec<TemplateParameter> = rows
        .clone()
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
            has_lookback: row.get(2) && row.get(3),
            lookback_parameter: row.get(4),
        })
        .collect();

    for row in &rows {
        if row.get(2) && row.get(3) {
            let new_parameter = TemplateParameter {
                name: row.get(4),
                parameter_type: ParameterType::LookbackVariable,
                has_lookback: false,
                lookback_parameter: None,
            };
            if !parameters.iter().any(|p| {
                p.name == new_parameter.name && p.parameter_type == ParameterType::LookbackVariable
            }) {
                parameters.push(new_parameter);
            }
        }
    }

    Ok(Template {
        id: bare_template.id,
        name: bare_template.name,
        description: bare_template.body,
        sql: bare_template.sql_body,
        parameters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templated_trigger_to_full_templated_trigger() {
        let template = Template {
            id: 1,
            name: "Test Template".to_string(),
            description: "Trigger when {trend} {comparison} {value}".to_string(),
            sql: "{trend} {comparison} {value}".to_string(),
            parameters: vec![TemplateParameter {
                name: "trend".to_string(),
                parameter_type: ParameterType::Counter,
                has_lookback: false,
                lookback_parameter: None,
            },
            TemplateParameter {
                name: "comparison".to_string(),
                parameter_type: ParameterType::Default,
                has_lookback: false,
                lookback_parameter: None,
            },
            TemplateParameter {
                name: "value".to_string(),
                parameter_type: ParameterType::ThresholdVariable,
                has_lookback: false,
                lookback_parameter: None,
            }],
        };
        let templated_trigger1 = TemplatedTrigger {
            template: template.clone(),
            name: "Test Trigger 1".to_string(),
            description: Some("A test trigger".to_string()),
            parameters: vec![
                ExtendedParameterValue {
                    name: "trend".to_string(),
                    value: "cpu_usage".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "comparison".to_string(),
                    value: ">".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "value".to_string(),
                    value: "min_usage".to_string(),
                    data_type: Some("numeric".to_string()),
                    default_value: Some("80".to_string()),
                },
            ],
            thresholds: None,
            entity_type: "server".to_string(),
            granularity: Duration::from_secs(900),
            weight: 10,
            enabled: true,
        };
        let full_templated_trigger1: FullTemplatedTrigger =
            templated_trigger1.clone().into();
        assert_eq!(full_templated_trigger1.thresholds.len(), 1);
        assert_eq!(full_templated_trigger1.thresholds[0].name, "min_usage");
        assert_eq!(full_templated_trigger1.thresholds[0].data_type, "numeric");
        assert_eq!(full_templated_trigger1.thresholds[0].value, "80");

        let templated_trigger2 = TemplatedTrigger {
            template: template.clone(),
            name: "Test Trigger 1".to_string(),
            description: Some("A test trigger".to_string()),
            parameters: vec![
                ExtendedParameterValue {
                    name: "trend".to_string(),
                    value: "cpu_usage".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "comparison".to_string(),
                    value: ">".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "value".to_string(),
                    value: "min_usage".to_string(),
                    data_type: None,
                    default_value: None,
                },
            ],
            thresholds: Some(vec![Threshold {
                name: "min_usage".to_string(),
                data_type: "numeric".to_string(),
                value: "80".to_string(),
            }]),
            entity_type: "server".to_string(),
            granularity: Duration::from_secs(900),
            weight: 10,
            enabled: true,
        };        
        let full_templated_trigger2: FullTemplatedTrigger =
            templated_trigger2.clone().into();
        assert_eq!(full_templated_trigger2.thresholds.len(), 1);
        assert_eq!(full_templated_trigger2.thresholds[0].name, "min_usage");
        assert_eq!(full_templated_trigger2.thresholds[0].data_type, "numeric");
        assert_eq!(full_templated_trigger2.thresholds[0].value, "80");

        let templated_trigger3 = TemplatedTrigger {
            template: template.clone(),
            name: "Test Trigger 1".to_string(),
            description: Some("A test trigger".to_string()),
            parameters: vec![
                ExtendedParameterValue {
                    name: "trend".to_string(),
                    value: "cpu_usage".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "comparison".to_string(),
                    value: ">".to_string(),
                    data_type: None,
                    default_value: None,
                },
                ExtendedParameterValue {
                    name: "value".to_string(),
                    value: "min_usage".to_string(),
                    data_type: Some("numeric".to_string()),
                    default_value: Some("80".to_string()),
                },
            ],
            thresholds: Some(vec![]),
            entity_type: "server".to_string(),
            granularity: Duration::from_secs(900),
            weight: 10,
            enabled: true,
        };
        let full_templated_trigger3: FullTemplatedTrigger =
            templated_trigger3.clone().into();
        assert_eq!(full_templated_trigger3.thresholds.len(), 1);
        assert_eq!(full_templated_trigger3.thresholds[0].name, "min_usage");
        assert_eq!(full_templated_trigger3.thresholds[0].data_type, "numeric");
        assert_eq!(full_templated_trigger3.thresholds[0].value, "80");

    }
}