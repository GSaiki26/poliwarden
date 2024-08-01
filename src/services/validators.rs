use tracing::info;
// Libs
use tracing_subscriber::EnvFilter;
use ulid::Ulid;
use url::Url;

use crate::utils::{get_env, get_optional_env, gracefully_shutdown};

// Structs
/**
Validator for multiples types of data.
It works as a pipe for raw fields, validating them and returning the correct type.
*/
pub struct EnvValidator;

// Implementations
impl EnvValidator {
    /**
    Validate and return the log level.
    */
    pub fn validate_log_level() -> EnvFilter {
        match get_optional_env("POLIWARDEN_LOG_LEVEL") {
            Some(_) => EnvFilter::from_env("POLIWARDEN_LOG_LEVEL"),
            None => EnvFilter::new("INFO"),
        }
    }

    /**
    Validate and return the master bearer.
    */
    pub fn validate_master_bearer() -> String {
        match get_optional_env("POLIWARDEN_MASTER_BEARER") {
            None => {
                let master_bearer = Ulid::new().to_string();
                info!(
                    master_bearer = master_bearer.as_str(),
                    "Generated a new master bearer.",
                );
                master_bearer
            }
            Some(master_bearer) => match Ulid::from_string(&master_bearer) {
                Ok(_) => master_bearer,
                Err(_e) => gracefully_shutdown("The provided master bearer is invalid."),
            },
        }
    }

    /**
    Validate and return the master certificate.
    */
    pub fn validate_master_cert() -> Option<String> {
        match get_optional_env("POLIWARDEN_MASTER_CERT") {
            None => None,
            Some(master_cert) => Some(master_cert),
        }
    }

    /**
    Validate and return the bearer header.
    */
    pub fn validate_bearer_header() -> String {
        match get_optional_env("POLIWARDEN_BEARER_HEADER") {
            None => String::from("Poliwarden-Bearer"),
            Some(bearer_header) => bearer_header,
        }
    }

    /**
    Validate and return the disable bearer.
    */
    pub fn validate_disable_bearer() -> bool {
        match get_optional_env("POLIWARDEN_DISABLE_BEARER") {
            None => false,
            Some(disable_bearer) => match disable_bearer.to_ascii_lowercase().as_str() {
                "true" => true,
                "false" => false,
                _ => gracefully_shutdown("Invalid disable bearer"),
            },
        }
    }

    // db_url: get_env("POLIWARDEN_DB_URL"),
    // db_port: get_env("POLIWARDEN_DB_PORT"),
    // db_username: get_env("POLIWARDEN_DB_USERNAME"),
    // db_password: get_env("POLIWARDEN_DB_PASSWORD"),
    // db_name: get_env("POLIWARDEN_DB_NAME"),
    /**
    Validate and return the database URL.
    */
    pub fn validate_db_url() -> String {
        match Url::parse(&get_env("POLIWARDEN_DB_URL")) {
            Ok(url) => url.to_string(),
            Err(_) => gracefully_shutdown("Invalid database URL."),
        }
    }

    /**
    Validate and return the database port.
    */
    pub fn validate_db_port() -> String {
        match get_env("POLIWARDEN_DB_PORT").parse::<u16>() {
            Ok(_) => get_env("POLIWARDEN_DB_PORT"),
            Err(_) => gracefully_shutdown("Invalid database port."),
        }
    }

    /**
    Validate and return the database username.
    */
    pub fn validate_db_username() -> String {
        get_env("POLIWARDEN_DB_USERNAME")
    }

    /**
    Validate and return the database password.
    */
    pub fn validate_db_password() -> String {
        get_env("POLIWARDEN_DB_PASSWORD")
    }

    /**
    Validate and return the database name.
    */
    pub fn validate_db_name() -> String {
        get_env("POLIWARDEN_DB_NAME")
    }
}
