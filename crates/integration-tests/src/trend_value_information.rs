#[cfg(test)]
mod tests {
    use log::info;
    use minerva::trend_store::{create_partitions_for_timestamp, TrendStore};
    use std::env;
    use std::path::PathBuf;

    use minerva::change::Change;
    use minerva::change::InformationOption;
    use minerva::changes::trend_store::{AddTrendStore, TrendValueInformation};
    use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
    use minerva::schema::create_schema;

    const TREND_STORE_DEFINITION: &str = r###"
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
        generated_trends:
          - name: power_Mwh
            data_type: numeric
            description: test
            expression: power_kwh / 1000

    "###;

    #[tokio::test]
    async fn trend_value_information() -> Result<(), Box<dyn std::error::Error>> {
        crate::setup();

        let cluster_config = MinervaClusterConfig {
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let test_database = cluster.create_db().await?;

        info!("Created database '{}'", test_database.name);

        {
            let mut client = test_database.connect().await?;
            create_schema(&mut client).await?;

            let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
                .map_err(|e| format!("Could not read trend store definition: {}", e))?;

            let add_trend_store = AddTrendStore {
                trend_store: trend_store.clone(),
            };

            let timestamp =
                chrono::DateTime::parse_from_rfc3339("2024-12-12T09:15:00+00:00").unwrap();

            add_trend_store.apply(&mut client).await?;
            create_partitions_for_timestamp(&mut client, timestamp.into()).await?;

            client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (1, '2024-12-12T09:15:00+00:00', now(), 42, 19.2, 29.5, 34, null)", &[]).await?;
            client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (2, '2024-12-12T09:15:00+00:00', now(), 42, 19.3, 30.5, 34, null)", &[]).await?;
            client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (3, '2024-12-12T09:15:00+00:00', now(), 42, 25.0, 36.1, 34, null)", &[]).await?;
            client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (4, '2024-12-12T09:15:00+00:00', now(), 42, 24.0, 31.9, 34, null)", &[]).await?;

            let info_provider = TrendValueInformation {
                trend_store_part_name: "hub_node_main_15m".to_string(),
                trend_names: vec!["outside_temp".to_string(), "freq_power".to_string()],
            };

            let info = info_provider.retrieve(&mut client).await;

            for line in info {
                println!("{}", line);
            }
        }

        Ok(())
    }
}
