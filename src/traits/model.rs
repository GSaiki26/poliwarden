// Libs
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::schemas::{Identity, Policy};

// Traits
pub trait ModelProperties: Send + Sync {
    /**
    Get the table name for the model.
    */
    fn get_table_name(&self) -> String;

    /**
    Get the respective SQL migration script for the model, based on the database.
    */
    fn get_migration_script(&self, database_name: &str) -> String;

    /**
    Get the model's id.
    */
    fn get_id(&self) -> String;
}

pub trait SerdeModel: DeserializeOwned + Send + Serialize + Sync {}
