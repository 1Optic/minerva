#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use log::{debug, info};
    use minerva::trend_materialization::{
        AddTrendMaterialization, TrendFunctionMaterialization, TrendMaterialization,
    };
    use std::env;
    use std::path::PathBuf;
    use std::time::Duration;

    use assert_cmd::cargo::cargo_bin;
    use tokio::io::{AsyncBufReadExt, BufReader};
    use tokio::process::Command;

    use minerva::change::Change;
    use minerva::changes::trend_store::AddTrendStore;
    use minerva::cluster::MinervaCluster;
    use minerva::schema::create_schema;
    use minerva::trend_store::{create_partitions_for_timestamp, TrendStore};

    const TREND_STORE_DEFINITION: &str = r###"title: Raw node data
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

    const TARGET_TREND_STORE_DEFINITION: &str = r###"title: Raw node data
data_source: hub
entity_type: node
granularity: 1h
partition_size: 1d
parts:
  - name: hub_node_main_1h
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

    const MATERIALIZATION: &str = r###"
target_trend_store_part: hub_node_main_1h
enabled: true
processing_delay: 30m
stability_delay: 5m
reprocessing_period: 3 days
sources:
- trend_store_part: hub_node_main_15m
  mapping_function: trend.mapping_15m->1h
function:
  return_type: |
    TABLE (
      "entity_id" integer,
      "timestamp" timestamp with time zone,
      samples smallint,
      "outside_temp" numeric,
      "inside_temp" numeric,
      "power_kwh" numeric,
      "freq_power" numeric
    )
  src: |
    BEGIN
    RETURN QUERY EXECUTE $query$
        SELECT
          entity_id,
          $2 AS timestamp,
          (count(*))::smallint AS samples,
          SUM(t."outside_temp")::numeric AS "outside_temp",
          SUM(t."inside_temp")::numeric AS "inside_temp",
          SUM(t."power_kwh")::numeric AS "power_kwh",
          SUM(t."freq_power")::numeric AS "freq_power"
        FROM trend."hub_node_main_15m" AS t
        WHERE $1 < timestamp AND timestamp <= $2
        GROUP BY entity_id
    $query$ USING $1 - interval '1h', $1;
    END;
  language: plpgsql
fingerprint_function: |
  SELECT max(modified.last), format('{%s}', string_agg(format('"%s":"%s"', t, modified.last), ','))::jsonb
  FROM generate_series($1 - interval '1h' + interval '15m', $1, interval '15m') t
  LEFT JOIN (
    SELECT timestamp, last
    FROM trend_directory.trend_store_part part
    JOIN trend_directory.modified ON modified.trend_store_part_id = part.id
    WHERE part.name = 'hub_node_main_15m'
  ) modified ON modified.timestamp = t;
description: {}
"###;

    #[tokio::test]
    async fn materialize_service() -> Result<(), Box<dyn std::error::Error>> {
        crate::setup();

        let config_file = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/postgresql.conf"));

        let cluster = MinervaCluster::start(&config_file, 3).await?;

        let test_database = cluster.create_db().await?;

        info!("Created database '{}'", test_database.name);

        let mut client = test_database.connect().await?;
        create_schema(&mut client).await?;

        let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {}", e))?;

        let add_trend_store = AddTrendStore {
            trend_store: trend_store.clone(),
        };

        let target_trend_store: TrendStore = serde_yaml::from_str(TARGET_TREND_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {}", e))?;

        let add_target_trend_store = AddTrendStore {
            trend_store: target_trend_store.clone(),
        };

        let materialization: TrendFunctionMaterialization =
            serde_yaml::from_str(MATERIALIZATION)
                .map_err(|e| format!("Could not read materialization definition: {}", e))?;

        let add_materialization = AddTrendMaterialization {
            trend_materialization: TrendMaterialization::Function(materialization),
        };

        let mut tx = client.transaction().await?;

        let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339("2024-12-12T09:15:00+00:00")
            .unwrap()
            .to_utc();

        add_trend_store.apply(&mut tx).await?;
        add_target_trend_store.apply(&mut tx).await?;
        add_materialization.apply(&mut tx).await?;

        create_partitions_for_timestamp(&mut tx, timestamp).await?;

        tx.execute("INSERT INTO trend.\"hub_node_main_15m\"(entity_id, timestamp, created, job_id, outside_temp, inside_temp, power_kwh, freq_power) VALUES (1, '2024-12-12T09:15:00+00:00', now(), 42, 4.5, 19.2, 34, 559)", &[]).await?;
        tx.execute("INSERT INTO trend_directory.modified(trend_store_part_id, timestamp, first, last) SELECT id, '2024-12-12T09:15:00+00:00', '2024-12-12T09:21:33+00:00', '2024-12-12T09:26:56+00:00' FROM trend_directory.trend_store_part tsp WHERE tsp.name = 'hub_node_main_15m'", &[]).await?;

        tx.commit().await?;

        let executable_path = cargo_bin("minerva");
        let mut cmd = Command::new(executable_path.clone())
            .stdout(std::process::Stdio::piped())
            .env("RUST_LOG", "debug")
            .env("PGSSLMODE", "disable")
            .env("PGUSER", "postgres")
            .env("PGHOST", cluster.controller_host.to_string())
            .env("PGPORT", cluster.controller_port.to_string())
            .env("PGDATABASE", &test_database.name)
            .arg("trend-materialization")
            .arg("service")
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to execute process");

        let stdout = cmd.stdout.take().expect("could not take stdout");

        let mut buf_reader = BufReader::new(stdout).lines();

        let output_check_handle = tokio::spawn(async move {
            while let Some(line) = buf_reader.next_line().await.unwrap() {
                debug!("RUN1: {}", line);

                if line.contains("hub_node_main_1h: 1") {
                    return;
                }
            }
        });

        let check_result = tokio::select! {
            _ = output_check_handle => { "Ok".to_string() },
            _ = tokio::time::sleep(Duration::from_millis(15000)) => { "Timeout".to_string() },
        };

        println!("Done running service: {:?}", check_result);

        Ok(())
    }
}
