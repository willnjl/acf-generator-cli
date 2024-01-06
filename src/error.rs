use glob::PatternError;
use std::fmt::{Display, Result};

#[derive(Debug)]
pub enum ALGError {
    FileNotFound(String),
    InvalidJson(serde_json::Error),
    FileAlreadyExists(String),
    IoError(std::io::Error),
    GlobError(PatternError),
}

impl Display for ALGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            ALGError::FileNotFound(path) => write!(f, "{} - {}", "File not found", path),
            ALGError::InvalidJson(e) => write!(f, "{} - {}", "Invalid Json!", e),
            ALGError::FileAlreadyExists(path) => write!(f, "{} - {}", "File Exists", path),
            _ => write!(f, "{}", "Somthing went wrong"),
        }
    }
}

impl From<std::io::Error> for ALGError {
    fn from(err: std::io::Error) -> Self {
        ALGError::IoError(err)
    }
}
