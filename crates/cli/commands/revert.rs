use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;

use erased_serde::Serializer;
use minerva::change::Changed;
use minerva::error::RuntimeError;

use minerva::error::Error;

use crate::interact::interact;

use super::common::{connect_db, Cmd, CmdResult};

#[derive(Debug, Parser, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct RevertOpt {
    #[arg(short, long)]
    non_interactive: bool,
    #[arg(short, long)]
    log_dir: Option<PathBuf>,
}

impl RevertOpt {
    async fn revert(&self) -> CmdResult {
        let default_log_dir = PathBuf::from("/var/lib/minerva/log");
        let log_dir = self.log_dir.clone().unwrap_or(default_log_dir);

        if !log_dir.exists() {
            return Err(Error::Runtime(RuntimeError::from_msg(format!(
                "No such directory '{}'",
                log_dir.to_string_lossy()
            ))));
        }

        let mut change_file_list = Vec::new();

        for entry in std::fs::read_dir(&log_dir).unwrap() {
            let e = entry.unwrap();

            let path = e.path();

            if path.is_file() {
                change_file_list.push(path);
            }
        }

        change_file_list.sort();

        let mut client = connect_db().await?;

        println!("Applying changes:");

        let num_changes = change_file_list.len();

        for (index, path) in change_file_list.iter().enumerate() {
            println!(
                "\n\n* [{}/{num_changes}] {}",
                index + 1,
                path.to_string_lossy()
            );

            let change_file = std::fs::File::open(path).unwrap();
            let reader = BufReader::new(change_file);
            let changed: Box<dyn Changed> = serde_json::from_reader(reader).map_err(|e| {
                minerva::error::RuntimeError::from_msg(format!("Could not load diff: {e}"))
            })?;

            println!("Change: {}", changed);

            match changed.revert() {
                Some(change) => {
                    println!("Revert: {}", change);

                    if interact(&mut client, change.as_ref()).await? {
                        match change.apply(&mut client).await {
                            Ok(changed) => {
                                let now = chrono::offset::Local::now();

                                let mut file_path: PathBuf = log_dir.clone();

                                file_path.push(format!("{}.json", now.to_rfc3339()));

                                let file = File::create(file_path.clone()).map_err(|e| {
                                    format!(
                                        "Could not write entity materialization to '{}': {e}",
                                        file_path.to_string_lossy()
                                    )
                                })?;

                                let writer = BufWriter::new(file);

                                let json = &mut serde_json::Serializer::new(writer);
                                let mut serializer = Box::new(<dyn Serializer>::erase(json));
                                let _ = changed.erased_serialize(&mut serializer);

                                println!("> {}", &changed);
                            }
                            Err(err) => {
                                println!("! Error applying change: {}", &err);
                            }
                        }
                    }
                }
                None => {
                    println!("Nothing to revert");
                }
            }
        }

        Ok(())
    }
}

impl Cmd for RevertOpt {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.revert())
    }
}
