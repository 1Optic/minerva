use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use dialoguer::Confirm;

use minerva::instance::DiffOptions;
use tokio_postgres::Client;

use minerva::error::{ConfigurationError, Error, RuntimeError};
use minerva::instance::MinervaInstance;

use super::common::{connect_db, Cmd, CmdResult, ENV_MINERVA_INSTANCE_ROOT};

#[derive(Debug, Parser, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct UpdateOpt {
    #[arg(short, long)]
    non_interactive: bool,
    #[arg(help = "Minerva instance root directory")]
    instance_root: Option<PathBuf>,
    #[arg(long)]
    ignore_trend_extra_data: bool,
    #[arg(long)]
    ignore_trend_data_type: bool,
    #[arg(long)]
    ignore_deletions: bool,
}

#[async_trait]
impl Cmd for UpdateOpt {
    async fn run(&self) -> CmdResult {
        let mut client = connect_db().await?;

        print!("Reading Minerva instance from database... ");
        io::stdout().flush().unwrap();
        let instance_db = MinervaInstance::load_from_db(&mut client).await?;
        println!("Ok");

        let minerva_instance_root = match &self.instance_root {
            Some(root) => {
                // Next to passing on the Minerva instance root directory, we need to set the
                // environment variable for any child processes that might be started during
                // initialization.
                std::env::set_var(ENV_MINERVA_INSTANCE_ROOT, root);

                root.clone()
            }
            None => match env::var(ENV_MINERVA_INSTANCE_ROOT) {
                Ok(v) => PathBuf::from(v),
                Err(e) => {
                    return Err(Error::Configuration(ConfigurationError {
                        msg: format!(
                            "Environment variable '{}' could not be read: {}",
                            &ENV_MINERVA_INSTANCE_ROOT, e
                        ),
                    }));
                }
            },
        };

        print!(
            "Reading Minerva instance from '{}'... ",
            &minerva_instance_root.to_string_lossy()
        );
        io::stdout().flush().unwrap();
        let instance_def = MinervaInstance::load_from(&minerva_instance_root)?;
        println!("Ok");

        let diff_options = DiffOptions {
            ignore_trend_extra_data: self.ignore_trend_extra_data,
            ignore_trend_data_type: self.ignore_trend_data_type,
            ignore_deletions: self.ignore_deletions,
        };

        update(
            &mut client,
            &instance_db,
            &instance_def,
            !self.non_interactive,
            diff_options,
        )
        .await
    }
}

async fn update(
    client: &mut Client,
    db_instance: &MinervaInstance,
    other: &MinervaInstance,
    interactive: bool,
    diff_options: DiffOptions,
) -> CmdResult {
    let changes = db_instance.diff(other, diff_options);

    println!("Applying changes:");

    for change in changes {
        println!("\n\n* {change}");

        if (!interactive)
            || Confirm::new()
                .with_prompt("Apply change?")
                .interact()
                .map_err(|e| {
                    Error::Runtime(RuntimeError {
                        msg: format!("Could not process input: {e}"),
                    })
                })?
        {
            match change.apply(client).await {
                Ok(message) => {
                    println!("> {}", &message);
                }
                Err(err) => {
                    println!("! Error applying change: {}", &err);
                }
            }
        }
    }

    Ok(())
}
