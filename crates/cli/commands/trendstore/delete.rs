use clap::Parser;

use crate::commands::common::{connect_db, Cmd, CmdResult};

use minerva::error::{Error, RuntimeError};
use minerva::trend_store::delete_trend_store;

#[derive(Debug, Parser, PartialEq)]
pub struct TrendStoreDelete {
    id: i32,
}

impl TrendStoreDelete {
    async fn delete(&self) -> CmdResult {
        println!("Deleting trend store {}", self.id);

        let mut client = connect_db().await?;

        delete_trend_store(&mut client, self.id).await.map_err(|e| {
            Error::Runtime(RuntimeError {
                msg: format!("Error deleting trend store: {e}"),
            })
        })
    }
}

impl Cmd for TrendStoreDelete {
    fn run(&self) -> CmdResult {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.delete())
    }
}
