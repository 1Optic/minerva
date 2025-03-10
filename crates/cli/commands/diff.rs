use std::env;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use erased_serde::Serializer;
use minerva::error::{ConfigurationError, Error};
use minerva::instance::{DiffOptions, MinervaInstance};

use super::common::{
    connect_to_db, get_db_config, show_db_config, Cmd, CmdResult, ENV_MINERVA_INSTANCE_ROOT,
};

#[derive(Debug, Parser, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct DiffOpt {
    #[arg(
        long = "with-dir",
        help = "compare with other Minerva instance directory"
    )]
    with_dir: Option<PathBuf>,
    #[arg(long)]
    ignore_trend_extra_data: bool,
    #[arg(long)]
    ignore_trend_data_type: bool,
    #[arg(long)]
    ignore_deletions: bool,
    #[arg(long, help = "output diff in json format")]
    json: bool,
}

#[async_trait]
impl Cmd for DiffOpt {
    async fn run(&self) -> CmdResult {
        let minerva_instance_root = match env::var(ENV_MINERVA_INSTANCE_ROOT) {
            Ok(v) => PathBuf::from(v),
            Err(e) => {
                return Err(Error::Configuration(ConfigurationError {
                    msg: format!(
                        "Environment variable '{}' could not be read: {}",
                        &ENV_MINERVA_INSTANCE_ROOT, e
                    ),
                }));
            }
        };

        let from_instance_descr = format!("dir('{}')", minerva_instance_root.to_string_lossy());
        let to_instance_descr: String;

        let instance_def = MinervaInstance::load_from(&minerva_instance_root)?;

        let other_instance = if let Some(with_dir) = &self.with_dir {
            to_instance_descr = format!("dir('{}')", with_dir.to_string_lossy());
            MinervaInstance::load_from(with_dir)?
        } else {
            let db_config = get_db_config()?;

            let db_config_text = show_db_config(&db_config);

            to_instance_descr = format!("database('{db_config_text}')");

            let mut client = connect_to_db(&db_config).await?;

            MinervaInstance::load_from_db(&mut client).await?
        };

        let diff_options = DiffOptions {
            ignore_trend_extra_data: self.ignore_trend_extra_data,
            ignore_trend_data_type: self.ignore_trend_data_type,
            ignore_deletions: self.ignore_deletions,
        };

        let changes = other_instance.diff(&instance_def, diff_options);

        if changes.is_empty() {
            println!("Database is up-to-date");
        } else {
            if !self.json {
                println!("Differences {from_instance_descr} -> {to_instance_descr}");
            }

            for change in changes {
                if self.json {
                    let json = &mut serde_json::Serializer::new(std::io::stdout());
                    let mut serializer = Box::new(<dyn Serializer>::erase(json));
                    let _ = change.erased_serialize(&mut serializer);
                } else {
                    println!("* {}", &change);
                }
            }
        }

        Ok(())
    }
}
