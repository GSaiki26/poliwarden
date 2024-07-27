// libs
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use tracing::info;

use crate::{
    databases::FileDatabase,
    errors::DatabaseError,
    schemas::{Identity, Model, Policy},
    traits::{Database, ModelProperties},
};

// Data
pub static DATABASE: OnceCell<Arc<RwLock<Box<dyn Database>>>> = OnceCell::const_new();

// Functions
pub async fn init_db() -> Result<(), DatabaseError> {
    info!("Initializing the database...");
    let db = DATABASE
        .get_or_init(|| async { Arc::new(RwLock::new(get_enabled_database())) })
        .await;

    if db.read().await.is_first_run().await? {
        run_migrations().await?;
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
    info!("Running the migration scripts...");
    let db = DATABASE.get().unwrap();
    let db_name = db.read().await.get_database_name();

    // Run the models migration scripts.
    let identity_script = Model::Identity(Identity::default()).get_migration_script(&db_name);
    let policy_script = Model::Policy(Policy::default()).get_migration_script(&db_name);

    db.write().await.query(&identity_script).await?;
    db.write().await.query(&policy_script).await?;

    info!("Migration scripts successfully executed.");
    Ok(())
}
