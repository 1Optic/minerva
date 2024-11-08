use std::io::Write;

use async_trait::async_trait;
use postgres_protocol::escape::escape_identifier;
use tokio_postgres::GenericClient;

use clap::{Parser, Subcommand};
use thiserror::Error;

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct VirtualEntityOpt {
    #[command(subcommand)]
    command: VirtualEntityOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum VirtualEntityOptCommands {
    #[command(about = "materialize a relation")]
    Materialize(VirtualEntityMaterialize),
}

impl VirtualEntityOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            VirtualEntityOptCommands::Materialize(materialize) => materialize.run().await,
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct VirtualEntityMaterialize {
    #[arg(help = "virtual entity type name")]
    name: String,
}

#[derive(Debug, Error)]
enum VirtualEntityUpdateError {
    #[error("Could not create temp table: {0}")]
    CreateTempTable(String),
    #[error("Could not load data into temp table: {0}")]
    LoadTempTable(String),
    #[error("Could not add entities from temp table: {0}")]
    AddEntities(String),
    #[error("Could not drop temp table: {0}")]
    DropTempTable(String),
}

async fn materialize_virtual_entity<C>(
    client: &mut C,
    name: &str,
) -> Result<u64, VirtualEntityUpdateError>
where
    C: GenericClient,
{
    let temp_table_name = "temp_entity";

    let create_temp_table_query = format!(
        "CREATE TEMPORARY TABLE {} (name text)",
        escape_identifier(temp_table_name),
    );

    client
        .execute(&create_temp_table_query, &[])
        .await
        .map_err(|e| VirtualEntityUpdateError::CreateTempTable(format!("{e}")))?;

    let insert_temp_query = format!(
        "INSERT INTO {}(name) SELECT name FROM virtual_entity.{}",
        escape_identifier(temp_table_name),
        escape_identifier(name),
    );

    print!("Loading into temporary table '{temp_table_name}' ... ");

    std::io::stdout().flush().unwrap();

    let insert_temp_count = client
        .execute(&insert_temp_query, &[])
        .await
        .map_err(|e| VirtualEntityUpdateError::LoadTempTable(format!("{e}")))?;

    println!("Done ({insert_temp_count} records)");

    let insert_query = format!(
        "INSERT INTO entity.{}(name) SELECT t.name FROM {} AS t ON CONFLICT DO NOTHING",
        escape_identifier(name),
        escape_identifier(temp_table_name),
    );

    print!("Loading into target table 'entity.{name}' ... ");

    std::io::stdout().flush().unwrap();

    let insert_count = client
        .execute(&insert_query, &[])
        .await
        .map_err(|e| VirtualEntityUpdateError::AddEntities(format!("{e}")))?;

    println!("Done ({insert_count} records)");

    let drop_temp_table_query = format!("DROP TABLE {}", escape_identifier(temp_table_name));

    client
        .execute(&drop_temp_table_query, &[])
        .await
        .map_err(|e| VirtualEntityUpdateError::DropTempTable(format!("{e}")))?;

    Ok(insert_count)
}

#[async_trait]
impl Cmd for VirtualEntityMaterialize {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let mut tx = client.transaction().await?;

        match materialize_virtual_entity(&mut tx, &self.name).await {
            Ok(record_count) => {
                tx.commit().await?;
                println!(
                    "Materialized virtual entity '{}' ({} records)",
                    self.name, record_count
                );
            }
            Err(e) => {
                tx.rollback().await?;
                println!("Error materializing relation '{}': {e}", self.name);
            }
        }

        Ok(())
    }
}
