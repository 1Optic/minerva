use clap::{Parser, Subcommand};

pub mod check;
pub mod create;
pub mod delete;
pub mod deletetimestamp;
pub mod diff;
pub mod dump;
pub mod list;
pub mod part;
pub mod partition;
pub mod renametrend;
pub mod statistics;
pub mod update;

use crate::commands::common::{Cmd, CmdResult};
use check::TrendStoreCheck;
use create::TrendStoreCreate;
use delete::TrendStoreDelete;
use deletetimestamp::TrendStoreDeleteTimestamp;
use diff::TrendStoreDiff;
use dump::TrendStoreDump;
use list::TrendStoreList;
use part::{TrendStorePartOpt, TrendStorePartOptCommands};
use partition::{TrendStorePartition, TrendStorePartitionCommands};
use renametrend::TrendStoreRenameTrend;
use statistics::TrendStoreStatistics;
use update::TrendStoreUpdate;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreOpt {
    #[command(subcommand)]
    command: TrendStoreOptCommands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TrendStoreOptCommands {
    #[command(about = "list existing trend stores")]
    List(TrendStoreList),
    #[command(about = "create a trend store")]
    Create(TrendStoreCreate),
    #[command(about = "show differences for a trend store")]
    Diff(TrendStoreDiff),
    #[command(about = "update a trend store")]
    Update(TrendStoreUpdate),
    #[command(about = "delete a trend store")]
    Delete(TrendStoreDelete),
    #[command(about = "partition management commands")]
    Partition(TrendStorePartition),
    #[command(about = "run sanity checks for trend store")]
    Check(TrendStoreCheck),
    #[command(about = "part management commands")]
    Part(TrendStorePartOpt),
    #[command(about = "rename a trend")]
    RenameTrend(TrendStoreRenameTrend),
    #[command(about = "delete all data for a specific timestamp")]
    DeleteTimestamp(TrendStoreDeleteTimestamp),
    #[command(about = "dump the definition of a trend store")]
    Dump(TrendStoreDump),
    #[command(about = "recalculate all trend statistics")]
    Statistics(TrendStoreStatistics),
}

impl TrendStoreOpt {
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub async fn run(&self) -> CmdResult {
        match &self.command {
            TrendStoreOptCommands::List(list) => list.run().await,
            TrendStoreOptCommands::Create(create) => create.run().await,
            TrendStoreOptCommands::Diff(diff) => diff.run().await,
            TrendStoreOptCommands::Update(update) => update.run().await,
            TrendStoreOptCommands::Delete(delete) => delete.run().await,
            TrendStoreOptCommands::Partition(partition) => match &partition.command {
                TrendStorePartitionCommands::Create(create) => create.run().await,
                TrendStorePartitionCommands::Remove(remove) => remove.run().await,
                TrendStorePartitionCommands::Columnar(columnarize) => columnarize.run().await,
            },
            TrendStoreOptCommands::Check(check) => check.run().await,
            TrendStoreOptCommands::Part(part) => match &part.command {
                TrendStorePartOptCommands::Analyze(analyze) => analyze.run().await,
            },
            TrendStoreOptCommands::RenameTrend(rename_trend) => rename_trend.run().await,
            TrendStoreOptCommands::DeleteTimestamp(delete_timestamp) => {
                delete_timestamp.run().await
            }
            TrendStoreOptCommands::Dump(dump) => dump.run().await,
            TrendStoreOptCommands::Statistics(stats) => stats.run().await,
        }
    }
}
