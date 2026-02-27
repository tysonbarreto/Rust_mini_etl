use thiserror::Error;
use csv;
use std::io;
use rusqlite;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("CSV error: {0}")]
    Csv (#[from] csv::Error),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("DB error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("Parse error: {0}")]
    Parse(String)
}

pub type AppResult<T>= Result<T,AppError>;