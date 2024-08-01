// libs
use super::Migrations;
use crate::{databases::FileDatabase, errors::DatabaseError, traits::Database};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use tracing::info;

// Data
pub static DATABASE: OnceCell<Arc<RwLock<Box<dyn Database>>>> = OnceCell::const_new();

// Structs
pub struct DatabaseService;

// Implementations
impl DatabaseService {
    pub async fn init_db() -> Result<(), DatabaseError> {
        info!("Initializing the database...");
        let db = DATABASE
            .get_or_init(|| async {
                Arc::new(RwLock::new(DatabaseService::get_enabled_database()))
            })
            .await;

        if db.read().await.is_first_run().await? {
            DatabaseService::run_migrations().await?;
        }

        info!("Database initialized.");
        Ok(())
    }

    fn get_enabled_database() -> Box<dyn Database> {
        info!("FileDB selected.");
        Box::new(FileDatabase::default())
    }

    /**
    A method to be called on the first run of the application.
    */
    async fn run_migrations() -> Result<(), DatabaseError> {
        Migrations::new().run().await
    }
}
