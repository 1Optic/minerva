use log::debug;

use minerva::change::Change;
use minerva::changes::trend_store::{AddTrendStore, CreateStatistics};
use minerva::cluster::MinervaClusterConnector;
use minerva::schema::create_schema;
use minerva::trend_store::TrendStore;

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

pub async fn test_statistics(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_database = cluster.create_db().await?;

    debug!("Created database '{}'", test_database.name);

    let mut client = test_database.connect().await?;
    create_schema(&mut client).await?;

    let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
        .map_err(|e| format!("Could not read trend store definition: {e}"))?;

    let add_trend_store = AddTrendStore { trend_store };

    add_trend_store.apply(&mut client).await?;

    let insert_command = concat!(
        "INSERT INTO trend.hub_node_main_15m",
        "(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) ",
        "VALUES ",
        "(1, '2024-12-12T09:15:00+00:00', now(), 42, 4.5, 19.2, 34, 559), ",
        "(1, '2024-12-12T09:15:15+00:00', now(), 42, 5.5, 19.4, 34, 559), ",
        "(1, '2024-12-12T09:15:30+00:00', now(), 42, 5.5, 19.6, 34, 559), ",
        "(1, '2024-12-12T09:15:45+00:00', now(), 42, 4.5, NULL, 34, 559), ",
    );
    client.execute(insert_command, &[]).await?;

    let create_statistics = CreateStatistics {
        trend_store_part_name: Some("trend.hub_node_main_15m".to_string()),
    };
    create_statistics.apply(&mut client).await?;

    let command = concat!(
        "SELECT min, max FROM trend_directory.table_trend_statistics tts ",
        "JOIN trend_directory.table trend tt ON tts.table_trend_id = tt.id ",
        "JOIN trend_directory.trend_store_part tsp ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = 'hub_node_main_15m' AND tt.name = 'outside_temp'",
    );
    let row = client.query_one(command, &[]).await?;
    let min: u32 = row.get(0);
    let max: u32 = row.get(1);
    assert_eq!(min, 4);
    assert_eq!(max, 7);

    let command = concat!(
        "SELECT min, max FROM trend_directory.table_trend_statistics tts ",
        "JOIN trend_directory.table trend tt ON tts.table_trend_id = tt.id ",
        "JOIN trend_directory.trend_store_part tsp ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = 'hub_node_main_15m' AND tt.name = 'inside_temp'",
    );
    let row = client.query_one(command, &[]).await?;
    let min: u32 = row.get(0);
    let max: u32 = row.get(1);
    assert_eq!(min, 17);
    assert_eq!(max, 22);

    let insert_command = concat!(
        "INSERT INTO trend.hub_node_main_15m",
        "(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) ",
        "VALUES ",
        "(1, '2024-12-12T09:16:00+00:00', now(), 42, 7.5, 19.8, 34, 559), ",
        "(1, '2024-12-12T09:16:15+00:00', now(), 42, 8.5, 19.8, 34, 559), ",
        "(1, '2024-12-12T09:16:30+00:00', now(), 42, 8.5, 19.8, 34, 559), ",
        "(1, '2024-12-12T09:16:45+00:00', now(), 42, 7.5, 19.6, 34, 559), ",
    );
    client.execute(insert_command, &[]).await?;

    let create_statistics = CreateStatistics {
        trend_store_part_name: None,
    };
    create_statistics.apply(&mut client).await?;

    let command = concat!(
        "SELECT min, max FROM trend_directory.table_trend_statistics tts ",
        "JOIN trend_directory.table trend tt ON tts.table_trend_id = tt.id ",
        "JOIN trend_directory.trend_store_part tsp ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = 'hub_node_main_15m' AND tt.name = 'outside_temp'",
    );
    let row = client.query_one(command, &[]).await?;
    let min: u32 = row.get(0);
    let max: u32 = row.get(1);
    assert_eq!(min, 4);
    assert_eq!(max, 7);

    let command = concat!(
        "SELECT min, max FROM trend_directory.table_trend_statistics tts ",
        "JOIN trend_directory.table trend tt ON tts.table_trend_id = tt.id ",
        "JOIN trend_directory.trend_store_part tsp ON tt.trend_store_part_id = tsp.id ",
        "WHERE tsp.name = 'hub_node_main_15m' AND tt.name = 'inside_temp'",
    );
    let row = client.query_one(command, &[]).await?;
    let min: u32 = row.get(0);
    let max: u32 = row.get(1);
    assert_eq!(min, 17);
    assert_eq!(max, 22);

    Ok(())
}
