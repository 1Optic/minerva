use std::io::Write;

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
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub fn run(&self) -> CmdResult {
        match &self.command {
            VirtualEntityOptCommands::Materialize(materialize) => materialize.run(),
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct VirtualEntityMaterialize {
    #[arg(help = "virtual entity type name")]
    name: Vec<String>,
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

async fn get_virtual_entity_type_names<C>(client: &mut C) -> Result<Vec<String>, String>
where
    C: GenericClient,
{
    let query = "SELECT et.name FROM directory.entity_type et JOIN pg_class c ON et.name = c.relname JOIN pg_namespace ns ON ns.oid = relnamespace WHERE c.relkind = 'v' AND ns.nspname = 'virtual_entity'";
    let rows = client.query(query, &[]).await.map_err(|e| format!("{e}"))?;

    let names = rows.iter().map(|row| row.get(0)).collect();

    Ok(names)
}

async fn materialize_virtual_entity_type<C>(client: &mut C, name: &str) -> Result<u64, String>
where
    C: GenericClient,
{
    let mut tx = client
        .transaction()
        .await
        .map_err(|e| format!("Could not start transaction: {e}"))?;

    match materialize_virtual_entity(&mut tx, name).await {
        Ok(record_count) => {
            tx.commit()
                .await
                .map_err(|e| format!("Could not commit change: {e}"))?;
            Ok(record_count)
        }
        Err(e) => {
            tx.rollback()
                .await
                .map_err(|e| format!("Could not rollback change: {e}"))?;
            Err(format!("{e}"))
        }
    }
}

#[derive(Debug, Error)]
enum VirtualEntityMaterializeError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl From<VirtualEntityMaterializeError> for minerva::error::Error {
    fn from(err: VirtualEntityMaterializeError) -> minerva::error::Error {
        minerva::error::Error::Runtime(minerva::error::RuntimeError::from_msg(err.to_string()))
    }
}

async fn materialize_select_virtual_entities<C, T>(
    client: &mut C,
    names: &[T],
) -> Result<(), VirtualEntityMaterializeError>
where
    C: GenericClient,
    T: AsRef<str>,
{
    let mut error_count = 0;

    for name in names {
        match materialize_virtual_entity_type(client, name.as_ref()).await {
            Ok(record_count) => {
                println!(
                    "Materialized virtual entity '{}' ({} records)",
                    name.as_ref(),
                    record_count
                );
            }
            Err(e) => {
                println!("Error materializing relation '{}': {e}", name.as_ref());
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        return Err(VirtualEntityMaterializeError::RuntimeError(format!(
            "{error_count} virtual entity types failed to materialize"
        )));
    }

    Ok(())
}

async fn materialize_all_virtual_entities<C>(
    client: &mut C,
) -> Result<(), VirtualEntityMaterializeError>
where
    C: GenericClient,
{
    let entity_type_names = get_virtual_entity_type_names(client)
        .await
        .map_err(VirtualEntityMaterializeError::RuntimeError)?;

    materialize_select_virtual_entities(client, entity_type_names.as_slice()).await
}

impl VirtualEntityMaterialize {
    async fn materialize(&self) -> CmdResult {
        let mut client = connect_db().await?;

        if self.name.is_empty() {
            materialize_all_virtual_entities(&mut client)
                .await
                .map_err(|e| format!("{e}"))?;
        } else {
            materialize_select_virtual_entities(&mut client, &self.name)
                .await
                .map_err(|e| format!("{e}"))?;
        }

        Ok(())
    }
}

impl Cmd for VirtualEntityMaterialize {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.materialize())
    }
}
