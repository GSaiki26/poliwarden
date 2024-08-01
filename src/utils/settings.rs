// Libs
use crate::services::EnvValidator;
use once_cell::sync::Lazy;
use std::sync::Arc;

// Data
pub static DEFAULT_SETTINGS: Lazy<Arc<DefaultSettings>> = Lazy::new(|| {
    Arc::new(DefaultSettings {
        master_bearer: EnvValidator::validate_master_bearer(),
        master_cert: EnvValidator::validate_master_cert(),
        bearer_header: EnvValidator::validate_bearer_header(),
        disable_bearer: EnvValidator::validate_disable_bearer(),
    })
});

pub static DATABASE_SETTINGS: Lazy<Arc<DatabaseSettings>> = Lazy::new(|| {
    Arc::new(DatabaseSettings {
        db_url: EnvValidator::validate_db_url(),
        db_port: EnvValidator::validate_db_port(),
        db_username: EnvValidator::validate_db_username(),
        db_password: EnvValidator::validate_db_password(),
        db_name: EnvValidator::validate_db_name(),
    })
});

// pub const HTTP_SETTINGS: Lazy<Arc<HttpSettings>> = Lazy::new(|| {
//     Arc::new(HttpSettings {
//         feature_http_port: get_env("POLIWARDEN_FEATURE_HTTP_PORT"),
//     })
// });

// pub const HTTPS_SETTINGS: Lazy<Arc<HttpsSettings>> = Lazy::new(|| {
//     Arc::new(HttpsSettings {
//         feature_https_port: get_env("POLIWARDEN_FEATURE_HTTPS_PORT"),
//         feature_https_cert: get_env("POLIWARDEN_FEATURE_HTTPS_CERT"),
//         feature_https_key: get_env("POLIWARDEN_FEATURE_HTTPS_KEY"),
//     })
// });

// Structs
pub struct DefaultSettings {
    pub master_bearer: String,
    pub master_cert: Option<String>,
    pub bearer_header: String,
    pub disable_bearer: bool,
}

pub struct DatabaseSettings {
    pub db_url: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}

// pub struct HttpSettings {
//     feature_http_port: String,
// }

// pub struct HttpsSettings {
//     feature_https_port: String,
//     feature_https_cert: String,
//     feature_https_key: String,
// }
