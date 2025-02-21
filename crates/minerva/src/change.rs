use std::fmt;

use super::error::Error;
use async_trait::async_trait;
use std::marker::{Send, Sync};
use tokio_postgres::{Client, Transaction};

pub type ChangeResult = Result<String, Error>;

#[async_trait]
pub trait Change: fmt::Display + Send + Sync {
    async fn apply(&self, client: &mut Transaction) -> ChangeResult;
    async fn client_apply(&self, client: &mut Client) -> ChangeResult;

    // The default implementation for client_apply is:
    //
    // let mut tx = client.transaction().await?;
    // let result = self.apply(&mut tx).await?;
    // tx.commit().await?;
    // Ok(result)
}
