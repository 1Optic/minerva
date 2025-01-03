use std::path::PathBuf;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use tokio_postgres::{Client, Row};

use minerva::attribute_store::{
    load_attribute_store, load_attribute_store_from_file, AddAttributeStore, AttributeStore,
};
use minerva::change::Change;
use minerva::compact::{
    compact_attribute_store_by_id, compact_attribute_store_by_name, CompactError,
};
use minerva::error::{Error, RuntimeError};

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreCompact {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
    #[arg(long, help = "compact all modified attribute stores")]
    all_modified: bool,
    #[arg(long, help = "limit how many records to compact")]
    limit: Option<usize>,
    #[arg(long, help = "limit how many records to compact and loop until done")]
    limit_loop: Option<usize>,
}

#[async_trait]
impl Cmd for AttributeStoreCompact {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        client
            .execute("SET citus.max_intermediate_result_size = -1", &[])
            .await?;

        let (limit, loop_until_done) = if self.limit_loop.is_some() {
            (self.limit_loop, true)
        } else if self.limit.is_some() {
            (self.limit, false)
        } else {
            (None, false)
        };

        if let Some(id) = self.id {
            let transaction = client.transaction().await?;

            let result = compact_attribute_store_by_id(&transaction, id, limit).await?;

            transaction.commit().await?;

            println!(
                "Compacted attribute store '{}'({}): {}",
                result.attribute_store_name, result.attribute_store_id, result.record_count
            );
        } else if let Some(name) = &self.name {
            let mut done: bool = false;

            while !done {
                let transaction = client.transaction().await?;

                let result = compact_attribute_store_by_name(&transaction, name, limit).await?;

                transaction.commit().await?;

                println!(
                    "Compacted attribute store '{}'({}): {}",
                    result.attribute_store_name, result.attribute_store_id, result.record_count
                );

                if !loop_until_done || result.record_count == 0 {
                    done = true
                }
            }
        } else if self.all_modified {
            compact_all_attribute_stores(&mut client, limit).await?;
        }

        Ok(())
    }
}

async fn compact_all_attribute_stores(
    client: &mut Client,
    limit: Option<usize>,
) -> Result<(), CompactError> {
    let query = "SELECT ast.id, ast::text FROM attribute_directory.attribute_store ast LEFT JOIN attribute_directory.attribute_store_compacted astc ON astc.attribute_store_id = ast.id JOIN attribute_directory.attribute_store_modified astm ON astm.attribute_store_id = ast.id WHERE astc.compacted IS NULL OR astm.modified <> astc.compacted";

    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

    if rows.is_empty() {
        println!("All attribute stores are already compacted, nothing to do");
        return Ok(());
    }

    for row in rows {
        let id = row.get(0);
        let attribute_store_name: String = row.get(1);

        let transaction = client
            .transaction()
            .await
            .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

        println!(
            "Compacting attribute store '{}'({})",
            attribute_store_name, id
        );

        let result = compact_attribute_store_by_id(&transaction, id, limit).await?;

        // When any attribute data is compacted, also update the curr-ptr data
        if result.record_count > 0 {
            println!(
                "Materializing curr-ptr table for attribute store '{}'",
                attribute_store_name
            );

            let query = "SELECT attribute_directory.materialize_curr_ptr(ast) FROM attribute_directory.attribute_store ast WHERE id = $1";

            let row = transaction
                .query_one(query, &[&id])
                .await
                .map_err(|e| CompactError::CurrPtr(format!("{e}")))?;

            let record_count: i32 = row.get(0);

            println!(
                "Materialized curr-ptr table for attribute store '{}': {} records",
                attribute_store_name, record_count
            );
        }

        transaction
            .commit()
            .await
            .map_err(|e| CompactError::Unexpected(format!("{e}")))?;

        println!(
            "Compacted attribute store '{}'({}): {}",
            result.attribute_store_name, result.attribute_store_id, result.record_count
        );
    }

    Ok(())
}

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreMaterializeCurrPtr {
    #[arg(short, long, help = "Id of attribute store")]
    id: Option<i32>,
    #[arg(short, long, help = "name of attribute store")]
    name: Option<String>,
    #[arg(long, help = "materialize all modified attribute stores")]
    all_modified: bool,
}

#[async_trait]
impl Cmd for AttributeStoreMaterializeCurrPtr {
    async fn run(&self) -> CmdResult {
        let client = connect_db().await?;

        if let Some(id) = self.id {
            println!(
                "Materializing curr-ptr table for attribute store with Id {}",
                id
            );

            let query = "SELECT attribute_directory.materialize_curr_ptr(ast) FROM attribute_directory.attribute_store ast WHERE id = $1";

            let row = client.query_one(query, &[&id]).await?;

            let record_count: i32 = row.get(0);

            println!(
                "Materialized curr-ptr table for attribute store with Id {}: {} records",
                id, record_count
            );
        } else if let Some(name) = &self.name {
            println!(
                "Materializing curr-ptr table for attribute store '{}'",
                name
            );

            let query = "SELECT attribute_directory.materialize_curr_ptr(ast) FROM attribute_directory.attribute_store ast WHERE ast::text = $1";

            let row = client.query_one(query, &[&name]).await?;

            let record_count: i32 = row.get(0);

            println!(
                "Materialized curr-ptr table for attribute store '{}': {} records",
                name, record_count
            );
        } else if self.all_modified {
            let query = "SELECT ast.id, ast::text FROM attribute_directory.attribute_store ast LEFT JOIN attribute_directory.attribute_store_curr_materialized ascm ON ascm.attribute_store_id = ast.id LEFT JOIN attribute_directory.attribute_store_modified asm ON asm.attribute_store_id = ascm.attribute_store_id WHERE asm.modified <> ascm.materialized or (ascm.materialized is null and asm.modified is not null)";

            let rows = client.query(query, &[]).await?;

            for row in rows {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);

                println!(
                    "Materializing curr-ptr table for attribute store '{}'",
                    name
                );

                let query = "SELECT attribute_directory.materialize_curr_ptr(ast) FROM attribute_directory.attribute_store ast WHERE id = $1";

                let row = client.query_one(query, &[&id]).await?;

                let record_count: i32 = row.get(0);

                println!(
                    "Materialized curr-ptr table for attribute store '{}': {} records",
                    name, record_count
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreList {}

pub async fn list_attribute_stores(conn: &mut Client) -> Result<Vec<(i32, String)>, String> {
    let query = concat!(
        "SELECT ast.id, ast::text ",
        "FROM attribute_directory.attribute_store ast",
    );

    let result = conn.query(query, &[]).await.unwrap();

    let attribute_stores = result
        .into_iter()
        .map(|row: Row| (row.get::<usize, i32>(0), row.get::<usize, String>(1)))
        .collect();

    Ok(attribute_stores)
}

#[async_trait]
impl Cmd for AttributeStoreList {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let trend_stores = list_attribute_stores(&mut client).await.unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Id", "Name"]);

        for trend_store in trend_stores {
            table.add_row(vec![trend_store.0.to_string(), trend_store.1]);
        }

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreCreate {
    #[arg(help = "attribute store definition file")]
    definition: PathBuf,
}

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreUpdate {
    #[arg(help = "attribute store definition file")]
    definition: PathBuf,
}

#[derive(Debug, Parser, PartialEq)]
pub struct AttributeStoreOpt {
    #[command(subcommand)]
    command: AttributeStoreOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum AttributeStoreOptCommands {
    #[command(about = "list existing attribute stores")]
    List(AttributeStoreList),
    #[command(about = "create an attribute store")]
    Create(AttributeStoreCreate),
    #[command(about = "update an attribute store")]
    Update(AttributeStoreUpdate),
    #[command(about = "materialize attribute store curr-ptr table")]
    MaterializeCurrPtr(AttributeStoreMaterializeCurrPtr),
    #[command(about = "compact attribute store history")]
    Compact(AttributeStoreCompact),
}

impl AttributeStoreOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            AttributeStoreOptCommands::List(list) => list.run().await,
            AttributeStoreOptCommands::Create(args) => run_attribute_store_create_cmd(args).await,
            AttributeStoreOptCommands::Update(args) => run_attribute_store_update_cmd(args).await,
            AttributeStoreOptCommands::MaterializeCurrPtr(materialize_curr_ptr) => {
                materialize_curr_ptr.run().await
            }
            AttributeStoreOptCommands::Compact(compact) => compact.run().await,
        }
    }
}

async fn run_attribute_store_create_cmd(args: &AttributeStoreCreate) -> CmdResult {
    let attribute_store: AttributeStore = load_attribute_store_from_file(&args.definition)?;

    println!("Loaded definition, creating attribute store");

    let mut client = connect_db().await?;

    let change = AddAttributeStore { attribute_store };

    let mut tx = client.transaction().await?;

    tx.execute(
        "SET LOCAL citus.multi_shard_modify_mode TO 'sequential'",
        &[],
    )
    .await?;

    let result = change.apply(&mut tx).await;

    match result {
        Ok(_) => {
            tx.commit().await?;
            println!("Created attribute store");

            Ok(())
        }
        Err(e) => {
            tx.rollback().await?;
            Err(Error::Runtime(RuntimeError {
                msg: format!("Error creating attribute store: {e}"),
            }))
        }
    }
}

async fn run_attribute_store_update_cmd(args: &AttributeStoreUpdate) -> CmdResult {
    let attribute_store: AttributeStore = load_attribute_store_from_file(&args.definition)?;

    println!("Loaded definition, updating attribute store");

    let mut client = connect_db().await?;

    let attribute_store_db = load_attribute_store(
        &mut client,
        &attribute_store.data_source,
        &attribute_store.entity_type,
    )
    .await?;

    let changes = attribute_store_db.diff(&attribute_store);

    if !changes.is_empty() {
        println!("Updating attribute store");

        for change in changes {
            let mut tx = client.transaction().await?;

            let apply_result = change.apply(&mut tx).await;

            match apply_result {
                Ok(_) => {
                    tx.commit().await?;
                    println!("{}", &change);
                }
                Err(e) => {
                    println!("Error applying update: {e}");
                }
            }
        }
    } else {
        println!("Attribute store already up-to-date");
    }

    Ok(())
}
