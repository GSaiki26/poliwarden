// Libs
use std::error::Error;
use std::fmt::Display;

use ulid::DecodeError;

// Types
pub type DBResult<T> = std::result::Result<T, DatabaseError>;

// Enums
#[derive(Debug)]
pub enum DatabaseError {
    Io(std::io::Error),
    Json(serde_json::Error),

    InvalidField(String),

    #[cfg(feature = "surreal")]
    Surreal(surrealdb::Error),
}

// Traits
impl Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::Io(e) => write!(f, "IO error: {}", e),
            DatabaseError::Json(e) => write!(f, "JSON error: {}", e),
            DatabaseError::InvalidField(e) => write!(f, "Invalid field: {}", e),
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(e: std::io::Error) -> Self {
        DatabaseError::Io(e)
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(e: serde_json::Error) -> Self {
        DatabaseError::Json(e)
    }
}

impl From<DecodeError> for DatabaseError {
    fn from(_e: DecodeError) -> Self {
        DatabaseError::InvalidField(String::from("Provided ULID is invalid."))
    }
}

#[cfg(feature = "surreal")]
mod surreal {
    use super::*;
    use surrealdb::Error;

    impl From<Error> for DatabaseError {
        fn from(e: Error) -> Self {
            DatabaseError {
                message: e.to_string(),
            }
        }
    }
}
