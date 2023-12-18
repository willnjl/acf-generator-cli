use std::fmt::Display;

#[derive(Debug)]
pub enum ALGError {
    InvalidSource,
    InvalidField,
    FileAlreadyExists,
    IoError(std::io::Error),
}

impl Display for ALGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", "Somthing went wrong"),
        }
    }
}

impl From<std::io::Error> for ALGError {
    fn from(err: std::io::Error) -> Self {
        ALGError::IoError(err)
    }
}
