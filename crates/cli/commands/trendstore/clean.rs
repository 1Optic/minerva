use erased_serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::Parser;

use minerva::change::Change;
use minerva::changes::trend_store::RemoveTrends;
use minerva::error::RuntimeError;
use minerva::trend_store::get_trends_to_delete;

use crate::commands::common::{Cmd, CmdResult, connect_db};
use crate::commands::update::update_variation;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreClean {
    #[arg(short, long)]
    report: bool,
    #[arg(short, long)]
    interactive: bool,
    #[arg(long)]
    log_dir: Option<PathBuf>,
}

impl TrendStoreClean {
    async fn clean(&self) -> CmdResult {
        let default_log_dir = PathBuf::from("/var/lib/minerva/log");
        let log_dir = self.log_dir.clone().unwrap_or(default_log_dir);

        if !log_dir.exists() {
            create_dir_all(&log_dir).map_err(|e| {
                RuntimeError::from_msg(format!(
                    "Could not create log directory '{}': {e}",
                    log_dir.to_string_lossy()
                ))
            })?;
        }

        let mut client = connect_db().await?;

        let trends_to_delete = get_trends_to_delete(&mut client).await?;

        let mut trend_store_parts_with_deletions: HashMap<String, Vec<String>> = HashMap::new();

        for trend in &trends_to_delete {
            trend_store_parts_with_deletions
                .entry(trend.0.clone())
                .or_default()
                .push(trend.1.clone());
        }

        let changes = trend_store_parts_with_deletions
            .into_iter()
            .map(|(trend_store_part_name, trends)| {
                Box::new(RemoveTrends {
                    trend_store_part_name,
                    trends,
                }) as Box<dyn Change>
            })
            .collect::<Vec<_>>();

        if self.report {
            let json = &mut serde_json::Serializer::new(std::io::stdout());
            let mut serializer = Box::new(<dyn Serializer>::erase(json));
            let _ = changes.erased_serialize(&mut serializer);
        } else {
            update_variation(&mut client, &log_dir, changes, self.interactive).await?;
        }
        Ok(())
    }
}

impl Cmd for TrendStoreClean {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.clean())
    }
}
