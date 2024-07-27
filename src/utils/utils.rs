// Libs
use std::{fmt::Display, io::IsTerminal, process::exit};
use tracing::error;
use tracing_subscriber::EnvFilter;

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
    if stdin.is_terminal() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();
        return;
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
}
