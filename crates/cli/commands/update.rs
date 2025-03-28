use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use dialoguer::Select;
use minerva::change::Change;
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

async fn interact(client: &mut Client, change: &(dyn Change + Send)) -> Result<bool, Error> {
    let mut items: Vec<String> = vec!["Yes".to_string(), "No".to_string()];

    let information_options: Vec<String> = change
        .information_options()
        .iter()
        .map(|i| i.name())
        .collect();

    items.extend_from_slice(&information_options);

    let mut continuation_answer: Option<bool> = None;

    while continuation_answer.is_none() {
        let selection = Select::new()
            .with_prompt("Apply change?")
            .default(0)
            .items(&items)
            .interact()
            .map_err(|e| {
                Error::Runtime(RuntimeError {
                    msg: format!("Could not process input: {e}"),
                })
            })?;

        if selection > 1 {
            let information_option = &change.information_options()[selection - 2];

            let information = information_option.retrieve(client).await;

            println!();
            for line in information {
                println!("{}", line);
            }
            println!();
        } else {
            continuation_answer = Some(selection == 0);
        }
    }

    Ok(continuation_answer.unwrap())
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

    let num_changes = changes.len();

    for (index, change) in changes.iter().enumerate() {
        println!("\n\n* [{}/{num_changes}] {change}", index + 1);

        if !interactive || interact(client, change.as_ref()).await? {
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
