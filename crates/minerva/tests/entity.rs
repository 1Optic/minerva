mod common;

#[cfg(test)]
mod tests {
    use log::debug;
    use std::path::PathBuf;

    use minerva::cluster::{MinervaCluster, MinervaClusterConfig};
    use minerva::entity::{DbEntityMapping, EntityMapping};
    use minerva::schema::create_schema;

    #[tokio::test]
    async fn db_entity_mapping() -> Result<(), Box<dyn std::error::Error>> {
        crate::common::setup();

        let entity_mapping = DbEntityMapping {};

        let cluster_config = MinervaClusterConfig {
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let test_database = cluster.create_db().await?;

        debug!("Created database '{}'", test_database.name);

        let (elapsed, stored_count) = {
            let mut client = test_database.connect().await?;
            create_schema(&mut client).await?;

            let start = std::time::Instant::now();

            let tx = client.transaction().await?;

            let entity_type_name = "node";

            tx.query(
                "SELECT directory.create_entity_type($1)",
                &[&entity_type_name],
            )
            .await
            .unwrap();

            let names = vec![
                "n0001".to_string(),
                "n0004".to_string(),
                "n0008".to_string(),
                "n0003".to_string(),
            ];

            let entity_ids = entity_mapping
                .names_to_entity_ids(&tx, &"node".to_string(), names)
                .await
                .unwrap();

            tx.commit().await?;

            (start.elapsed(), entity_ids.len())
        };

        println!("Duration: {:?}", elapsed);

        assert_eq!(stored_count, 4);

        {
            let client = test_database.connect().await?;

            let rows = client
                .query("SELECT id, name FROM entity.node ORDER BY name DESC", &[])
                .await?;

            assert_eq!(rows.len(), 4);

            let first_row = rows.first().unwrap();

            let first_name: &str = first_row.get(1);

            assert_eq!(first_name, "n0008");
        }

        Ok(())
    }
}
