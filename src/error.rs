use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("JSON (de)serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("error adding meal: '{0}' already exists")]
    AddError(String),

    #[error("invalid search pattern: '{0}'")]
    InvalidSearchPattern(String),

    #[error("meal '{0}' does not exist")]
    MealDoesNotExist(String),
}

