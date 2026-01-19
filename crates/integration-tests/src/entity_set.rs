use std::net::Ipv4Addr;

use log::debug;
use serde_json::{json, Value};

use tokio::time::Duration;

use minerva::change::Change;
use minerva::changes::trend_store::AddTrendStore;
use minerva::cluster::MinervaClusterConnector;
use minerva::trend_store::TrendStore;

use crate::common::{
    create_schema_with_retry, create_webservice_role, get_available_port, MinervaService,
    MinervaServiceConfig,
};

/// Test the listing and creation of new entity sets
pub async fn get_and_create_entity_sets(
    cluster: MinervaClusterConnector,
) -> Result<(), Box<dyn std::error::Error>> {
    let webservice_role = "webservice";
    let test_database = cluster.create_db().await?;

    {
        let mut client = test_database.connect().await?;
        create_schema_with_retry(&mut client, 5).await?;

        let trend_store: TrendStore = TrendStore {
            title: None,
            data_source: "integration_test".to_string(),
            entity_type: "pvpanel".to_string(),
            granularity: Duration::from_secs(300),
            partition_size: Duration::from_secs(86400),
            retention_period: humantime::parse_duration("1y").unwrap(),
            parts: [].to_vec(),
        };

        let add_trend_store = AddTrendStore { trend_store };

        // Using this as a hack to make sure the entity type is created
        add_trend_store.apply(&mut client).await?;

        let entities = vec!["panel_01".to_string(), "panel_02".to_string()];

        client
            .execute(
                "INSERT INTO entity.pvpanel(name) SELECT unnest($1::text[])",
                &[&entities],
            )
            .await?;

        create_webservice_role(&cluster, webservice_role).await?;
    }

    {
        let service_address = Ipv4Addr::new(127, 0, 0, 1);
        let service_port = get_available_port(service_address).unwrap();

        let service_conf = MinervaServiceConfig {
            pg_host: cluster.coordinator_connector.host.to_string(),
            pg_port: cluster.coordinator_connector.port.to_string(),
            pg_sslmode: "disable".to_string(),
            pg_database: test_database.name.to_string(),
            pg_user: webservice_role.to_string(),
            service_address: service_address.to_string(),
            service_port,
        };

        let mut service = MinervaService::start(service_conf)?;

        debug!("Started service");

        service.wait_for().await?;

        let http_client = reqwest::Client::new();
        let url = format!("{}/entitysets", service.base_url());
        let response = http_client.get(url.clone()).send().await?;
        let body = response.text().await?;

        assert_eq!(body, "[]");

        let create_entity_set_data = json!({
            "name": "TinySet",
            "owner": "John Doe",
            "entities": ["panel_01", "panel_02"],
            "entity_type": "pvpanel",
        });

        let response = http_client
            .post(url.clone())
            .json(&create_entity_set_data)
            .send()
            .await?;

        assert_eq!(
            response.status(),
            200,
            "error response: {}",
            response.text().await.unwrap()
        );

        let body: Value = response.json().await?;

        assert_eq!(
            body,
            json!({
                "code": 200,
                "message": "Entity set created",
                "id": 1,
            })
        );
    }

    //let mut admin_client = cluster.connect_to_coordinator().await;

    //test_database.drop_database(&mut admin_client).await;

    Ok(())
}
