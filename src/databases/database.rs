// Libs
use crate::{
    errors::DatabaseError,
    utils::{settings::DatabaseSettings, traits::Model},
};
use async_trait::async_trait;

// Traits
#[async_trait]
pub trait Database {
    /**
    Create a new instance of the database.
    */
    fn new(settings: DatabaseSettings) -> Self;

    /**
    Get the database instance name. Its used to identify the database.
    Its not the database name, but the instance name. Such as SurrealDB, PostgresDB...
    */
    fn get_database_name() -> String;

    /**
    Connect to the database.
    */
    async fn connect(&self) -> Result<(), DatabaseError>;

    /**
    Run a model migration on the database.
    */
    async fn migrate<T: Model>(&self) -> Result<(), DatabaseError>;

    /**
    Get a single record from the database.
    */
    async fn get<T: Model, U: AsRef<str> + Sync + Send>(
        &self,
        id: &U,
    ) -> Result<Option<T>, DatabaseError>;

    /**
    Get all records from the database.
    */
    async fn get_all<T: Model>(&self) -> Result<Vec<T>, DatabaseError>;

    /**
    Insert a record into the database.
    */
    async fn insert<T: Model>(&self, data: &T) -> Result<(), DatabaseError>;

    /**
    Update a record in the database.
    */
    async fn update<T: AsRef<str> + Sync + Send, U: Model>(
        &self,
        id: &T,
        data: &U,
    ) -> Result<Option<()>, DatabaseError>;

    /**
    Delete a record from the database.
    */
    async fn delete<T: Model, U: AsRef<str> + Sync + Send>(
        &self,
        id: &U,
    ) -> Result<(), DatabaseError>;

    /**
    Run a query the database.
    */
    async fn query<T: Model, U: AsRef<str> + Sync + Send>(
        &self,
        query: &U,
    ) -> Result<Vec<T>, DatabaseError>;

    /**
    Check if is the first time the database is being used.
    */
    async fn is_first_run(&self) -> Result<bool, DatabaseError>;
}
