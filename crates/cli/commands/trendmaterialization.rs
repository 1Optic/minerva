use std::path::PathBuf;

use async_trait::async_trait;
use clap::{Parser, Subcommand, ValueHint};

use minerva::change::Change;
use minerva::error::{Error, RuntimeError};
use minerva::trend_materialization::{
    load_materializations, populate_source_fingerprint, reset_source_fingerprint,
    trend_materialization_from_config, AddTrendMaterialization, UpdateTrendMaterialization,
};

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationCreate {
    #[arg(help = "trend materialization definition file", value_hint = ValueHint::FilePath)]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TrendMaterializationCreate {
    async fn run(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, creating trend materialization");
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        let change = AddTrendMaterialization {
            trend_materialization,
        };

        let result = change.apply(&mut transaction).await;

        transaction.commit().await?;

        match result {
            Ok(_) => {
                println!("Created trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error creating trend materialization: {e}"),
            })),
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationUpdate {
    #[arg(help = "trend materialization definition file")]
    definition: PathBuf,
}

#[async_trait]
impl Cmd for TrendMaterializationUpdate {
    async fn run(&self) -> CmdResult {
        let trend_materialization = trend_materialization_from_config(&self.definition)?;

        println!("Loaded definition, updating trend materialization");
        let mut client = connect_db().await?;

        let mut transaction = client.transaction().await?;

        let change = UpdateTrendMaterialization {
            trend_materialization,
        };

        let result = change.apply(&mut transaction).await;

        transaction.commit().await?;

        match result {
            Ok(_) => {
                println!("Updated trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error updating trend materialization: {e}"),
            })),
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationPopulateSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationPopulateSourceFingerprint {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let result = populate_source_fingerprint(&mut client, &self.materialization).await;

        match result {
            Ok(_) => {
                println!(
                    "Populated state for trend materialization '{}'",
                    &self.materialization
                );

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!(
                    "Error populating state for trend materialization '{}': {e}",
                    &self.materialization
                ),
            })),
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationResetSourceFingerprint {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationResetSourceFingerprint {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let result = reset_source_fingerprint(&mut client, &self.materialization).await;

        match result {
            Ok(_) => {
                println!("Updated trend materialization");

                Ok(())
            }
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error updating trend materialization: {e}"),
            })),
        }
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationDump {
    #[arg(help = "materialization ")]
    materialization: String,
}

#[async_trait]
impl Cmd for TrendMaterializationDump {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        let materializations = load_materializations(&mut client).await?;

        for materialization in materializations {
            let materialization_name = materialization.name();
            if materialization_name == self.materialization {
                let definition = materialization.dump().unwrap();

                println!("{}", definition);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationList {}

#[async_trait]
impl Cmd for TrendMaterializationList {
    async fn run(&self) -> CmdResult {
        let client = connect_db().await?;

        let rows = client
            .query(
                "SELECT m.id, tsp.name FROM trend_directory.materialization m JOIN trend_directory.trend_store_part tsp ON tsp.id = m.dst_trend_store_part_id",
                &[]
            )
            .await
            .unwrap();

        let mut table = comfy_table::Table::new();
        let style = "     ═╪ ┆          ";
        table.load_preset(style);
        table.set_header(vec!["Id", "Name"]);

        for row in rows {
            let id: i32 = row.get(0);
            let name: &str = row.get(1);
            table.add_row(vec![id.to_string(), name.to_string()]);
        }

        println!("{table}");

        Ok(())
    }
}

#[derive(Debug, Parser, PartialEq)]
pub struct TrendMaterializationOpt {
    #[command(subcommand)]
    command: Option<TrendMaterializationOptCommand>,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TrendMaterializationOptCommand {
    #[command(about = "create a trend materialization")]
    Create(TrendMaterializationCreate),
    #[command(about = "update a trend materialization")]
    Update(TrendMaterializationUpdate),
    #[command(about = "reset the source fingerprint of the materialization state")]
    ResetSourceFingerprint(TrendMaterializationResetSourceFingerprint),
    #[command(about = "populate the source fingerprint of the materialization state")]
    PopulateSourceFingerprint(TrendMaterializationPopulateSourceFingerprint),
    #[command(about = "dump the definition of a trend materialization")]
    Dump(TrendMaterializationDump),
    #[command(about = "list trend materializations")]
    List(TrendMaterializationList),
}

impl TrendMaterializationOpt {
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            Some(TrendMaterializationOptCommand::Create(trend_materialization_create)) => {
                trend_materialization_create.run().await
            }
            Some(TrendMaterializationOptCommand::Update(trend_materialization_update)) => {
                trend_materialization_update.run().await
            }
            Some(TrendMaterializationOptCommand::PopulateSourceFingerprint(
                populate_source_fingerprint,
            )) => populate_source_fingerprint.run().await,
            Some(TrendMaterializationOptCommand::ResetSourceFingerprint(
                reset_source_fingerprint,
            )) => reset_source_fingerprint.run().await,
            Some(TrendMaterializationOptCommand::Dump(dump)) => dump.run().await,
            Some(TrendMaterializationOptCommand::List(list)) => list.run().await,
            None => Ok(()),
        }
    }
}
