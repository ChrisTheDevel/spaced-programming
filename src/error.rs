use rusqlite::Error as RusqliteError;
use std::error::Error;
use std::fmt::Display;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DatabaseErrorSource),
    TUIError(std::io::Error),
    SchedulerError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "DatabaseError: {err}"),
            AppError::TUIError(err) => write!(f, "TUIError: {err}"),
            AppError::SchedulerError => write!(f, "SchedulerError"),
        }
    }
}

impl From<DatabaseErrorSource> for AppError {
    fn from(err: DatabaseErrorSource) -> Self {
        Self::DatabaseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::TUIError(err)
    }
}

pub type DatabaseResult<T> = std::result::Result<T, DatabaseErrorSource>;

#[derive(Debug)]
pub enum DatabaseErrorSource {
    InvalidSchemaError(String),
    DirectoryCreationError(std::io::Error),
    SQLError(RusqliteError),
}

impl Display for DatabaseErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseErrorSource::InvalidSchemaError(err) => write!(f, "InvalidSchemaError: {err}"),
            DatabaseErrorSource::DirectoryCreationError(err) => {
                write!(f, "DirectoryCreationError: {err}")
            }
            DatabaseErrorSource::SQLError(err) => write!(f, "SQLError: {err}"),
        }
    }
}

impl From<std::io::Error> for DatabaseErrorSource {
    fn from(err: std::io::Error) -> Self {
        Self::DirectoryCreationError(err)
    }
}
impl From<RusqliteError> for DatabaseErrorSource {
    fn from(err: RusqliteError) -> Self {
        Self::SQLError(err)
    }
}
