// Libs
use crate::errors::DatabaseError;
use async_trait::async_trait;

use super::Model;

// Traits
#[async_trait]
pub trait Database: Default + Send + Sync {
    /**
    Get the database instance name. Its used to identify the database.
    Its not the database name, but the instance name. Such as SurrealDB, FileDB...
    */
    fn get_database_name(&self) -> String;

    /**
    Connect to the database.
    */
    async fn connect(&self) -> Result<(), DatabaseError>;

    /**
    Get a single record from the database.
    */
    async fn get(&self, table_name: &str, id: &str) -> Result<Option<Model>, DatabaseError>;

    /**
    Get all records from the database.
    */
    async fn get_all(&self, table_name: &str) -> Result<Vec<Model>, DatabaseError>;

    /**
    Insert a record into the database.
    */
    async fn insert(&self, table_name: &str, data: &Model) -> Result<(), DatabaseError>;

    /**
    Update a record in the database.
    */
    async fn update(
        &self,
        table_name: &str,
        id: &str,
        data: &Model,
    ) -> Result<Option<()>, DatabaseError>;

    /**
    Delete a record from the database.
    */
    async fn delete(&self, table_name: &str, id: &str) -> Result<Option<()>, DatabaseError>;

    /**
    Run a query the database.
    */
    async fn query(&self, query: &str) -> Result<Vec<Model>, DatabaseError>;

    /**
    Check if is the first time the database is being used.
    */
    async fn is_first_run(&self) -> Result<bool, DatabaseError>;
}
