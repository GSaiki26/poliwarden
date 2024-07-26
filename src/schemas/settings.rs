// Libs
use once_cell::sync::Lazy;
use std::env::var;

// Data
pub const DEFAULT_SETTINGS: Lazy<DefaultSettings> = Lazy::new(|| DefaultSettings {
    log_level: var("POLIWARDEN_LOG_LEVEL").expect("POLIWARDEN_LOG_LEVEL must be set."),
    master_bearer: var("POLIWARDEN_MASTER_BEARER").expect("POLIWARDEN_LOG_LEVEL must be set."),
    master_cert: var("POLIWARDEN_MASTER_CERT").expect("POLIWARDEN_LOG_LEVEL must be set."),
    bearer_header: var("POLIWARDEN_BEARER_HEADER").expect("POLIWARDEN_LOG_LEVEL must be set."),
    disable_bearer: var("POLIWARDEN_DISABLE_BEARER").expect("POLIWARDEN_LOG_LEVEL must be set."),
    secret_key: var("POLIWARDEN_SECRET_KEY").expect("POLIWARDEN_LOG_LEVEL must be set."),
});

pub const DATABASE_SETTINGS: Lazy<DatabaseSettings> = Lazy::new(|| DatabaseSettings {
    db_url: var("POLIWARDEN_DB_URL").expect("POLIWARDEN_DB_URL must be set."),
    db_port: var("POLIWARDEN_DB_PORT").expect("POLIWARDEN_DB_PORT must be set."),
    db_username: var("POLIWARDEN_DB_USERNAME").expect("POLIWARDEN_DB_USERNAME must be set."),
    db_password: var("POLIWARDEN_DB_PASSWORD").expect("POLIWARDEN_DB_PASSWORD must be set."),
    db_name: var("POLIWARDEN_DB_NAME").expect("POLIWARDEN_DB_NAME must be set."),
});

pub const HTTP_SETTINGS: Lazy<HttpSettings> = Lazy::new(|| HttpSettings {
    feature_http_port: var("POLIWARDEN_FEATURE_HTTP_PORT")
        .expect("POLIWARDEN_FEATURE_HTTP_PORT must be set."),
});

pub const HTTPS_SETTINGS: Lazy<HttpsSettings> = Lazy::new(|| HttpsSettings {
    feature_https_port: var("POLIWARDEN_FEATURE_HTTPS_PORT")
        .expect("POLIWARDEN_FEATURE_HTTPS_PORT must be set."),
    feature_https_cert: var("POLIWARDEN_FEATURE_HTTPS_CERT")
        .expect("POLIWARDEN_FEATURE_HTTPS_CERT must be set."),
    feature_https_key: var("POLIWARDEN_FEATURE_HTTPS_KEY")
        .expect("POLIWARDEN_FEATURE_HTTPS_KEY must be set."),
});

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

pub struct HttpSettings {
    feature_http_port: String,
}

pub struct HttpsSettings {
    feature_https_port: String,
    feature_https_cert: String,
    feature_https_key: String,
}
