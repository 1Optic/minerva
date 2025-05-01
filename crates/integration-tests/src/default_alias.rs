#[cfg(test)]
mod tests {
    use log::debug;
    use std::path::PathBuf;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use rust_decimal::prelude::FromPrimitive;
    use tokio_postgres::binary_copy::BinaryCopyInWriter;
    use tokio_postgres::types::ToSql;
    use tokio_postgres::GenericClient;

    use minerva::change::Change;
    use minerva::changes::trend_store::AddTrendStore;
    use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
    use minerva::entity::{AddEntityType, EntityType};
    use minerva::meas_value::{DataType, MeasValue};
    use minerva::schema::create_schema;
    use minerva::trend_store::MeasurementStore;
    use minerva::trend_store::{create_partitions_for_timestamp, DataPackage, TrendStore};

    const ENTITY_TYPE_DEFINITION: &str = r"
        name: Site
        primary_alias: substring(name from '.*=(\d+)$')
        ";

    const TREND_STORE_DEFINITION: &str = r"
        title: Sample trend store
        data_source: minerva
        entity_type: Site
        granularity: 15m
        partition_size: 1d
        retention_period: 1y
        parts:
          - name: sample_trend_store_part
            primary_alias: true
            trends:
              - name: value
                data_type: numeric
          - name: sample_trend_store_part_2
            primary_alias: false
            trends:
              - name: value_2
                data_type: numeric
        ";

    pub struct RefinedDataPackage {
        timestamp: DateTime<Utc>,
        trends: Vec<String>,
        entity_ids: Vec<i32>,
        job_id: i64,
        rows: Vec<Vec<MeasValue>>,
    }

    #[async_trait]
    impl DataPackage for RefinedDataPackage {
        fn timestamp(&self) -> &DateTime<Utc> {
            &self.timestamp
        }

        fn trends(&self) -> &Vec<String> {
            &self.trends
        }

        async fn write(
            &self,
            mut writer: std::pin::Pin<&mut BinaryCopyInWriter>,
            values: &[(usize, DataType)],
            created_timestamp: &DateTime<chrono::Utc>,
        ) -> Result<usize, minerva::error::Error> {
            for (index, entity_id) in self.entity_ids.iter().enumerate() {
                let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                    vec![entity_id, &self.timestamp, created_timestamp, &self.job_id];

                let row = self.rows.get(index).unwrap();

                for (column_index, _data_type) in values {
                    let v = row.get(*column_index).unwrap();
                    sql_values.push(v);
                }

                writer.as_mut().write(&sql_values).await?;
            }

            Ok(self.entity_ids.len())
        }

        async fn insert<C: GenericClient + std::marker::Sync + std::marker::Send>(
            &self,
            client: &mut C,
            query: &str,
            values: &[(usize, DataType)],
            created_timestamp: &DateTime<chrono::Utc>,
        ) -> Result<usize, minerva::error::Error> {
            let mut count: usize = 0;

            for (row_index, entity_id) in self.entity_ids.iter().enumerate() {
                let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                    vec![entity_id, &self.timestamp, &created_timestamp, &self.job_id];

                let row = self.rows.get(row_index).unwrap();

                for (index, _data_type) in values {
                    let v = row.get(*index).unwrap();
                    sql_values.push(v);
                }

                client.execute(query, &sql_values).await?;
                count += 1;
            }

            Ok(count)
        }
    }

    // This is a temporary form that does not use yaml, so the test can be done to the
    // database code before the Yaml definitions are included. Once the Yaml definitions
    // are included, this one can be removed as it is then implied by the next test.
    #[tokio::test]
    async fn default_alias_database_temporary() -> Result<(), Box<dyn std::error::Error>> {
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

            client.execute(
                r"SELECT directory.create_entity_type('Site', 'substring(name from ''.*=(\d+)$''')",
                &[],
            ).await?;

            debug!("Created entity type");

            client
                .execute("SELECT entity.\"create_Site\"('name=Site20,number=100')", &[])
                .await?;

            let row = client
                .query_one(
                    "SELECT primary_alias FROM entity.\"Site\" WHERE name = 'name=Site20,number=100'",
                    &[],
                )
                .await?;

            let alias: String = row.get(0);

            assert_eq!(alias, "100".to_string());

            let query = concat!(
                "SELECT trend_directory.define_trend_store(",
                "directory.name_to_data_source('minerva'), ",
                "directory.name_to_entity_set('Site'), ",
                "'15m'::interval, '1d'::interval",
                ") RETURNING id",
            );

            let trend_store_row = client.query_one(query, &[]).await?;

            let trend_store_id = trend_store_row.get::<usize, i32>(0);

            let query = concat!(
                "SELECT trend_directory.assure_table_trends_exist(",
                "$1, 'sample_trend_store_part',",
                "ARRAY[('value', 'integer', '', 'SUM', 'SUM', '{}'::jsonb)]::trend_directory.trend_descr[],",
                "ARRAY[]::trend_directory.generated_trend_descr[]",
                ")",
            );

            client.execute(query, &[&trend_store_id]).await?;

            let query = concat!(
                "SELECT trend_directory.assure_table_trends_exist(",
                "$1, 'sample_trend_store_part_2',",
                "ARRAY[('value', 'integer', '', 'SUM', 'SUM', '{}'::jsonb)]::trend_directory.trend_descr[],",
                "ARRAY[]::trend_directory.generated_trend_descr[]",
                ")",
            );

            client.execute(query, &[&trend_store_id]).await?;

            let query = concat!(
                "UPDATE trend_directory.trend_store_part ",
                "SET primary_alias = true ",
                "WHERE name = 'sample_trend_store_part'",
            );

            client.execute(query, &[]).await?;

            let query = concat!(
                "SELECT column_name FROM information_schema.columns ",
                "WHERE table_schema = 'trend' AND table_name = 'sample_trend_store_part'",
            );

            let rows = client.query(query, &[]).await?;
            let columns: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            assert!(
                columns.contains(&"alias".to_string()),
                "alias column not created"
            );

            client.execute(query, &[]).await?;

            let query = concat!(
                "SELECT column_name FROM information_schema.columns ",
                "WHERE table_schema = 'trend' AND table_name = 'sample_trend_store_part_2'",
            );

            let rows = client.query(query, &[]).await?;
            let columns: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            assert!(
                !columns.contains(&"alias".to_string()),
                "alias column created where it should not"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn default_alias_database() -> Result<(), Box<dyn std::error::Error>> {
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

            let entity_type: EntityType = serde_yaml::from_str(ENTITY_TYPE_DEFINITION)
                .map_err(|e| format!("Could not read entity type definition: {e}"))?;

            let add_entity_type = AddEntityType {
                entity_type: entity_type.clone(),
            };

            add_entity_type.apply(&mut client).await?;

            debug!("Created entity type");

            client
                .execute("SELECT entity.\"create_Site\"('name=Site20,number=100')", &[])
                .await?;

            let row = client
                .query_one(
                    "SELECT primary_alias FROM entity.\"Site\" WHERE name = 'name=Site20,number=100'",
                    &[],
                )
                .await?;

            let alias: String = row.get(0);

            assert_eq!(alias, "100".to_string());

            let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
                .map_err(|e| format!("Could not read trend store definition: {e}"))?;

            let add_trend_store = AddTrendStore {
                trend_store: trend_store.clone(),
            };

            add_trend_store.apply(&mut client).await?;

            debug!("Created trend store");

            let query = concat!(
                "SELECT column_name FROM information_schema.columns ",
                "WHERE table_schema = 'trend' AND table_name = 'sample_trend_store_part'",
            );

            let rows = client.query(query, &[]).await?;
            let columns: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            assert!(
                columns.contains(&"alias".to_string()),
                "alias column not created"
            );

            client.execute(query, &[]).await?;

            let query = concat!(
                "SELECT column_name FROM information_schema.columns ",
                "WHERE table_schema = 'trend' AND table_name = 'sample_trend_store_part_2'",
            );

            let rows = client.query(query, &[]).await?;
            let columns: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            assert!(
                !columns.contains(&"alias".to_string()),
                "alias column created where it should not"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn default_alias_insert() -> Result<(), Box<dyn std::error::Error>> {
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

            let entity_type: EntityType = serde_yaml::from_str(ENTITY_TYPE_DEFINITION)
                .map_err(|e| format!("Could not read entity type definition: {e}"))?;

            let add_entity_type = AddEntityType {
                entity_type: entity_type.clone(),
            };

            add_entity_type.apply(&mut client).await?;

            debug!("Created entity type");

            let trend_store: TrendStore = serde_yaml::from_str(TREND_STORE_DEFINITION)
                .map_err(|e| format!("Could not read trend store definition: {e}"))?;

            let add_trend_store = AddTrendStore {
                trend_store: trend_store.clone(),
            };

            add_trend_store.apply(&mut client).await?;

            debug!("Created trend store");

            let timestamp = chrono::DateTime::parse_from_rfc3339("2025-03-25T14:00:00+00:00")
                .unwrap()
                .to_utc();

            create_partitions_for_timestamp(&mut client, timestamp).await?;

            let job_id = 10;

            let trends = vec!["value".to_string()];

            let trend_store_part = trend_store
                .parts
                .iter()
                .find(|p| p.name == "sample_trend_store_part")
                .unwrap();

            client
                .execute("SELECT entity.\"create_Site\"('name=Site20,number=100')", &[])
                .await?;

            let names = vec![
                "name=Site20,number=100",
                "name=Site20,number=101",
                "name=Site20,number=102",
                "name=Site20,number=103",
                "name=Site20,number=104",
                "name=Site20,number=105",
                "name=Site20,number=106",
                "name=Site20,number=107",
                "name=Site20,number=108",
                "name=Site20,number=109",
            ];

            let targets = vec![
                "100", "101", "102", "103", "104", "105", "106", "107", "108", "109",
            ];

            let mut entity_ids: Vec<i32> = vec![];
            let query = "SELECT entity.\"create_Site\"($1)";

            for target in names.iter() {
                entity_ids.push(client.query_one(query, &[target]).await?.get(0));
            }

            let rows = vec![vec![MeasValue::Numeric(Some(
                rust_decimal::Decimal::from_f64(20.0).unwrap(),
            ))]];

            let package = RefinedDataPackage {
                timestamp,
                trends,
                entity_ids,
                job_id,
                rows,
            };

            trend_store_part
                .store_package(&mut client, &package)
                .await?;

            let rows = client
                .query("SELECT alias FROM trend.sample_trend_store_part", &[])
                .await?;
            let aliases: Vec<&str> = rows.iter().map(|row| row.get::<usize, &str>(0)).collect();

            for target in targets.iter() {
                assert!(
                    aliases.contains(target),
                    "No trend found for alias {}",
                    target
                );
            }
        }
        Ok(())
    }
}
