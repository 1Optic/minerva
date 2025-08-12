use refinery::embed_migrations;
use thiserror::Error;
use tokio_postgres::Client;

embed_migrations!("migrations");

const SCHEMA_HISTORY_TABLE: &str = "schema_history";

#[derive(Error, Debug)]
pub enum SchemaCreationError {
    #[error("{0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("{0}")]
    Refinery(#[from] refinery::Error),
    #[error("Tuple concurrently updated")]
    TupleConcurrentlyUpdated,
}

pub async fn create_schema(client: &mut Client) -> Result<(), SchemaCreationError> {
    client
        .execute("SET citus.multi_shard_modify_mode TO 'sequential'", &[])
        .await?;

    migrations::runner()
        .set_migration_table_name(SCHEMA_HISTORY_TABLE)
        .run_async(client)
        .await
        .map_err(|e| match e.kind() {
            refinery::error::Kind::Connection(_conn_err_msg, conn_err) => {
                if let Some(source) = conn_err.source() {
                    if source.to_string().contains("tuple concurrently updated") {
                        return SchemaCreationError::TupleConcurrentlyUpdated;
                    }
                }

                SchemaCreationError::from(e)
            }
            _ => SchemaCreationError::from(e),
        })?;

    Ok(())
}

/// Get version number of last applied migration in the database.
pub async fn get_current_version(client: &mut Client) -> Result<Option<u32>, String> {
    let migration = migrations::runner()
        .set_migration_table_name(SCHEMA_HISTORY_TABLE)
        .get_last_applied_migration_async(client)
        .await
        .map_err(|e| format!("Could not get last migration information: {e}"))?;

    Ok(migration.map(|m| m.version()))
}

/// Get list of migrations that have a greater version number than the last applied migration
/// in the database.
pub async fn get_pending_migrations(client: &mut Client) -> Result<Vec<(u32, String)>, String> {
    let mut runner = migrations::runner();
    runner.set_migration_table_name(SCHEMA_HISTORY_TABLE);

    let migrations = runner.get_migrations();

    let last_applied_migration = runner
        .get_last_applied_migration_async(client)
        .await
        .unwrap()
        .map(|m| m.version());

    let mut pending_migrations = migrations
        .iter()
        .filter(|m| last_applied_migration.is_none_or(|applied| m.version() > applied))
        .map(|m| (m.version(), m.name().to_string()))
        .collect::<Vec<_>>();

    // List of migrations is not sorted by default, so sort by version number
    pending_migrations.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(pending_migrations)
}

pub async fn migrate(client: &mut Client) -> Result<(), String> {
    let report = migrations::runner()
        .set_migration_table_name(SCHEMA_HISTORY_TABLE)
        .run_async(client)
        .await
        .map_err(|e| format!("Could not migrate database schema: {e}"))?;

    let migrations = report.applied_migrations();

    if migrations.is_empty() {
        println!("Already up-to-date");
    } else {
        for m in migrations {
            println!("Applied: {m}");
        }
    }

    Ok(())
}
