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

// Enums
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    Policy(Policy),
    Identity(Identity),
}

impl ModelProperties for Model {
    fn get_table_name(&self) -> String {
        match self {
            Model::Policy(_) => "policies".to_string(),
            Model::Identity(_) => "identities".to_string(),
        }
    }

    fn get_id(&self) -> String {
        match self {
            Model::Policy(policy) => policy.id.clone(),
            Model::Identity(identity) => identity.id.clone(),
        }
    }

    fn get_migration_script(&self, database_name: &str) -> String {
        // let filedb_script = format!("./migrations/{}_filedb.sql", self.get_table_name());
        match database_name {
            _ => String::new(),
        }
    }
}

impl SerdeModel for Model {}
