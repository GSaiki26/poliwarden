// Libs
use crate::{databases::Database, utils::traits::Model};
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
    id: String,
    name: String,
    host: String,
    bearer: Option<String>,
    certificate: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityOut {
    id: String,
    name: String,
    host: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Implementations
impl Model for Identity {
    fn get_table_name() -> String {
        "identity".to_string()
    }

    fn get_migration_schema<T: Database>() -> String {
        match T::get_database_name().as_str() {
            "filedb" => "".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
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
