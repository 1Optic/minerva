use std::collections::HashMap;

use async_trait::async_trait;
use clap::Parser;

use tokio_postgres::GenericClient;

use crate::commands::common::{connect_db, Cmd, CmdResult};
use minerva::trend_materialization::{
    get_function_result_columns, load_materializations, ResultColumn, TrendFunctionMaterialization,
    TrendMaterialization, MATERIALIZATION_FUNCTION_SCHEMA,
};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationCheck {}

#[async_trait]
impl Cmd for TrendMaterializationCheck {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client).await?;

        for materialization in materializations {
            let issues = match materialization {
                TrendMaterialization::View(ref _view_materialization) => Vec::new(),
                TrendMaterialization::Function(ref function_materialization) => {
                    check_function_materialization(&mut client, function_materialization)
                        .await
                        .unwrap()
                }
            };

            if issues.is_empty() {
                println!("'{}': Ok", &materialization);
            } else {
                println!("'{}':", &materialization);
                for issue in issues {
                    println!(" - {}", issue);
                }
            }
        }

        Ok(())
    }
}

pub async fn check_function_materialization<T: GenericClient + Send + Sync>(
    client: &mut T,
    function_materialization: &TrendFunctionMaterialization,
) -> Result<Vec<String>, String> {
    let mut report: Vec<String> = Vec::new();
    let function_result_columns: Vec<ResultColumn> = get_function_result_columns(
        client,
        MATERIALIZATION_FUNCTION_SCHEMA,
        &function_materialization.target_trend_store_part,
    )
    .await
    .unwrap()
    .into_iter()
    .filter(|c| c.name != "entity_id" && c.name != "timestamp")
    .collect();

    let function_result_columns_map: HashMap<String, String> = function_result_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    let trend_store_part_columns =
        get_trend_store_part_columns(client, &function_materialization.target_trend_store_part)
            .await
            .unwrap();

    let trend_store_part_columns_map: HashMap<String, String> = trend_store_part_columns
        .iter()
        .map(|c| (c.name.clone(), c.data_type.clone()))
        .collect();

    for function_result_column in function_result_columns {
        let trend_data_type = trend_store_part_columns_map.get(&function_result_column.name);

        match trend_data_type {
            None => {
                report.push(format!(
                    "Column '{}'({}) is returned from function but has no matching trend",
                    function_result_column.name, function_result_column.data_type
                ));
            }
            Some(data_type) => {
                if !data_type.eq(&function_result_column.data_type) {
                    report.push(format!(
                        "Column '{}'({}) returned from function differs in type: '{}' != '{}' ",
                        function_result_column.name,
                        function_result_column.data_type,
                        function_result_column.data_type,
                        data_type
                    ));
                }
            }
        }
    }

    for trend_store_part_column in trend_store_part_columns {
        let function_result_column_data_type =
            function_result_columns_map.get(&trend_store_part_column.name);

        if function_result_column_data_type.is_none() {
            report.push(format!(
                "Column '{}'({}) is defined as trend in trend store part '{}' but is not returned from function",
                trend_store_part_column.name, trend_store_part_column.data_type, function_materialization.target_trend_store_part
            ));
        }
    }

    Ok(report)
}

pub struct TrendStorePartColumn {
    pub name: String,
    pub data_type: String,
}

pub async fn get_trend_store_part_columns<T: GenericClient + Send + Sync>(
    client: &mut T,
    trend_store_part_name: &str,
) -> Result<Vec<TrendStorePartColumn>, String> {
    let query = concat!(
        "SELECT tt.name, tt.data_type ",
        "FROM trend_directory.trend_store_part tsp ",
        "JOIN trend_directory.table_trend tt ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = $1"
    );

    let columns: Vec<TrendStorePartColumn> = client
        .query(query, &[&trend_store_part_name])
        .await
        .map(|rows| {
            rows.iter()
                .map(|row| TrendStorePartColumn {
                    name: row.get(0),
                    data_type: row.get(1),
                })
                .collect()
        })
        .map_err(|e| format!("could not retrieve columns for trend store part: {e}"))?;

    Ok(columns)
}
