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
            Model::Policy(policy) => policy.get_id(),
            Model::Identity(identity) => identity.get_id(),
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

impl From<Policy> for Model {
    fn from(policy: Policy) -> Self {
        Model::Policy(policy)
    }
}

impl From<Identity> for Model {
    fn from(identity: Identity) -> Self {
        Model::Identity(identity)
    }
}
