// Libs
use crate::{errors::DBResult, schemas::Model};
use async_trait::async_trait;

// Traits
#[async_trait]
pub trait Database: Send + Sync {
    /**
    Get the database instance name. Its used to identify the database.
    Its not the database name, but the instance name. Such as SurrealDB, FileDB...
    */
    fn get_database_name(&self) -> String;

    /**
    Connect to the database.
    */
    async fn connect(&self) -> DBResult<()>;

    /**
    Get a single record from the database.
    */
    async fn get(&self, table_name: &str, id: &str) -> DBResult<Option<Model>>;

    /**
    Get all records from the database.
    */
    async fn get_all(&self, table_name: &str) -> DBResult<Vec<Model>>;

    /**
    Insert a record into the database.
    */
    async fn insert(&self, table_name: &str, data: &Model) -> DBResult<()>;

    /**
    Update a record in the database.
    */
    async fn update(&self, table_name: &str, data: &Model) -> DBResult<Option<()>>;

    /**
    Delete a record from the database.
    */
    async fn delete(&self, table_name: &str, id: &str) -> DBResult<Option<()>>;

    /**
    Run a query the database.
    */
    async fn query(&self, query: &str) -> DBResult<Vec<Model>>;

    /**
    Check if is the first time the database is being used.
    */
    async fn is_first_run(&self) -> DBResult<bool>;
}
