// Libs
use services::DatabaseService;
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

    if let Err(e) = DatabaseService::init_db().await {
        gracefully_shutdown(e);
    }

    info!("Hello, world!");
}
