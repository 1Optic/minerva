#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::path::PathBuf;

    use log::debug;

    use minerva::change::Change;

    use minerva::changes::trend_store::AddTrendStore;
    use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
    use minerva::schema::create_schema;
    use minerva::trend_materialization::get_function_def;
    use minerva::trend_store::TrendStore;

    use crate::common::get_available_port;
    use crate::common::{MinervaService, MinervaServiceConfig};

    const TREND_STORE_DEFINITION: &str = r###"
    title: Raw node data
    data_source: hub
    entity_type: node
    granularity: 15m
    partition_size: 1d
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
    "###;

    #[tokio::test]
    async fn create_trigger() -> Result<(), Box<dyn std::error::Error>> {
        crate::setup();

        let cluster_config = MinervaClusterConfig {
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let test_database = cluster.create_db().await?;

        debug!("Created database '{}'", test_database.name);

        {
            let mut client = test_database.connect().await?;
            create_schema(&mut client).await?;

            let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
                .map_err(|e| format!("Could not read trend store definition: {}", e))?;

            let add_trend_store = AddTrendStore { trend_store };

            let mut tx = client.transaction().await?;

            add_trend_store.apply(&mut tx).await?;

            tx.execute("INSERT INTO trigger.template (name, description_body, sql_body) VALUES ('first template', 'compare counter to value', '{counter} {comparison} {value}');", &[]).await?;

            tx.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'counter', false, true from trigger.template WHERE name = 'first template';", &[]).await?;

            tx.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'comparison', false, false from trigger.template WHERE name = 'first template';", &[]).await?;

            tx.execute("INSERT INTO trigger.template_parameter (template_id, name, is_variable, is_source_name) SELECT id, 'value', true, false from trigger.template WHERE name = 'first template';", &[]).await?;

            tx.commit().await?;
        }

        let service_address = Ipv4Addr::new(127, 0, 0, 1);
        let service_port = get_available_port(service_address).unwrap();

        let service_conf = MinervaServiceConfig {
            pg_host: cluster.controller_host.to_string(),
            pg_port: cluster.controller_port.to_string(),
            pg_sslmode: "disable".to_string(),
            pg_database: test_database.name.to_string(),
            service_address: service_address.to_string(),
            service_port,
        };

        let mut service = MinervaService::start(service_conf)?;

        println!("Started service");

        service.wait_for().await?;

        let address = format!("{service_address}:{service_port}");

        let client = reqwest::Client::new();

        let url = format!("http://{address}/triggers");
        let request_body = r#"{
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
        "template_id": 1,
        "parameters": [
            {
                "parameter": "counter",
                "value": "inside_temp"
            },
            {
                "parameter": "comparison",
                "value": "<"
            },
            {
                "parameter": "value",
                "value": "min_temperature"
            }
        ]
    }
}"#;

        let response = client.post(url.clone()).body(request_body).send().await?;
        let body = response.text().await?;

        assert_eq!(
            body,
            "{\"code\":200,\"message\":\"Created trigger 'low_temperature'\"}"
        );

        let response = client.get(url).send().await?;
        let body = response.text().await?;

        assert_eq!(
            body,
            "[{\"name\":\"low_temperature\",\"enabled\":true,\"description\":\"inside temperature low\",\"thresholds\":[{\"name\":\"min_temperature\",\"data_type\":\"numeric\",\"value\":\"10\"}]}]"
        );

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

        //assert_eq!(language, "plpgsql");

        //let expected_src = concat!(
        //    "\nBEGIN\n",
        //    "RETURN QUERY EXECUTE $query$\n",
        //    "SELECT\n",
        //    "  t1.entity_id,\n",
        //    "  $1 AS timestamp,\n",
        //    "  inside_temp - outside_temp AS \"test-kpi-name\"\n",
        //    "FROM trend.\"hub_node_main_15m\" t1\n",
        //    "WHERE t1.timestamp = $1\n",
        //    "GROUP BY t1.entity_id\n",
        //    "$query$ USING $1;\n",
        //    "END;\n\n"
        //);

        //assert_eq!(src, expected_src);

        Ok(())
    }
}
