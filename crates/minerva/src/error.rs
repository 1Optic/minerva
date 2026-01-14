use std::fmt;

use tokio_postgres::{self, error::SqlState};

use crate::entity::EntityMappingError;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("0")]
    Default(String),
    #[error("0")]
    UniqueViolation(String),
}

impl DatabaseError {
    #[must_use]
    pub fn from_msg(msg: String) -> DatabaseError {
        DatabaseError::Default(msg)
    }

    pub fn from_postgres_error(msg: &str, e: tokio_postgres::Error) -> DatabaseError {
        DatabaseError::Default(format!("{msg}: {}", postgres_error_to_string(e)))
    }
}

pub fn postgres_error_to_string(error: tokio_postgres::Error) -> String {
    match error.as_db_error() {
        Some(db_error) => match db_error.detail() {
            Some(detail) => format!("{}: {}", db_error.message(), detail),
            None => db_error.message().to_string(),
        },
        None => error.to_string(),
    }
}

impl From<tokio_postgres::Error> for DatabaseError {
    fn from(err: tokio_postgres::Error) -> DatabaseError {
        let error_msg = match err.as_db_error() {
            Some(db_error) => match db_error.detail() {
                Some(detail) => format!("{}: {}", db_error.message(), detail),
                None => db_error.message().to_string(),
            },
            None => err.to_string(),
        };

        match err.code() {
            Some(&SqlState::UNIQUE_VIOLATION) => DatabaseError::UniqueViolation(error_msg),
            _ => DatabaseError::Default(error_msg),
        }
    }
}

#[derive(Debug)]
pub struct ConfigurationError {
    pub msg: String,
}

impl ConfigurationError {
    #[must_use]
    pub fn from_msg(msg: String) -> ConfigurationError {
        ConfigurationError { msg }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}

impl RuntimeError {
    #[must_use]
    pub fn from_msg(msg: String) -> RuntimeError {
        RuntimeError { msg }
    }
}

impl From<String> for RuntimeError {
    fn from(msg: String) -> RuntimeError {
        RuntimeError { msg }
    }
}

#[derive(Debug)]
pub enum Error {
    Database(DatabaseError),
    Configuration(ConfigurationError),
    Runtime(RuntimeError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            Error::Database(_) => "Database error",
            Error::Configuration(_) => "Configuration error",
            Error::Runtime(_) => "Runtime error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Database(e) => write!(f, "{}", &e),
            Error::Configuration(e) => write!(f, "{}", &e.msg),
            Error::Runtime(e) => write!(f, "{}", &e.msg),
        }
    }
}

impl From<DatabaseError> for Error {
    fn from(err: DatabaseError) -> Error {
        Error::Database(err)
    }
}

impl From<ConfigurationError> for Error {
    fn from(err: ConfigurationError) -> Error {
        Error::Configuration(err)
    }
}

impl From<RuntimeError> for Error {
    fn from(err: RuntimeError) -> Error {
        Error::Runtime(err)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        Error::Database(DatabaseError::from(err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Runtime(RuntimeError {
            msg: err.to_string(),
        })
    }
}

impl From<EntityMappingError> for Error {
    fn from(err: EntityMappingError) -> Error {
        Error::Runtime(RuntimeError {
            msg: err.to_string(),
        })
    }
}
