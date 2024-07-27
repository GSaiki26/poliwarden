// Libs
use serde::{de::DeserializeOwned, Serialize};

use crate::databases::Database;

// Traits
pub trait Model: Clone + DeserializeOwned + Send + Serialize + Sync {
    /**
    Get the table name for the model.
    */
    fn get_table_name() -> String;

    /**
    Get the model schema.
    It receives a Database as generic type to be able to return the correct SQL syntax for the database.
    */
    fn get_migration_schema<T: Database>() -> String;

    /**
    Get the model's id.
    */
    fn get_id(&self) -> String;
}
