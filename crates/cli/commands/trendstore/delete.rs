use async_trait::async_trait;
use clap::Parser;

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

        delete_trend_store(&mut client, self.id).await.map_err(|e| {
            Error::Runtime(RuntimeError {
                msg: format!("Error deleting trend store: {e}"),
            })
        })
    }
}
