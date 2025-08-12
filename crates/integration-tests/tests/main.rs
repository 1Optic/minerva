use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use libtest_mimic::{Arguments, Failed, Trial};

use minerva::cluster::{MinervaCluster, MinervaClusterConfig, MinervaClusterConnector};

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args = Arguments::from_args();

    if !args.list {
        env_logger::init();

        let cluster_config = MinervaClusterConfig {
            config_file: PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "postgresql.conf"]),
            ..Default::default()
        };

        let cluster = MinervaCluster::start(&cluster_config).await?;

        let tests = setup_tests(Some(cluster.connector.clone()));

        Ok(libtest_mimic::run(&args, tests).exit_code())
    } else {
        let tests = setup_tests(None);

        Ok(libtest_mimic::run(&args, tests).exit_code())
    }
}

fn setup_test<F>(
    connector: Option<MinervaClusterConnector>,
    test_fn: fn(MinervaClusterConnector) -> F,
) -> Box<dyn FnOnce() -> Result<(), Failed> + Send>
where
    F: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + 'static,
{
    match connector {
        Some(c) => Box::new(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            Ok(runtime.block_on(test_fn(c))?)
        }),
        None => Box::new(move || Ok(())),
    }
}

fn setup_test_without_connector<F>(
    test_fn: fn() -> F,
) -> Box<dyn FnOnce() -> Result<(), Failed> + Send>
where
    F: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + 'static,
{
    Box::new(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Ok(runtime.block_on(test_fn())?)
    })
}

fn setup_tests(connector: Option<MinervaClusterConnector>) -> Vec<Trial> {
    vec![
        Trial::test(
            "compact_attribute",
            setup_test(
                connector.clone(),
                integration_tests::compact_attribute::compact_attribute,
            ),
        ),
        Trial::test(
            "create_kpi",
            setup_test(connector.clone(), integration_tests::create_kpi::create_kpi),
        ),
        Trial::test(
            "create_trigger",
            setup_test(
                connector.clone(),
                integration_tests::create_trigger::create_trigger,
            ),
        ),
        Trial::test(
            "default_alias_database",
            setup_test(
                connector.clone(),
                integration_tests::default_alias::default_alias_database,
            ),
        ),
        Trial::test(
            "default_alias_insert",
            setup_test(
                connector.clone(),
                integration_tests::default_alias::default_alias_insert,
            ),
        ),
        Trial::test(
            "default_alias_insert_delayed",
            setup_test(
                connector.clone(),
                integration_tests::default_alias::default_alias_insert_delayed,
            ),
        ),
        Trial::test(
            "get_and_create_entity_sets",
            setup_test(
                connector.clone(),
                integration_tests::entity_set::get_and_create_entity_sets,
            ),
        ),
        Trial::test(
            "get_entity_types",
            setup_test(
                connector.clone(),
                integration_tests::get_entity_types::get_entity_types,
            ),
        ),
        Trial::test(
            "initialize",
            setup_test(connector.clone(), integration_tests::initialize::initialize),
        ),
        Trial::test(
            "load_data",
            setup_test(connector.clone(), integration_tests::load_data::load_data),
        ),
        Trial::test(
            "load_data_twice",
            setup_test(
                connector.clone(),
                integration_tests::load_data::load_data_twice,
            ),
        ),
        Trial::test(
            "db_connection_instability",
            setup_test_without_connector(integration_tests::service::db_connection_instability),
        ),
        Trial::test(
            "materialize_service",
            setup_test(
                connector.clone(),
                integration_tests::trend_materialization::materialize_service,
            ),
        ),
        Trial::test(
            "trend_storage",
            setup_test(
                connector.clone(),
                integration_tests::trend_storage::store_package,
            ),
        ),
        Trial::test(
            "trend_value_information",
            setup_test(
                connector.clone(),
                integration_tests::trend_value_information::trend_value_information,
            ),
        ),
        Trial::test(
            "trigger_trigger_notifications",
            setup_test(
                connector.clone(),
                integration_tests::trigger_trigger::trigger_trigger_notifications,
            ),
        ),
        Trial::test(
            "load_attribute_data",
            setup_test(
                connector.clone(),
                integration_tests::attribute_storage::load_attribute_data,
            ),
        ),
        Trial::test(
            "db_entity_mapping",
            setup_test(
                connector.clone(),
                integration_tests::entity::db_entity_mapping,
            ),
        ),
    ]
}
