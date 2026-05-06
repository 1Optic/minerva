use assert_cmd::assert;
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

pub async fn avoid_deadlock(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {

    let test_database = cluster.create_db().await?;

    debug!("Created database '{}'", test_database.name);

    let mut client = test_database.connect().await?;
    let mut client1 = test_database.connect().await?;
    let mut client2 = test_database.connect().await?;
    create_schema(&mut client).await?;
    
    let txpre = client.transaction().await?;

    let entity_type_name = "node";

    txpre.query(
        "SELECT directory.create_entity_type($1)",
        &[&entity_type_name],
    )
    .await
    .unwrap();
    let names = vec![
        "n0001".to_string(),
        "n0002".to_string(),
        "n0003".to_string(),
        "n0004".to_string(),
    ];

    let entity_mapping = DbEntityMapping {};
    let _ = entity_mapping
        .names_to_entity_ids(&txpre, &"node".to_string(), &names)
        .await
        .unwrap()
        .clone();
    txpre.commit().await?;

    let txwait = client.transaction().await?;

    txwait.commit().await?;

    let pid_query = "SELECT pg_backend_pid()";
    let mypid = client.query_one(pid_query, &[]).await?.get::<_, i32>(0);
    let (sender1, receiver1) = tokio::sync::oneshot::channel();
    let (sender2, receiver2) = tokio::sync::oneshot::channel();

    let mut client = test_database.connect().await?;

    let txselect = client.transaction().await?;
    let query = "SELECT * FROM entity.node";

    txselect.query(query, &[]).await?;

    let handle1 = tokio::spawn(async move {
        let tx1 = client1.transaction().await.unwrap();
        let mypid = tx1.query_one(pid_query, &[]).await.unwrap().get::<_, i32>(0);
        sender1.send(mypid).unwrap();

        let names = vec![
            "n0001".to_string(),
            "n0002".to_string(),
            "n0005".to_string(),
            "n0006".to_string(),
        ];

        let entity_mapping = DbEntityMapping {};
        let entities = entity_mapping
            .names_to_entity_ids(&tx1, &"node".to_string(), &names)
            .await
            .unwrap()
            .clone();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        tx1.commit().await.unwrap();

        assert_eq!(entities.len(), 4, "Expected to retrieve 4 entities in transaction 1, but got {}", entities.len());
        entities
    });

    let handle2 = tokio::spawn(async move {
        let tx2 = client2.transaction().await.unwrap();
        let mypid = tx2.query_one(pid_query, &[]).await.unwrap().get::<_, i32>(0);
        sender2.send(mypid).unwrap();
       

        let names = vec![
            "n0001".to_string(),
            "n0003".to_string(),
            "n0005".to_string(),
            "n0007".to_string(),
        ];

        let entity_mapping = DbEntityMapping {};
        let entities = entity_mapping
            .names_to_entity_ids(&tx2, &"node".to_string(), &names)
            .await
            .unwrap()
            .clone();

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        tx2.commit().await.unwrap();

        assert_eq!(entities.len(), 4, "Expected to retrieve 4 entities in transaction 2, but got {}", entities.len());
        entities
    });

    let observer_client = test_database.connect().await?;

    let pid1 = receiver1.await.unwrap();
    let pid2 = receiver2.await.unwrap();
    println!("My PID: {}, PID1: {}, PID2: {}", mypid, pid1, pid2);
    let query = concat!(
        "select l1.pid, l2.pid, l1.relation::regclass::text, l2.relation::regclass::text, l1.mode, l2.mode, a1.query as query1, a2.query as query2 ",
        "from pg_locks l1 join pg_locks l2 on l1.relation = l2.relation ",
        "join pg_stat_activity a1 on l1.pid = a1.pid join pg_stat_activity a2 on l2.pid = a2.pid ",
        "where l1.pid = $1 OR l2.pid = $1 OR l1.pid = $2 OR l2.pid = $2 OR l1.pid = $3 OR l2.pid = $3;"
    );
    let rows = txselect.query(query, &[&mypid, &pid1, &pid2]).await?;
    for row in rows {
        println!("Pid 1: {}, Pid 2: {},Lock 1: {}, Lock 2: {}, Mode 1: {}, Mode 2: {}, Query 1: {}, Query 2: {}",
            row.get::<_, i32>(0),
            row.get::<_, i32>(1),
            row.get::<_, String>(2),
            row.get::<_, String>(3),
            row.get::<_, String>(4),
            row.get::<_, String>(5),
            row.get::<_, String>(6),
            row.get::<_, String>(7),
        );
    }

    let entities1 = tokio::select! {
        result = handle1 => {
            // The task completed successfully, which means there was no deadlock
            result?
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            // The task is still running after 20 seconds, which likely indicates a deadlock
            return Err("Deadlock for handle1".into());
        }
    };

    let entities2 = tokio::select! {
        result = handle2 => {
            // The task completed successfully, which means there was no deadlock
            result?
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
            // The task is still running after 20 seconds, which likely indicates a deadlock
            return Err("Deadlock for handle2".into());
        }
    };
    
    txselect.commit().await?;

    let mut client = test_database.connect().await?;
    let tx = client.transaction().await?;
    let query = "SELECT id FROM entity.node";
    let rows = tx.query(query, &[]).await?;

    let entities = rows.iter().map(|row| row.get::<_, i32>(0)).collect::<Vec<_>>();
    assert_eq!(entities.len(), 7, "Expected 7 entities to be inserted, but found {}", entities.len());
    for entity in entities1 {
        assert!(entities.contains(&(entity as i32)), "Entity ID {} from first transaction not found in database", entity);
    }
    for entity in entities2 {
        assert!(entities.contains(&(entity as i32)), "Entity ID {} from second transaction not found in database", entity);
    }   
    tx.commit().await?;

    Ok(())
}