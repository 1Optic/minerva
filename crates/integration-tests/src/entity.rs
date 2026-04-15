use log::debug;

use minerva::cluster::MinervaClusterConnector;
use minerva::entity::{DbEntityMapping, EntityMapping};
use minerva::schema::create_schema;

pub async fn db_entity_mapping(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let entity_mapping = DbEntityMapping {};

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
            .names_to_entity_ids(&tx, &"node".to_string(), &names)
            .await
            .unwrap();

        tx.commit().await?;

        (start.elapsed(), entity_ids.len())
    };

    println!("Duration: {elapsed:?}");

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

        // The following is the test for the bug in KPN1OPTIC2026-158

        let db_ids = rows
            .iter()
            .map(|row| row.get::<_, i32>(0))
            .collect::<Vec<_>>();

        let names = vec![
            "n0008".to_string(),
            "n0004".to_string(),
            "n0003".to_string(),
            "n0001".to_string(),
        ];

        let entity_mapping = DbEntityMapping {};
        let entities = entity_mapping
            .names_to_entities(&client, &"node".to_string(), &names)
            .await?;

        let request_ids = entities
            .iter()
            .map(|entity| entity.id as i32)
            .collect::<Vec<_>>();
        assert_eq!(request_ids, db_ids);
    }

    Ok(())
}
