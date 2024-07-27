// Libs
use services::{init_db, DATABASE};
use tracing::info;
use utils::utils::{gracefully_shutdown, setup_logger};

mod databases;
mod errors;
mod schemas;
mod services;
mod traits;
mod utils;

// Functions
#[tokio::main]
async fn main() {
    setup_logger();

    if let Err(e) = init_db().await {
        gracefully_shutdown(e);
    }
    DATABASE
        .get()
        .unwrap()
        .read()
        .await
        .get("identity", "yes")
        .await
        .unwrap();

    info!("Hello, world!");
}
