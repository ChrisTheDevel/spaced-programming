use std::fmt::Display;
use std::error::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError  {
    DatabaseError(DatabaseErrorSource),
    SqliteError(SQLError),
}

#[derive(Debug)]
pub enum DatabaseErrorSource {
    IOError(std::io::Error),
    InvalidSchemaVersion(String),
}

impl Display for DatabaseErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            DatabaseErrorSource::IOError(err) => format!("{err}"),
            DatabaseErrorSource::InvalidSchemaVersion(str) => format!("Invalid schema: {str}"),
        };
        write!(f, "{message}")

    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_error_message = match self {
            AppError::DatabaseError(err) => format!("DatabaseInitError: {err}"),
            AppError::SqliteError(err) => format!("SqliteError ==== \n\t{}", err),
        };
        write!(f, "{full_error_message}")
    }
}

use rusqlite::Error as SQLError;
impl From<SQLError> for AppError {
    fn from(e: SQLError) -> Self {
        AppError::SqliteError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        let source = DatabaseErrorSource::IOError(err);
        AppError::DatabaseError(source)
    }
}

