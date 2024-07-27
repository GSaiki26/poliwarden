// Libs
use crate::utils::utils::gracefully_shutdown;
use once_cell::sync::Lazy;
use std::{env::var, sync::Arc};

// Data
pub const DEFAULT_SETTINGS: Lazy<Arc<DefaultSettings>> = Lazy::new(|| {
    Arc::new(DefaultSettings {
        log_level: get_env("POLIWARDEN_LOG_LEVEL"),
        master_bearer: get_env("POLIWARDEN_MASTER_BEARER"),
        master_cert: get_env("POLIWARDEN_MASTER_CERT"),
        bearer_header: get_env("POLIWARDEN_BEARER_HEADER"),
        disable_bearer: get_env("POLIWARDEN_DISABLE_BEARER"),
        secret_key: get_env("POLIWARDEN_SECRET_KEY"),
    })
});
pub const DATABASE_SETTINGS: Lazy<Arc<DatabaseSettings>> = Lazy::new(|| {
    Arc::new(DatabaseSettings {
        db_url: get_env("POLIWARDEN_DB_URL"),
        db_port: get_env("POLIWARDEN_DB_PORT"),
        db_username: get_env("POLIWARDEN_DB_USERNAME"),
        db_password: get_env("POLIWARDEN_DB_PASSWORD"),
        db_name: get_env("POLIWARDEN_DB_NAME"),
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
    log_level: String,
    master_bearer: String,
    master_cert: String,
    bearer_header: String,
    disable_bearer: String,
    secret_key: String,
}

pub struct DatabaseSettings {
    db_url: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_name: String,
}

// pub struct HttpSettings {
//     feature_http_port: String,
// }

// pub struct HttpsSettings {
//     feature_https_port: String,
//     feature_https_cert: String,
//     feature_https_key: String,
// }

// Functions
fn get_env(env_name: &str) -> String {
    match var(env_name) {
        Ok(value) => value,
        Err(_) => gracefully_shutdown(format!("{env_name} must be set.")),
    }
}
