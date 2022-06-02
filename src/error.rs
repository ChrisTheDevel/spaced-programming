use std::fmt::Display;
use std::error::Error;

pub type BackendResult<T> = std::result::Result<T, BackendError>;

pub enum BackendError  {
    DatabaseInitError(DatabaseInitErrorSource),
    SqliteError(SQLError),
}

pub enum DatabaseInitErrorSource {
    CouldNotCreatePaths(std::io::Error),
    InvalidSchemaVersion(String),
}

impl Display for DatabaseInitErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            DatabaseInitErrorSource::CouldNotCreatePaths(err) => format!("{err}"),
            DatabaseInitErrorSource::InvalidSchemaVersion(str) => format!("Invalid schema: {str}"),
        };
        write!(f, "{message}")

    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_error_message = match self {
            BackendError::DatabaseInitError(err) => format!("DatabaseInitError: {err}"),
            BackendError::SqliteError(err) => format!("SqliteError ==== \n\t{}", err),
        };
        write!(f, "{full_error_message}")
    }
}

use rusqlite::Error as SQLError;
impl From<SQLError> for BackendError {
    fn from(e: SQLError) -> Self {
        BackendError::SqliteError(e)
    }
}

impl From<std::io::Error> for BackendError {
    fn from(err: std::io::Error) -> Self {
        let source = DatabaseInitErrorSource::CouldNotCreatePaths(err);
        BackendError::DatabaseInitError(source)
    }
}

