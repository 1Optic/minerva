#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use log::info;
    use std::iter::zip;
    use std::path::PathBuf;
    use std::process::ExitStatus;
    use std::{env, io};

    use assert_cmd::cargo::cargo_bin;
    use tokio::io::{AsyncBufReadExt, BufReader};
    use tokio::process::Command;

    use minerva::attribute_store::{AddAttributeStore, AttributeStore};
    use minerva::change::Change;
    use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
    use minerva::schema::create_schema;

    const ATTRIBUTE_STORE_DEFINITION: &str = r"
data_source: hub
entity_type: node
attributes:
- name: name
  data_type: text
  unit: null
  description: null
  extra_data: null
- name: equipment_type
  data_type: text
  unit: null
  description: The equipment type
  extra_data: null
- name: equipment_serial
  data_type: text
  unit: null
  description: The manufacturer serial number of the equipment
  extra_data: null
- name: longitude
  data_type: real
  unit: null
  description: Coordinate of equipment location
  extra_data: null
- name: latitude
  data_type: real
  unit: null
  description: Coordinate of equipment location
  extra_data: null
    ";

    #[tokio::test]
    async fn compact_attribute() -> Result<(), Box<dyn std::error::Error>> {
        crate::setup();

        let cluster_config = MinervaClusterConfig {
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let test_database = cluster.create_db().await?;

        info!("Created database '{}'", test_database.name);

        let mut client = test_database.connect().await?;
        create_schema(&mut client).await?;

        let attribute_store: AttributeStore = serde_yaml::from_str(ATTRIBUTE_STORE_DEFINITION)
            .map_err(|e| format!("Could not read trend store definition: {e}"))?;

        let add_attribute_store = AddAttributeStore { attribute_store };

        add_attribute_store.apply(&mut client).await?;

        let batch_insert_query = r#"
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (1, '2024-11-01 02:00:00Z', '2024-11-01 02:33:03Z', '2024-11-01 02:33:03Z', 'n20003', 'HAL905', 'AE40030-334315');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (1, '2024-11-02 02:00:00Z', '2024-11-02 02:31:26Z', '2024-11-02 02:31:26Z', 'n20003', 'HAL905', 'AE40030-334315');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (1, '2024-11-03 02:00:00Z', '2024-11-03 02:32:17Z', '2024-11-03 02:32:17Z', 'n20003', 'HAL905', 'AE40030-334315');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (1, '2024-11-04 02:00:00Z', '2024-11-04 02:32:02Z', '2024-11-04 02:32:02Z', 'n20003b', 'HAL905', 'AE40030-334315');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (1, '2024-11-05 02:00:00Z', '2024-11-05 02:30:57Z', '2024-11-05 02:30:57Z', 'n20003', 'HAL905', 'AE40030-334315');

INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (3, '2024-11-01 02:00:00Z', '2024-11-01 02:33:09Z', '2024-11-01 02:33:09Z', 'n20004', 'HAL905', 'AE40030-334319');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (3, '2024-11-02 02:00:00Z', '2024-11-02 02:31:33Z', '2024-11-02 02:31:33Z', 'n20004', 'HAL905', 'AE40030-334319');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (3, '2024-11-03 02:00:00Z', '2024-11-03 02:32:21Z', '2024-11-03 02:32:21Z', 'n20004', 'HAL905', 'AE40030-334319');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (3, '2024-11-04 02:00:00Z', '2024-11-04 02:32:19Z', '2024-11-04 02:32:19Z', 'n20004', 'HAL905', 'AE40030-334319');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (3, '2024-11-05 02:00:00Z', '2024-11-05 02:31:12Z', '2024-11-05 02:31:12Z', 'n20004', 'HAL905', 'AE40030-334319');

INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (4, '2024-11-01 02:00:00Z', '2024-11-01 02:31:11Z', '2024-11-01 02:31:11Z', 'n20005', 'HAL905', 'AE40030-33434');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (4, '2024-11-02 02:00:00Z', '2024-11-02 02:35:58Z', '2024-11-02 02:35:58Z', 'n20005', 'HAL905', 'AE40030-33434');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (4, '2024-11-03 02:00:00Z', '2024-11-03 02:32:01Z', '2024-11-03 02:32:01Z', 'n20005', 'HAL905', 'AE40030-33434');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (4, '2024-11-04 02:00:00Z', '2024-11-04 02:32:01Z', '2024-11-04 02:32:01Z', 'n20005', 'HAL905', 'AE40030-33434');
INSERT INTO attribute_history."hub_node"(entity_id, timestamp, first_appearance, modified, name, equipment_type, equipment_serial) VALUES (4, '2024-11-05 02:00:00Z', '2024-11-05 02:33:02Z', '2024-11-05 02:33:02Z', 'n20005', 'HAL905', 'AE40030-334344');
"#;

        client.batch_execute(batch_insert_query).await?;

        let exit_code = run_compact_cmd(&cluster, &test_database.name).await?;
        assert_eq!(exit_code.code(), Some(0));

        let row = client
            .query_one("SELECT count(*) FROM attribute_history.hub_node", &[])
            .await?;

        let row_count: i64 = row.get(0);

        assert_eq!(row_count, 6);

        //------------
        // Entity ID 1
        // For the entity with Id 1, there should remain 3 records because there is a change and a change back

        let entity_id = 1;

        let rows = client
            .query(
                "SELECT timestamp, first_appearance, modified FROM attribute_history.hub_node WHERE entity_id = $1 ORDER BY timestamp ASC",
                &[&entity_id],
            )
            .await?;

        assert_eq!(rows.len(), 3);

        let expected_rows = [
            (
                "2024-11-01T02:00:00Z",
                "2024-11-01T02:33:03Z",
                "2024-11-03T02:32:17Z",
            ),
            (
                "2024-11-04T02:00:00Z",
                "2024-11-04T02:32:02Z",
                "2024-11-04T02:32:02Z",
            ),
            (
                "2024-11-05T02:00:00Z",
                "2024-11-05T02:30:57Z",
                "2024-11-05T02:30:57Z",
            ),
        ];

        for (row, expected_row) in zip(rows, expected_rows) {
            let timestamp: DateTime<Utc> = row.get(0);
            let first_appearance: DateTime<Utc> = row.get(1);
            let modified: DateTime<Utc> = row.get(2);

            assert_eq!(timestamp, expected_row.0.parse::<DateTime<Utc>>().unwrap());
            assert_eq!(
                first_appearance,
                expected_row.1.parse::<DateTime<Utc>>().unwrap()
            );
            assert_eq!(modified, expected_row.2.parse::<DateTime<Utc>>().unwrap());
        }

        //------------
        // Entity ID 3
        // For the entity with Id 3, there should remain 1 record because there were no changes

        let entity_id = 3;

        let rows = client
            .query(
                "SELECT timestamp, first_appearance, modified FROM attribute_history.hub_node WHERE entity_id = $1 ORDER BY timestamp ASC",
                &[&entity_id],
            )
            .await?;

        assert_eq!(rows.len(), 1);

        let expected_rows = [(
            "2024-11-01T02:00:00Z",
            "2024-11-01T02:33:09Z",
            "2024-11-05T02:31:12Z",
        )];

        for (row, expected_row) in zip(rows, expected_rows) {
            let timestamp: DateTime<Utc> = row.get(0);
            let first_appearance: DateTime<Utc> = row.get(1);
            let modified: DateTime<Utc> = row.get(2);

            assert_eq!(timestamp, expected_row.0.parse::<DateTime<Utc>>().unwrap());
            assert_eq!(
                first_appearance,
                expected_row.1.parse::<DateTime<Utc>>().unwrap()
            );
            assert_eq!(modified, expected_row.2.parse::<DateTime<Utc>>().unwrap());
        }

        //------------
        // Entity ID 4
        // For the entity with Id 4, there should remain 2 records because there is 1 change

        let entity_id = 4;

        let rows = client
            .query(
                "SELECT timestamp, first_appearance, modified FROM attribute_history.hub_node WHERE entity_id = $1 ORDER BY timestamp ASC",
                &[&entity_id],
            )
            .await?;

        assert_eq!(rows.len(), 2);

        let expected_rows = [
            (
                "2024-11-01T02:00:00Z",
                "2024-11-01T02:31:11Z",
                "2024-11-04T02:32:01Z",
            ),
            (
                "2024-11-05T02:00:00Z",
                "2024-11-05T02:33:02Z",
                "2024-11-05T02:33:02Z",
            ),
        ];

        for (row, expected_row) in zip(rows, expected_rows) {
            let timestamp: DateTime<Utc> = row.get(0);
            let first_appearance: DateTime<Utc> = row.get(1);
            let modified: DateTime<Utc> = row.get(2);

            assert_eq!(timestamp, expected_row.0.parse::<DateTime<Utc>>().unwrap());
            assert_eq!(
                first_appearance,
                expected_row.1.parse::<DateTime<Utc>>().unwrap()
            );
            assert_eq!(modified, expected_row.2.parse::<DateTime<Utc>>().unwrap());
        }

        Ok(())
    }

    async fn run_compact_cmd(
        cluster: &MinervaCluster,
        database_name: &str,
    ) -> io::Result<ExitStatus> {
        let log_level = std::env::var("RUST_LOG").unwrap_or("error".to_string());

        let executable_path = cargo_bin("minerva");

        let mut cmd = Command::new(executable_path)
            .stdout(std::process::Stdio::piped())
            .env("RUST_LOG", log_level)
            .env("PGUSER", "postgres")
            .env("PGHOST", cluster.controller_host.to_string())
            .env("PGPORT", cluster.controller_port.to_string())
            .env("PGSSLMODE", "disable")
            .env("PGDATABASE", database_name)
            .arg("attribute-store")
            .arg("compact")
            .arg("--name")
            .arg("hub_node")
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to execute process");

        let stdout = cmd.stdout.take().expect("could not take stdout");

        let mut buf_reader = BufReader::new(stdout).lines();

        let _output_check_handle = tokio::spawn(async move {
            while let Some(line) = buf_reader.next_line().await.unwrap() {
                info!("CMD: {}", line);
            }
        });

        cmd.wait().await
    }
}
