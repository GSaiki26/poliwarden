// Libs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Structs
#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
    id: String,
    name: String,
    host: String,
    bearer: Option<String>,
    certificate: Option<String>,
    salt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdentityIn {
    id: String,
    name: String,
    host: String,
    bearer: Option<String>,
    certificate: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdentityOut {
    id: String,
    name: String,
    host: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[cfg(feature = "surreal")]
pub mod surreal {
    // Libs
    use super::*;
    use std::str::FromStr;
    use surrealdb::sql::{Datetime, Thing};

    // Structs
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SurrealIdentity {
        id: Thing,
        name: String,
        host: String,
        bearer: Option<String>,
        certificate: Option<String>,
        salt: String,
        created_at: Datetime,
        updated_at: Datetime,
    }

    // Implementations
    impl From<Identity> for SurrealIdentity {
        fn from(identity: Identity) -> Self {
            Self {
                id: Thing::from_str(&identity.id).unwrap(),
                name: identity.name,
                host: identity.host,
                bearer: identity.bearer,
                certificate: identity.certificate,
                salt: identity.salt,
                created_at: Datetime::from(identity.created_at),
                updated_at: Datetime::from(identity.updated_at),
            }
        }
    }
}
