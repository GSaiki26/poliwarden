// Libs
use crate::{errors::DatabaseError, schemas::DatabaseSettings};
use serde::{de::DeserializeOwned, Serialize};

// Traits
pub trait Database {
    fn new(settings: DatabaseSettings) -> Self;
    fn connect(&self) -> Result<(), DatabaseError>;
    fn query<T: Serialize + DeserializeOwned>(&self, query: &str) -> Result<Vec<T>, DatabaseError>;
}
