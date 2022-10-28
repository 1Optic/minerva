use std::fmt;

use super::error::Error;
use async_trait::async_trait;
use std::marker::{Send, Sync};
use tokio_postgres::{Client, GenericClient};

pub type ChangeResult = Result<String, Error>;

#[async_trait]
pub trait Change: fmt::Display + Send + Sync {
    async fn apply(&self, client: &mut Client) -> ChangeResult;
}

#[async_trait]
pub trait GenericChange: fmt::Display + Send + Sync {
    async fn generic_apply<T: GenericClient + Send + Sync>(&self, client: &mut T) -> ChangeResult;
}
