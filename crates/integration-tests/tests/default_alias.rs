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
    use minerva::entity_type::{AddEntityType, EntityType};
    use minerva::meas_value::{DataType, MeasValue};
    use minerva::schema::create_schema;
    use minerva::trend_store::MeasurementStore;
    use minerva::trend_store::{
        create_partitions_for_timestamp, DataPackage, DataPackageWriteError, TrendStore,
    };

    use integration_tests::setup;

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
            has_alias_column: true
            trends:
              - name: value
                data_type: numeric
          - name: sample_trend_store_part_2
            has_alias_column: false
            trends:
              - name: value_2
                data_type: numeric
        ";

    pub struct RefinedDataPackage {
        timestamp: DateTime<Utc>,
        trends: Vec<String>,
        entity_ids: Vec<i32>,
        entity_names: Option<Vec<String>>,
        job_id: i64,
        rows: Vec<Vec<MeasValue>>,
    }

    impl RefinedDataPackage {
        fn listed_entity_names(&self) -> Vec<Option<String>> {
            match &self.entity_names {
                Some(list) => list.iter().map(|name| Some(name.to_string())).collect(),
                None => self.entity_ids.iter().map(|_| None).collect(),
            }
        }
    }

    #[async_trait]
    impl DataPackage for RefinedDataPackage {
        fn timestamp(&self) -> &DateTime<Utc> {
            &self.timestamp
        }

        fn trends(&self) -> Vec<String> {
            match self.entity_names {
                Some(_) => [vec!["name".to_string()], self.trends.clone()].concat(),
                None => self.trends.clone(),
            }
        }

        async fn write(
            &self,
            mut writer: std::pin::Pin<&mut BinaryCopyInWriter>,
            values: &[(usize, DataType)],
            created_timestamp: &DateTime<chrono::Utc>,
        ) -> Result<usize, DataPackageWriteError> {
            let entity_names = self.listed_entity_names().clone();
            for (index, entity_id) in self.entity_ids.iter().enumerate() {
                let entity_name = entity_names.get(index).ok_or_else(|| {
                    DataPackageWriteError::DataPreparation(format!(
                        "No entity name with index {index}"
                    ))
                })?;

                let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                    vec![entity_id, &self.timestamp, created_timestamp, &self.job_id];
                if let Some(name) = entity_name {
                    sql_values.push(name);
                }

                let mut row = self
                    .rows
                    .get(index)
                    .ok_or_else(|| {
                        DataPackageWriteError::DataPreparation(format!(
                            "No data row with index {index}"
                        ))
                    })?
                    .clone();

                if let Some(name) = entity_name {
                    row.insert(0, MeasValue::Text(name.to_string()))
                };

                for (column_index, _data_type) in values {
                    let v = row.get(*column_index).ok_or_else(|| {
                        DataPackageWriteError::DataPreparation(format!(
                            "No data column with index {column_index}"
                        ))
                    })?;
                    sql_values.push(v);
                }

                writer.as_mut().write(&sql_values).await.map_err(|e| {
                    let db_error = e.as_db_error();

                    match db_error {
                        Some(db_e) => DataPackageWriteError::Generic(format!("dbe: {db_e}")),
                        None => DataPackageWriteError::Generic(format!("{e}")),
                    }
                })?;
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
            let entity_names = self.listed_entity_names().clone();

            for (row_index, entity_id) in self.entity_ids.iter().enumerate() {
                let mut sql_values: Vec<&(dyn ToSql + Sync)> =
                    vec![entity_id, &self.timestamp, &created_timestamp, &self.job_id];

                if let Some(name) = entity_names.get(row_index).unwrap() {
                    sql_values.push(name);
                }

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

    #[tokio::test]
    async fn default_alias_database() -> Result<(), Box<dyn std::error::Error>> {
        setup();

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
                .execute(
                    "SELECT entity.\"create_Site\"('name=Site20,number=100')",
                    &[],
                )
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
                columns.contains(&"name".to_string()),
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
                !columns.contains(&"name".to_string()),
                "alias column created where it should not"
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn default_alias_insert() -> Result<(), Box<dyn std::error::Error>> {
        setup();

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

            debug!("Schema created");

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
                .execute(
                    "SELECT entity.\"create_Site\"('name=Site20,number=100')",
                    &[],
                )
                .await?;

            debug!("First site created");

            let names = [
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

            let targets = [
                "100", "101", "102", "103", "104", "105", "106", "107", "108", "109",
            ];

            let mut entity_ids: Vec<i32> = vec![];
            let mut entity_names: Vec<String> = vec![];
            let query = "SELECT id, primary_alias FROM entity.\"create_Site\"($1)";

            debug!("All sites created");

            for target in names.iter() {
                let result = client.query_one(query, &[target]).await?;
                entity_ids.push(result.get(0));
                entity_names.push(result.get(1));
            }

            let rows: Vec<Vec<MeasValue>> = vec![
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.0).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.1).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.2).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.3).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.4).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.5).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.6).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.7).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.8).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.9).unwrap(),
                ))],
            ];

            let package = RefinedDataPackage {
                timestamp,
                trends,
                entity_ids,
                entity_names: Some(entity_names),
                job_id,
                rows,
            };

            trend_store_part
                .store_package(&mut client, &package)
                .await?;

            debug!("Package stored");

            let rows = client
                .query("SELECT name FROM trend.sample_trend_store_part", &[])
                .await?;
            let aliases: Vec<&str> = rows.iter().map(|row| row.get::<usize, &str>(0)).collect();

            for target in targets.iter() {
                assert!(
                    aliases.contains(target),
                    "No trend found for alias {target}"
                );
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn default_alias_insert_delayed() -> Result<(), Box<dyn std::error::Error>> {
        setup();

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

            debug!("Schema created");

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

            let trends = vec!["value_2".to_string()];

            let trend_store_part = trend_store
                .parts
                .iter()
                .find(|p| p.name == "sample_trend_store_part_2")
                .unwrap();

            client
                .execute(
                    "SELECT entity.\"create_Site\"('name=Site20,number=100')",
                    &[],
                )
                .await?;

            debug!("First site created");

            let names = [
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

            let targets = [
                "100", "101", "102", "103", "104", "105", "106", "107", "108", "109",
            ];

            let mut entity_ids: Vec<i32> = vec![];
            let mut entity_names: Vec<String> = vec![];
            let query = "SELECT id, primary_alias FROM entity.\"create_Site\"($1)";

            debug!("All sites created");

            for target in names.iter() {
                let result = client.query_one(query, &[target]).await?;
                entity_ids.push(result.get(0));
                entity_names.push(result.get(1));
            }

            let rows: Vec<Vec<MeasValue>> = vec![
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.0).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.1).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.2).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.3).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.4).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.5).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.6).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.7).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.8).unwrap(),
                ))],
                vec![MeasValue::Numeric(Some(
                    rust_decimal::Decimal::from_f64(10.9).unwrap(),
                ))],
            ];

            let package = RefinedDataPackage {
                timestamp,
                trends,
                entity_ids,
                entity_names: None,
                job_id,
                rows,
            };

            trend_store_part
                .store_package(&mut client, &package)
                .await?;

            debug!("Package stored");

            let query = concat!(
                "SELECT trend_directory.ensure_name_column(tsp) ",
                "FROM trend_directory.trend_store_part tsp ",
                "WHERE tsp.name = 'sample_trend_store_part_2'",
            );
            client.execute(query, &[]).await?;

            debug!("Given table alias column");

            let rows = client
                .query("SELECT name FROM trend.sample_trend_store_part_2", &[])
                .await?;
            let aliases: Vec<&str> = rows.iter().map(|row| row.get::<usize, &str>(0)).collect();

            for target in targets.iter() {
                assert!(
                    aliases.contains(target),
                    "No trend found for alias {target} when alias column added later"
                );
            }
        }
        Ok(())
    }
}
