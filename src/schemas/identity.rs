// Libs
use crate::utils::{generate_salt, hash_argon2};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

// Structs
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Identity {
    id: Ulid,
    name: String,
    host: String,
    bearer: Option<String>,
    certificate: Option<String>,
    salt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityIn {
    name: String,
    host: String,
    bearer: Option<String>,
    certificate: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityOut {
    id: Ulid,
    name: String,
    host: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Implementations
impl From<IdentityIn> for Identity {
    fn from(mut identity: IdentityIn) -> Self {
        let dt = Utc::now();
        let salt = generate_salt();
        if let Some(certificate) = &identity.certificate {
            let hash = hash_argon2(&certificate, &salt);
            identity.certificate = Some(hash);
        }

        match &identity.bearer {
            Some(bearer) => {
                let hash = hash_argon2(&bearer, &salt);
                identity.bearer = Some(hash);
            }
            None => {
                let hash = hash_argon2(&Ulid::new().to_string(), &salt);
                identity.bearer = Some(hash);
            }
        }

        Self {
            id: Ulid::new(),
            name: identity.name,
            host: identity.host,
            bearer: identity.bearer,
            certificate: identity.certificate,
            salt: salt.to_string(),
            created_at: dt,
            updated_at: dt,
        }
    }
}

impl From<Identity> for IdentityOut {
    fn from(identity: Identity) -> Self {
        Self {
            id: identity.id,
            name: identity.name,
            host: identity.host,
            created_at: identity.created_at,
            updated_at: identity.updated_at,
        }
    }
}

impl IdentityIn {
    pub fn new(
        name: String,
        host: String,
        bearer: Option<String>,
        certificate: Option<String>,
    ) -> Self {
        Self {
            name,
            host,
            bearer,
            certificate,
        }
    }
}

impl Identity {
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
