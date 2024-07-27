// Libs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub host: String,
    pub bearer: Option<String>,
    pub certificate: Option<String>,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityIn {
    pub id: String,
    pub name: String,
    pub host: String,
    pub bearer: Option<String>,
    pub certificate: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityOut {
    pub id: String,
    pub name: String,
    pub host: String,
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
