use clap::{Parser, Subcommand};

use service::TrendMaterializationService;

mod check;
mod create;
mod dump;
mod list;
mod listchunks;
mod populatesourcefingerprint;
mod remove;
mod resetsourcefingerprint;
mod service;
mod update;

use super::common::{Cmd, CmdResult};
use crate::commands::trendmaterialization::check::TrendMaterializationCheck;
use crate::commands::trendmaterialization::create::TrendMaterializationCreate;
use crate::commands::trendmaterialization::dump::TrendMaterializationDump;
use crate::commands::trendmaterialization::list::TrendMaterializationList;
use crate::commands::trendmaterialization::listchunks::TrendMaterializationChunkList;
use crate::commands::trendmaterialization::populatesourcefingerprint::TrendMaterializationPopulateSourceFingerprint;
use crate::commands::trendmaterialization::remove::TrendMaterializationRemove;
use crate::commands::trendmaterialization::resetsourcefingerprint::TrendMaterializationResetSourceFingerprint;
use crate::commands::trendmaterialization::update::TrendMaterializationUpdate;

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
    #[command(about = "remove a trend materialization")]
    Remove(TrendMaterializationRemove),
    #[command(about = "reset the source fingerprint of the materialization state")]
    ResetSourceFingerprint(TrendMaterializationResetSourceFingerprint),
    #[command(about = "populate the source fingerprint of the materialization state")]
    PopulateSourceFingerprint(TrendMaterializationPopulateSourceFingerprint),
    #[command(about = "dump the definition of a trend materialization")]
    Dump(TrendMaterializationDump),
    #[command(about = "list trend materializations")]
    List(TrendMaterializationList),
    #[command(about = "list trend materialization chunks that are pending materialization")]
    ListChunks(TrendMaterializationChunkList),
    #[command(about = "start materialization service")]
    Service(TrendMaterializationService),
    #[command(about = "check materializations for inconsistencies")]
    Check(TrendMaterializationCheck),
}

impl TrendMaterializationOpt {
    /// # Errors
    ///
    /// Will return `Err` if a subcommand returns an error.
    pub fn run(&self) -> CmdResult {
        match &self.command {
            Some(TrendMaterializationOptCommand::Create(trend_materialization_create)) => {
                trend_materialization_create.run()
            }
            Some(TrendMaterializationOptCommand::Update(trend_materialization_update)) => {
                trend_materialization_update.run()
            }
            Some(TrendMaterializationOptCommand::Remove(trend_materialization_remove)) => {
                trend_materialization_remove.run()
            }
            Some(TrendMaterializationOptCommand::PopulateSourceFingerprint(
                populate_source_fingerprint,
            )) => populate_source_fingerprint.run(),
            Some(TrendMaterializationOptCommand::ResetSourceFingerprint(
                reset_source_fingerprint,
            )) => reset_source_fingerprint.run(),
            Some(TrendMaterializationOptCommand::Dump(dump)) => dump.run(),
            Some(TrendMaterializationOptCommand::List(list)) => list.run(),
            Some(TrendMaterializationOptCommand::ListChunks(list_chunks)) => list_chunks.run(),
            Some(TrendMaterializationOptCommand::Service(service)) => service.run(),
            Some(TrendMaterializationOptCommand::Check(check)) => check.run(),
            None => Ok(()),
        }
    }
}
