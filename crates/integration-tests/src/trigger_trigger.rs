use log::info;
use minerva::trend_store::{create_partitions_for_timestamp, TrendStore};
use minerva::trigger::{AddTrigger, CreateNotifications, Trigger};
use serde_json::{json, Value};

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::MinervaClusterConnector;
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

const TRIGGER_DEFINITION: &str = r###"
name: HeatingUp
kpi_data:
  - name: temp_inside
    data_type: numeric
  - name: temp_outside
    data_type: numeric
  - name: temp_differential
    data_type: numeric
kpi_function: |-
  BEGIN
    RETURN QUERY EXECUTE $query$
    SELECT
        entity_id,
        $1 AS timestamp,
        inside_temp AS temp_inside,
        outside_temp AS temp_outside,
        inside_temp - outside_temp AS temp_differential
    FROM trend.hub_node_main_15m
    WHERE timestamp = $1;
    $query$ USING $1;
  END;
thresholds:
  - name: temp_inside_max
    data_type: numeric
    value: 30
  - name: temp_differential_max
    data_type: numeric
    value: 10
condition: |-
  temp_inside > temp_inside_max AND temp_differential > temp_differential_max
weight: |-
  SELECT 10
notification: |-
  SELECT 'Obsolete'
data: |-
  SELECT json_build_object(
    'entity', $1.entity_id,
    'temp_inside', $1.temp_inside,
    'temp_outside', $1.temp_outside,
    'temp_differential', $1.temp_differential
  )
tags: []
fingerprint: |-
  SELECT trigger.modified_to_fingerprint(
    ARRAY[
    trend.modified(trend.to_trendstore('trend.hub_node_main_15m'), $1)
    ]::timestamptz[]
  )
notification_store: trigger-notification
trend_store_links:
  - part_name: hub_node_main_15m
    mapping_function: mapping_id
mapping_functions: []
granularity: 15m
description: Test trigger
enabled: true
"###;

pub async fn trigger_trigger_notifications(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_database = cluster.create_db().await?;

    info!("Created database '{}'", test_database.name);

    {
        let mut client = test_database.connect().await?;
        create_schema(&mut client).await?;

        client
            .execute(
                "SELECT trigger.create_trigger_notification_store('trigger-notification')",
                &[],
            )
            .await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_trend_store = AddTrendStore {
            trend_store: trend_store.clone(),
        };

        let trigger: Trigger = serde_yaml::from_str(TRIGGER_DEFINITION)
            .map_err(|e| format!("Could not read trigger definition: {e}"))?;

        let create_trigger = AddTrigger {
            trigger: trigger.clone(),
            verify: false,
        };

        let timestamp = chrono::DateTime::parse_from_rfc3339("2024-12-12T09:15:00+00:00").unwrap();

        let create_notifications = CreateNotifications {
            trigger_name: trigger.name,
            timestamp: timestamp.into(),
        };

        add_trend_store.apply(&mut client).await?;
        create_trigger.apply(&mut client).await?;
        create_partitions_for_timestamp(&mut client, timestamp.into()).await?;

        client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (1, '2024-12-12T09:15:00+00:00', now(), 42, 19.2, 29.5, 34, 559)", &[]).await?;
        client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (2, '2024-12-12T09:15:00+00:00', now(), 42, 19.3, 30.5, 34, 559)", &[]).await?;
        client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (3, '2024-12-12T09:15:00+00:00', now(), 42, 25.0, 36.1, 34, 559)", &[]).await?;
        client.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (4, '2024-12-12T09:15:00+00:00', now(), 42, 24.0, 31.9, 34, 559)", &[]).await?;

        create_notifications.apply(&mut client).await?;

        let notification_rows = client
            .query(
                "SELECT entity_id, data FROM notification.\"trigger-notification\"",
                &[],
            )
            .await?;
        let notification_entities: Vec<i32> = notification_rows
            .clone()
            .into_iter()
            .map(|row| row.get(0))
            .collect();
        assert_eq!(notification_entities, vec![2, 3]);

        let expected_data1 = json!({"entity": 2, "temp_inside": 30.5, "temp_outside": 19.3, "temp_differential": 11.2});
        let expected_data2 = json!({"entity": 3, "temp_inside": 36.1, "temp_outside": 25.0, "temp_differential": 11.1});
        let notification_data: Vec<Value> = notification_rows
            .into_iter()
            .map(|row| row.get(1))
            .collect();
        assert_eq!(notification_data, vec![expected_data1, expected_data2]);
    }

    Ok(())
}
