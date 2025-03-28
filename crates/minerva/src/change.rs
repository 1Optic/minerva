use std::fmt;

use super::error::Error;
use async_trait::async_trait;
use std::marker::{Send, Sync};
use tokio_postgres::Client;

pub type ChangeResult = Result<String, Error>;

#[async_trait]
pub trait InformationOption: fmt::Display + Send + Sync {
    fn name(&self) -> String;
    async fn retrieve(&self, client: &mut Client) -> Vec<String>;
}

#[async_trait]
pub trait Change: fmt::Display + Send + Sync + erased_serde::Serialize {
    async fn apply(&self, client: &mut Client) -> ChangeResult;
    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        Vec::new()
    }
}
