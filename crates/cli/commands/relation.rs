use std::path::PathBuf;

use async_trait::async_trait;
use tokio_postgres::Client;

use minerva::change::Change;
use minerva::relation::{
    load_relation_from_file, materialize_relation, AddRelation, UpdateRelation,
};

use clap::{Parser, Subcommand};

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct RelationCreate {
    #[arg(help = "trigger definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for RelationCreate {
    async fn run(&self) -> CmdResult {
        let relation = load_relation_from_file(&self.definition)?;

        println!("Loaded definition, creating trigger");

        let mut client = connect_db().await?;

        let change = AddRelation { relation };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct RelationUpdate {
    #[arg(help = "relation definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for RelationUpdate {
    async fn run(&self) -> CmdResult {
        let relation = load_relation_from_file(&self.definition)?;

        println!("Loaded definition, updating relation");

        let mut client = connect_db().await?;

        let change = UpdateRelation { relation };

        let message = change.apply(&mut client).await?;

        println!("{message}");

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct RelationMaterialize {
    #[arg(help = "relation name")]
    name: Option<String>,
}

async fn get_relation_names(client: &Client) -> Vec<String> {
    let rows = client
        .query("SELECT name FROM relation_directory.type", &[])
        .await
        .unwrap();

    rows.iter().map(|row| row.get(0)).collect()
}

#[async_trait]
impl Cmd for RelationMaterialize {
    async fn run(&self) -> CmdResult {
        let mut error_count = 0;

        let mut client = connect_db().await?;

        let relation_names = match &self.name {
            Some(name) => vec![name.clone()],
            None => get_relation_names(&client).await,
        };

        for name in relation_names {
            let mut tx = client.transaction().await?;

            let materialization_definition_query = concat!(
                "SELECT 1 FROM pg_catalog.pg_class c ",
                "JOIN pg_catalog.pg_namespace ns ",
                "ON c.relnamespace = ns.oid ",
                "WHERE relname = $1 AND nspname = $2"
            );

            match tx
                .query_one(materialization_definition_query, &[&name, &"relation_def"])
                .await
            {
                Ok(_) => match materialize_relation(&mut tx, &name).await {
                    Ok(changed) => {
                        tx.commit().await?;
                        println!(
                            "Materialized relation '{name}' (deleted {}, inserted {})",
                            changed.deleted_count, changed.inserted_count
                        );
                    }
                    Err(e) => {
                        tx.rollback().await?;
                        println!("Error materializing relation '{name}': {e}");
                        error_count += 1;
                    }
                },
                Err(_) => {
                    println!(
                        "WARNING: Relation {name} does not have a defining view and is skipped."
                    )
                }
            }
        }

        if error_count > 0 {
            return Err(minerva::error::Error::Runtime(
                minerva::error::RuntimeError::from_msg(format!(
                    "{error_count} relations failed to materialize"
                )),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct RelationOpt {
    #[command(subcommand)]
    command: RelationOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum RelationOptCommands {
    #[command(about = "create a relation")]
    Create(RelationCreate),
    #[command(about = "update a relation")]
    Update(RelationUpdate),
    #[command(about = "materialize a relation")]
    Materialize(RelationMaterialize),
}

impl RelationOpt {
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            RelationOptCommands::Create(create) => create.run().await,
            RelationOptCommands::Update(update) => update.run().await,
            RelationOptCommands::Materialize(materialize) => materialize.run().await,
        }
    }
}
