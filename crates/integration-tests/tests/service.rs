use postgres_protocol::escape::{escape_identifier, escape_literal};
use serde_json::json;

use integration_tests::common::{MinervaService, MinervaServiceConfig, TestStackConfig};

#[tokio::test]
async fn db_connection_instability() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let test_stack_config = TestStackConfig::default();

    let test_stack = test_stack_config.start().await?;

    let test_database = test_stack.create_db_with_definition().await?;

    let conn = test_database.connect().await?;

    let service_conf = MinervaServiceConfig::from_test_stack(&test_stack, &test_database.name);

    let statement_timeout = "3s";

    let create_role_query = format!(
        "CREATE ROLE {} WITH LOGIN IN ROLE minerva",
        escape_identifier(&service_conf.pg_user)
    );
    conn.execute(&create_role_query, &[]).await?;
    let alter_role_query = format!(
        "ALTER ROLE {} SET statement_timeout = {}",
        escape_identifier(&service_conf.pg_user),
        escape_literal(statement_timeout),
    );
    conn.execute(&alter_role_query, &[]).await?;

    let service = MinervaService::start(service_conf)?;

    let client = reqwest::Client::new();

    let url = format!("http://{}/triggers", service.conf.address());

    let response = client.get(url.clone()).send().await?;
    let response_data: serde_json::Value = response.json().await?;

    println!("Response: {response_data}");
    assert_eq!(response_data, json!([]));

    test_stack.db_conn_down().await?;

    let url = format!("http://{}/triggers", service.conf.address());
    let response = client.get(url.clone()).send().await?;
    let status = response.status();
    let response_data: serde_json::Value = response.json().await?;

    println!("Response: {} {response_data}", status);
    assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);

    test_stack.db_conn_up().await?;

    let url = format!("http://{}/triggers", service.conf.address());
    let response = client.get(url.clone()).send().await?;
    let status = response.status();
    let response_data: serde_json::Value = response.json().await?;

    println!("Response: {} {response_data}", status);
    assert_eq!(response_data, json!([]));

    Ok(())
}
