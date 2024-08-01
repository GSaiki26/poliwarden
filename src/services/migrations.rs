// Libs
use super::DATABASE;
use crate::{
    errors::DBResult,
    schemas::{Identity, Model, Policy},
    traits::{Database, ModelProperties},
    utils::{
        get_default_identity_policies, get_default_master_identity, get_default_policy_policies,
        get_default_poliwarden_identity,
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// Structs
pub struct Migrations {
    db: Arc<RwLock<Box<dyn Database>>>,
}

// Implementations
impl Migrations {
    pub fn new() -> Self {
        Self {
            db: DATABASE.get().unwrap().clone(),
        }
    }

    pub async fn run(&self) -> DBResult<()> {
        info!("Running the migration scripts...");

        self.init_tables().await?;
        let (poliw_identity, master_identity) = self.init_identities().await?;
        self.init_policies(&poliw_identity, &master_identity)
            .await?;

        info!("Migration scripts successfully executed.");
        Ok(())
    }

    /**
    Initialize the database tables.
    */
    async fn init_tables(&self) -> DBResult<()> {
        info!("Initializing the database tables...");
        let db_name = self.db.read().await.get_database_name();

        let policy_model: Model = Policy::default().into();
        let identity_model: Model = Identity::default().into();

        let policy_script = policy_model.get_migration_script(&db_name);
        let identity_script = identity_model.get_migration_script(&db_name);

        self.db.write().await.query(&policy_script).await?;
        self.db.write().await.query(&identity_script).await?;

        info!("Database tables initialized.");
        Ok(())
    }

    /**
    Initialize the default identities.
    Returns the PoliWarden and master identities respectively.
    */
    async fn init_identities(&self) -> DBResult<(Model, Model)> {
        info!("Initializing the master identity...");

        let poliw_identity: Model = get_default_poliwarden_identity().into();
        let poliw_table_name = poliw_identity.get_table_name();
        let master_identity: Model = get_default_master_identity()?.into();
        let master_table_name = master_identity.get_table_name();

        let db = self.db.write().await;
        db.insert(&poliw_table_name, &poliw_identity).await?;
        db.insert(&master_table_name, &master_identity).await?;

        info!("Master identity initialized.");
        Ok((poliw_identity, master_identity))
    }

    async fn init_policies(&self, poliw_model: &Model, master_model: &Model) -> DBResult<()> {
        info!("Initializing the master policy...");

        let mut default_policies = get_default_policy_policies(poliw_model, master_model);
        default_policies.extend(get_default_identity_policies(poliw_model, master_model));
        let default_policies: Vec<Model> = default_policies.into_iter().map(|p| p.into()).collect();

        let db = self.db.write().await;
        for policy in default_policies {
            db.insert(&policy.get_table_name(), &policy).await?;
        }

        info!("Master policy initialized.");
        Ok(())
    }
}
