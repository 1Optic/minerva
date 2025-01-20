use clap::Parser;
use async_trait::async_trait;

use crate::commands::common::{connect_db, Cmd, CmdResult};

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::delete_trend_store;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDelete {
    id: i32,
}

#[async_trait]
impl Cmd for TrendStoreDelete {
    async fn run(&self) -> CmdResult {
        println!("Deleting trend store {}", self.id);

        let mut client = connect_db().await?;

        let result = delete_trend_store(&mut client, self.id).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Runtime(RuntimeError {
                msg: format!("Error deleting trend store: {e}"),
            })),
        }
    }
}
