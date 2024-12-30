use clap::{Parser, Subcommand};

use service::TrendMaterializationService;

mod check;
mod create;
mod dump;
mod list;
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
    #[command(about = "start materialization service")]
    Service(TrendMaterializationService),
    #[command(about = "check materializations for inconsistencies")]
    Check(TrendMaterializationCheck),
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
            Some(TrendMaterializationOptCommand::Remove(trend_materialization_remove)) => {
                trend_materialization_remove.run().await
            }
            Some(TrendMaterializationOptCommand::PopulateSourceFingerprint(
                populate_source_fingerprint,
            )) => populate_source_fingerprint.run().await,
            Some(TrendMaterializationOptCommand::ResetSourceFingerprint(
                reset_source_fingerprint,
            )) => reset_source_fingerprint.run().await,
            Some(TrendMaterializationOptCommand::Dump(dump)) => dump.run().await,
            Some(TrendMaterializationOptCommand::List(list)) => list.run().await,
            Some(TrendMaterializationOptCommand::Service(service)) => service.run().await,
            Some(TrendMaterializationOptCommand::Check(check)) => check.run().await,
            None => Ok(()),
        }
    }
}
