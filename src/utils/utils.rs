// Libs
use super::DEFAULT_SETTINGS;
use crate::{
    schemas::{Identity, IdentityIn, Model, Policy, PolicyIn},
    services::EnvValidator,
    traits::ModelProperties,
};
use regex::Regex;
use std::{env::var, fmt::Display, io::IsTerminal, process::exit};
use tracing::error;
use ulid::DecodeError;

// Function
/**
Gracefully shutdown the application with a message.
It logs the message as an error and exits the process with code 1.
*/
pub fn gracefully_shutdown(message: impl Display) -> ! {
    error!("{}", message);
    exit(1);
}

/**
Define the logger with the default configuration.
*/
pub fn setup_logger() {
    // Check if the terminal is a TTY.
    let stdin = std::io::stdin();
    let log_level = EnvValidator::validate_log_level();
    if stdin.is_terminal() {
        tracing_subscriber::fmt()
            .with_env_filter(log_level)
            .pretty()
            .init();
        return;
    }

    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .json()
        .init();
}

/**
Get the PoliWarden's default identity.
*/
pub fn get_default_poliwarden_identity() -> Identity {
    IdentityIn::new(
        String::from("poliwarden"),
        String::from("poliwarden"),
        None,
        None,
    )
    .into()
}

/**
Get the master's default identity.
*/
pub fn get_default_master_identity() -> Result<Identity, DecodeError> {
    Ok(IdentityIn::new(
        String::from("master"),
        String::from("master"),
        Some(DEFAULT_SETTINGS.bearer_header.clone()),
        DEFAULT_SETTINGS.master_cert.clone(),
    )
    .into())
}

/**
Get the default identity policies.
*/
pub fn get_default_identity_policies(poliw_model: &Model, master_model: &Model) -> Vec<Policy> {
    vec![
        PolicyIn::new(
            Regex::new(r"^\/identities\/?$").unwrap().to_string(),
            Regex::new(r"^(GET|POST)$").unwrap().to_string(),
            poliw_model.get_id(),
            master_model.get_id(),
        )
        .into(),
        PolicyIn::new(
            String::from(r"^\/identities\/\w+\/?$"),
            Regex::new(r"^(GET|PUT|DELETE)$").unwrap().to_string(),
            poliw_model.get_id(),
            master_model.get_id(),
        )
        .into(),
    ]
}

/**
Get the default policy policies.
*/
pub fn get_default_policy_policies(poliw_model: &Model, master_model: &Model) -> Vec<Policy> {
    vec![
        PolicyIn::new(
            Regex::new(r"^\/policies\/?$").unwrap().to_string(),
            Regex::new(r"^(GET|POST)$").unwrap().to_string(),
            poliw_model.get_id(),
            master_model.get_id(),
        )
        .into(),
        PolicyIn::new(
            String::from(r"^\/policies\/\w+\/?$"),
            Regex::new(r"^(GET|PUT|DELETE)$").unwrap().to_string(),
            poliw_model.get_id(),
            master_model.get_id(),
        )
        .into(),
    ]
}

/**
Get the some environment variable. If it is not set, it will gracefully shutdown the application.
*/
pub fn get_env(env_name: &str) -> String {
    match var(env_name) {
        Ok(value) if value.is_empty() => gracefully_shutdown(format!("{env_name} must be set.")),
        Ok(value) => value,
        Err(_) => gracefully_shutdown(format!("{env_name} must be set.")),
    }
}

/**
Get the some environment variable. If it is not set, it will return a None value.

*/
pub fn get_optional_env(env_name: &str) -> Option<String> {
    match var(env_name) {
        Ok(value) if value.is_empty() => None,
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
