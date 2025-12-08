use std::net::Ipv4Addr;

use log::debug;

use minerva::change::Change;

use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::MinervaClusterConnector;
use minerva::trend_materialization::get_function_def;
use minerva::trend_store::TrendStore;
use serde_json::json;

use crate::common::{
    create_schema_with_retry, create_webservice_role, get_available_port, MinervaService,
    MinervaServiceConfig,
};

const TREND_STORE_DEFINITION: &str = r"
title: Raw node data
data_source: hub
entity_type: node
granularity: 15m
partition_size: 1d
retention_period: 6 months
parts:
  - name: hub_node_main_15m
    trends:
      - name: outside_temp
        data_type: numeric
      - name: inside_temp
        data_type: numeric
      - name: power_kwh
        data_type: numeric
      - name: freq_power
        data_type: numeric
";

pub async fn create_trigger(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let webservice_role = "webservice";
    let test_database = cluster.create_db().await?;

    debug!("Created database '{}'", test_database.name);

    let trigger_template_id = {
        let mut client = test_database.connect().await?;
        create_schema_with_retry(&mut client, 5).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore { trend_store };

        add_trend_store.apply(&mut client).await?;

        let row = client.query_one("INSERT INTO trigger.template (name, description_body, sql_body) VALUES ('first template', 'compare counter to value', '{counter} {comparison} {value}') RETURNING id;", &[]).await?;

        let trigger_template_id: i32 = row.get(0);

        client.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'counter', false, true from trigger.template WHERE name = 'first template';", &[]).await?;

        client.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'comparison', false, false from trigger.template WHERE name = 'first template';", &[]).await?;

        client.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'value', true, false from trigger.template WHERE name = 'first template';", &[]).await?;

        create_webservice_role(&cluster, webservice_role).await?;

        trigger_template_id
    };

    let service_address = Ipv4Addr::new(127, 0, 0, 1);
    let service_port = get_available_port(service_address).unwrap();

    let service_conf = MinervaServiceConfig {
        pg_host: cluster.coordinator_connector.host.to_string(),
        pg_port: cluster.coordinator_connector.port.to_string(),
        pg_sslmode: "disable".to_string(),
        pg_database: test_database.name.to_string(),
        pg_user: webservice_role.to_string(),
        service_address: service_address.to_string(),
        service_port,
    };

    let mut service = MinervaService::start(service_conf)?;

    println!("Started service");

    service.wait_for().await?;

    let address = format!("{service_address}:{service_port}");

    let client = reqwest::Client::new();

    let url = format!("http://{address}/triggers");
    let request_data = json!({
        "name": "low_temperature",
        "description": "inside temperature low",
        "thresholds": [
            {
                "name": "min_temperature",
                "data_type": "numeric",
                "value": "10"
            }
        ],
        "entity_type": "node",
        "granularity": "15m",
        "weight": 100,
        "enabled": true,
        "template_instance": {
            "template_id": trigger_template_id,
            "parameters": [
                {
                    "name": "counter",
                    "value": "inside_temp"
                },
                {
                    "name": "comparison",
                    "value": "<"
                },
                {
                    "name": "value",
                    "value": "min_temperature"
                }
            ]
        }
    });

    let response = client.post(url.clone()).json(&request_data).send().await?;

    let response_data: serde_json::Value = response.json().await?;

    let trigger_id = response_data
        .get("id")
        .and_then(|v| v.as_i64())
        .expect("expected response to contain trigger id");

    assert!(trigger_id > 0);

    let response = client.get(url).send().await?;
    let response_data: serde_json::Value = response.json().await?;

    let expected_response = json!([
        {
            "name": "low_temperature",
            "enabled": true,
            "description": "inside temperature low",
            "thresholds": [
                {
                    "name": "min_temperature",
                    "data_type": "numeric",
                    "value": "10"
                }
            ]
        }
    ]);

    assert_eq!(response_data, expected_response);

    let (_, src): (String, String) = {
        let mut client = test_database.connect().await?;

        get_function_def(&mut client, "low_temperature")
            .await
            .unwrap()
    };

    assert_eq!(
        src.trim(),
        "SELECT * FROM trigger_rule.low_temperature_with_threshold($1) WHERE \"inside_temp\" < min_temperature;"
    );

    Ok(())
}
