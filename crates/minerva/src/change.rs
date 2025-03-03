use std::fmt;

use super::error::Error;
use async_trait::async_trait;
use std::marker::{Send, Sync};
use tokio_postgres::Client;

pub type ChangeResult = Result<String, Error>;

#[async_trait]
pub trait Change: fmt::Display + Send + Sync {
    async fn apply(&self, client: &mut Client) -> ChangeResult;
}
