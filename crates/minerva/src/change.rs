use super::error::Error;
use std::fmt::Debug;
use async_trait::async_trait;
use std::fmt::{self, Display};
use std::marker::{Send, Sync};
use tokio_postgres::Client;
use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "type")]
pub trait Changed: erased_serde::Serialize + Display + Send + Sync {
    fn revert(&self) -> Option<Box<dyn Change>>;
}

pub type ChangeResult = Result<Box<dyn Changed>, Error>;

#[async_trait]
pub trait InformationOption: fmt::Display + Send + Sync {
    fn name(&self) -> String;
    async fn retrieve(&self, client: &mut Client) -> Vec<String>;
}

// Points to a existing object by type and name
pub enum MinervaObjectRef {
    TrendStorePart(String),
    TrendFunctionMaterialization(String),
    AttributeStore(String),
    AttributeMaterialization(String),
    TrendViewMaterialization(String),
    Relation(String),
    VirtualEntity(String),
}

impl Display for MinervaObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrendStorePart(name) => write!(f, "TrendStorePart({name})"),
            Self::TrendFunctionMaterialization(name) => {
                write!(f, "TrendFunctionMaterialization({name})")
            }
            Self::AttributeStore(name) => write!(f, "AttributeStore({name})"),
            Self::AttributeMaterialization(name) => write!(f, "AttributeMaterialization({name})"),
            Self::TrendViewMaterialization(name) => write!(f, "TrendViewMaterialization({name})"),
            Self::Relation(name) => write!(f, "Relation({name})"),
            Self::VirtualEntity(name) => write!(f, "VirtualEntity({name})"),
        }
    }
}

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Change: fmt::Display + Send + Sync + Debug + erased_serde::Serialize {
    async fn apply(&self, client: &mut Client) -> ChangeResult;

    fn existing_object(&self) -> Option<MinervaObjectRef> {
        None
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        Vec::new()
    }

    fn is_sse_change(&self) -> bool {
        false
    }

    async fn apply_no_sse(&self, client: &mut Client) -> ChangeResult {
        if !self.is_sse_change() {
            self.apply(client).await
        } else {
            Ok(Box::new(NoChange {}))
        }
    }

    fn remove_sse_changes(&mut self) -> () {}
}

#[derive(Serialize, Deserialize, Debug)]
struct NoChange {}

impl Display for NoChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoChange")
    }
}

#[async_trait]
#[typetag::serde]
impl Change for NoChange {
    async fn apply(&self, _client: &mut Client) -> ChangeResult {
        Ok(Box::new(NoChange {}))
    }
}

#[typetag::serde]
impl Changed for NoChange {
    fn revert(&self) -> Option<Box<dyn Change>> {
        Some(Box::new(NoChange {}))
    }
}
