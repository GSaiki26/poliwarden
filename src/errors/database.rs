// Libs
use std::error::Error;
use std::fmt::Display;

// Structs
#[derive(Debug)]
pub struct DatabaseError {}

// Traits
impl Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database error")
    }
}

#[cfg(feature = "surreal")]
pub mod surreal {
    use super::*;
    use surrealdb::Error;

    impl From<Error> for DatabaseError {
        fn from(_: Error) -> Self {
            DatabaseError {}
        }
    }
}
