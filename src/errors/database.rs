// Libs
use std::error::Error;
use std::fmt::Display;

// Structs
#[derive(Debug)]
pub struct DatabaseError {
    pub message: String,
}

// Traits
impl Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(e: std::io::Error) -> Self {
        DatabaseError {
            message: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(e: serde_json::Error) -> Self {
        DatabaseError {
            message: e.to_string(),
        }
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
