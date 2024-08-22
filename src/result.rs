use std::env::VarError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct VantageError {
    pub msg: String,
}

impl VantageError {
    pub fn new(msg: &str) -> Self {
        VantageError {
            msg: String::from(msg),
        }
    }
}

impl Display for VantageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for VantageError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<VarError> for VantageError {
    fn from(value: VarError) -> Self {
        VantageError {
            msg: format!("Failed to read environment variable: {}", value),
        }
    }
}

impl From<reqwest::Error> for VantageError {
    fn from(value: reqwest::Error) -> Self {
        VantageError {
            msg: format!("Network error: {}", value),
        }
    }
}

impl From<serde_json::Error> for VantageError {
    fn from(value: serde_json::Error) -> Self {
        VantageError {
            msg: format!("Failed to parse JSON: {}", value),
        }
    }
}

impl From<String> for VantageError {
    fn from(value: String) -> Self {
        VantageError { msg: value }
    }
}

pub type VantageResult<T> = Result<T, VantageError>;
