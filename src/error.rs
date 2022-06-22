//! this module contains this applications various error types

// std imports
use std::fmt::Display;
// external imports
pub use rusqlite::Error as RusqliteError;
// internal imports
use crate::types::SchemaVersion;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DatabaseErrorSource),
    TUIError(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "DatabaseError: {err})"),
            AppError::TUIError(err) => write!(f, "TUIError: {err})"),
        }
    }
}

pub type DatabaseResult<T> = std::result::Result<T, DatabaseErrorSource>;

#[derive(Debug)]
pub enum DatabaseErrorSource {
    SQLError(RusqliteError),
    InvalidSchemaError(SchemaVersion),
    DirCreationError(std::io::Error),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::TUIError(err)
    }
}

impl Display for DatabaseErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseErrorSource::SQLError(err) => write!(f, "SQLError: {err}"),
            DatabaseErrorSource::InvalidSchemaError(version)=> write!(
                f,
                "InvalidSchemaError: schema version was {version}. Was not equal to expected constant"
            ),
            DatabaseErrorSource::DirCreationError(err) => write!(f, "DirCreationError: {err})"),
        }
    }
}

impl From<std::io::Error> for DatabaseErrorSource {
    fn from(err: std::io::Error) -> Self {
        Self::DirCreationError(err)
    }
}

impl From<RusqliteError> for DatabaseErrorSource {
    fn from(err: RusqliteError) -> Self {
        Self::SQLError(err)
    }
}
