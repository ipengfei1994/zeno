use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZenoError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}