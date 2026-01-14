use super::error::Error;
use async_trait::async_trait;
use std::fmt::{self, Display};
use std::marker::{Send, Sync};
use tokio_postgres::Client;

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
pub trait Change: fmt::Display + Send + Sync + erased_serde::Serialize {
    async fn apply(&self, client: &mut Client) -> ChangeResult;

    fn existing_object(&self) -> Option<MinervaObjectRef> {
        None
    }

    fn information_options(&self) -> Vec<Box<dyn InformationOption>> {
        Vec::new()
    }
}
