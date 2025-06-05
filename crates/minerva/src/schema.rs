use log::info;
use refinery::embed_migrations;
use tokio_postgres::Client;

embed_migrations!("migrations");

const SCHEMA_HISTORY_TABLE: &str = "schema_history";

pub async fn create_schema(client: &mut Client) -> Result<(), String> {
    migrations::runner()
        .set_migration_table_name(SCHEMA_HISTORY_TABLE)
        .run_async(client)
        .await
        .unwrap();

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
        info!("Already up-to-date");
    } else {
        for m in migrations {
            info!("Applied: {m}");
        }
    }

    Ok(())
}
