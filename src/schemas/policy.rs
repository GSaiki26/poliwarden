// Libs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Policy {
    pub id: String,
    pub path: String,
    pub method: String,
    pub owner_id: String,
    pub identity_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolicyIn {
    pub path: String,
    pub method: String,
    pub owner_id: String,
    pub identity_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolicyOut {
    pub id: String,
    pub path: String,
    pub method: String,
    pub owner_id: String,
    pub identity_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
