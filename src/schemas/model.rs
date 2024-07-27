use serde::{Deserialize, Serialize};

use crate::traits::{ModelProperties, SerdeModel};

use super::{Identity, Policy};

// Enums
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Model {
    Policy(Policy),
    Identity(Identity),
}

impl ModelProperties for Model {
    fn get_table_name(&self) -> String {
        match self {
            Model::Policy(_) => "policy".to_string(),
            Model::Identity(_) => "identity".to_string(),
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
            _ => self.get_table_name(),
        }
    }
}

impl SerdeModel for Model {}
