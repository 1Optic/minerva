use std::net::Ipv4Addr;

use log::debug;

use minerva::change::Change;

use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::MinervaClusterConnector;
use minerva::trend_store::TrendStore;

use crate::common::{create_schema_with_retry, create_webservice_role, get_available_port};
use crate::common::{MinervaService, MinervaServiceConfig};

const TREND_STORE_DEFINITION_15M: &str = r"
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

const TREND_STORE_DEFINITION_1D: &str = r"
title: Raw node data
data_source: hub
entity_type: node
granularity: 1d
partition_size: 1d
retention_period: 6 months
parts:
  - name: hub_node_main_1d
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

pub async fn create_kpi(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    use minerva::trend_materialization::get_function_def;

    let webservice_role = "webservice";
    let test_database = cluster.create_db().await?;

    debug!("Created database '{}'", test_database.name);

    {
        let mut client = test_database.connect().await?;
        create_schema_with_retry(&mut client, 5).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION_15M)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore { trend_store };

        add_trend_store.apply(&mut client).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION_1D)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore { trend_store };

        add_trend_store.apply(&mut client).await?;

        create_webservice_role(&cluster, webservice_role).await?;
    }

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

    let url = format!("http://{address}/kpis");
    let request_body = r#"{
"tsp_name": "test-kpi",
"kpi_name": "test-kpi-name",
"entity_type": "node",
"data_type": "numeric",
"enabled": true,
"source_trends": ["inside_temp"],
"definition": "inside_temp - outside_temp",
"description": {
  "factors": [
      [
        {
            "type": "trend",
            "value": "inside_temp"
        },
        {
            "type": "operator",
            "value": "-"
        },
        {
            "type": "trend",
            "value": "outside_temp"
        }
      ]
  ],
  "numberdenominator": 1,
  "numbernumerator": 1,
  "type": "Sum"
}
}"#;

    let response = client.post(url).body(request_body).send().await?;

    let body = response.text().await?;

    assert_eq!(
        body,
        "{\"code\":200,\"message\":\"Successfully created KPI\"}"
    );

    let (language, src): (String, String) = {
        let mut client = test_database.connect().await?;

        get_function_def(&mut client, "kpi-test-kpi_node_15m")
            .await
            .unwrap()
    };

    assert_eq!(language, "plpgsql");

    let expected_src = concat!(
        "BEGIN\n",
        "RETURN QUERY EXECUTE $query$\n",
        "SELECT\n",
        "  t1.entity_id,\n",
        "  $1 AS timestamp,\n",
        "  inside_temp - outside_temp AS \"test-kpi-name\"\n",
        "FROM trend.\"hub_node_main_15m\" t1\n",
        "WHERE t1.timestamp = $1\n",
        "GROUP BY t1.entity_id\n",
        "$query$ USING $1;\n",
        "END;"
    );

    assert_eq!(src.trim(), expected_src);

    Ok(())
}
