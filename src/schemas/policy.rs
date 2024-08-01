// Libs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

// Structs
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Policy {
    id: Ulid,
    path: String,
    method: String,
    owner_id: String,
    identity_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolicyIn {
    path: String,
    method: String,
    owner_id: String,
    identity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolicyOut {
    id: Ulid,
    path: String,
    method: String,
    owner_id: String,
    identity_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Implementations
impl From<PolicyIn> for Policy {
    fn from(policy: PolicyIn) -> Self {
        let dt = Utc::now();
        Self {
            id: Ulid::new(),
            path: policy.path,
            method: policy.method,
            owner_id: policy.owner_id,
            identity_id: policy.identity_id,
            created_at: dt,
            updated_at: dt,
        }
    }
}

impl PolicyIn {
    pub fn new(path: String, method: String, owner_id: String, identity_id: String) -> Self {
        Self {
            path,
            method,
            owner_id,
            identity_id,
        }
    }
}

impl Policy {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[cfg(feature = "surreal")]
pub mod surreal {
    // Libs
    use super::*;
    use std::str::FromStr;
    use surrealdb::sql::{Datetime, Thing};

    // Structs
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SurrealPolicy {
        id: Thing,
        path: String,
        method: String,
        owner_id: Thing,
        identity_id: Thing,
        created_at: Datetime,
        updated_at: Datetime,
    }

    // Implementations
    impl From<Policy> for SurrealPolicy {
        fn from(policy: Policy) -> Self {
            Self {
                id: Thing::from_str(&policy.id).unwrap(),
                path: policy.path,
                method: policy.method,
                owner_id: Thing::from_str(&policy.owner_id).unwrap(),
                identity_id: Thing::from_str(&policy.identity_id).unwrap(),
                created_at: Datetime::from(policy.created_at),
                updated_at: Datetime::from(policy.updated_at),
            }
        }
    }
}
