// Libs
use crate::{
    errors::DatabaseError,
    schemas::{Identity, Model},
    traits::{database::Database, ModelProperties},
};
use async_trait::async_trait;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};
use tracing::{debug, info, Level};

// Data
const DATABASE_FILEPATH: &str = "./vol/";

// Structs
pub struct FileDatabase {
    filepath: String,
}

// Implementations
impl FileDatabase {
    /**
    Read a file and return the contents as a vector of models.
    */
    fn read_file(&self, table_name: &str) -> Result<Vec<Model>, std::io::Error> {
        let filename = self.get_table_path(table_name);
        let reader = BufReader::new(File::open(filename)?);
        let data: Vec<Model> = serde_json::from_reader(reader)?;
        Ok(data)
    }

    /**
    Write a vector of models to a file.
    */
    fn write_file(&self, table_name: &str, data: Vec<Model>) -> Result<(), std::io::Error> {
        let filename = self.get_table_path(table_name);
        let writer = BufWriter::new(File::create(filename)?);
        serde_json::to_writer(writer, &data)?;
        Ok(())
    }

    /**
    Get the index of a record in a table by its id.
    */
    fn get_record_index(&self, table: &Vec<Model>, id: &str) -> Option<usize> {
        table.iter().position(|x| x.get_id() == id.as_ref())
    }

    /**
    Get the tablepath.
    */
    fn get_table_path(&self, table_name: &str) -> String {
        format!("{}{}.json", &self.filepath, table_name)
    }

    /**
    Create the database directory.
    */
    fn create_database_directory(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.filepath)
    }
}

impl Default for FileDatabase {
    fn default() -> Self {
        Self {
            filepath: String::from(DATABASE_FILEPATH),
        }
    }
}

#[async_trait]
impl Database for FileDatabase {
    fn get_database_name(&self) -> String {
        debug!("get_database_name() called.");
        return String::from("filedb");
    }

    async fn connect(&self) -> Result<(), DatabaseError> {
        debug!("connect() called.");
        Ok(())
    }

    async fn get(&self, table_name: &str, id: &str) -> Result<Option<Model>, DatabaseError> {
        let span = tracing::span!(Level::INFO, "", table = table_name, id = id);
        let _enter = span.enter();
        info!("Getting model by id...");

        let table: Vec<Model> = self.read_file(table_name)?;
        let model_index = self.get_record_index(&table, id);
        let result = model_index.map(|index| table[index].clone());

        info!(result = ?result, "Got model by id.");
        Ok(result)
    }

    async fn get_all(&self, table_name: &str) -> Result<Vec<Model>, DatabaseError> {
        let span = tracing::span!(Level::INFO, "", table = table_name);
        let _enter = span.enter();
        info!("Getting all models...");

        let result = self.read_file(&table_name)?;

        info!(result_size = result.len(), "Got all models.");
        Ok(result)
    }

    async fn insert(&self, table_name: &str, data: &Model) -> Result<(), DatabaseError> {
        let span = tracing::span!(Level::INFO, "", table = table_name);
        let _enter = span.enter();
        info!("Inserting model...");

        let mut table: Vec<Model> = self.read_file(&table_name)?;
        table.push(data.clone());
        self.write_file(&table_name, table)?;

        info!("Inserted model.");
        Ok(())
    }

    async fn update(&self, table_name: &str, data: &Model) -> Result<Option<()>, DatabaseError> {
        let span = tracing::span!(Level::INFO, "", table = table_name, id = data.get_id());
        let _enter = span.enter();
        info!("Updating model by id...");

        let mut table: Vec<Model> = self.read_file(&table_name)?;
        match self.get_record_index(&table, &data.get_id()) {
            None => return Ok(None),
            Some(record_index) => table[record_index] = data.clone(),
        };
        self.write_file(&table_name, table)?;

        info!("Updated model by id.");
        Ok(Some(()))
    }

    async fn delete(&self, table_name: &str, id: &str) -> Result<Option<()>, DatabaseError> {
        let span = tracing::span!(Level::INFO, "", table = table_name, id = id);
        let _enter = span.enter();
        info!("Deleting model by id...");

        let mut table: Vec<Model> = self.read_file(&table_name)?;
        match self.get_record_index(&table, id) {
            None => return Ok(None),
            Some(record_index) => table.swap_remove(record_index),
        };
        self.write_file(&table_name, table)?;

        info!("Deleted model by id.");
        Ok(Some(()))
    }

    async fn query(&self, query: &str) -> Result<Vec<Model>, DatabaseError> {
        // The table_name'll be passed as a query.

        self.create_database_directory()?;
        let table_path = self.get_table_path(query);
        if Path::new(&table_path).exists() {
            unreachable!("Bad logic. Query must not be called beyond migrations.");
        }

        self.write_file(query, Vec::new())?;

        Ok(vec![])
    }

    async fn is_first_run(&self) -> Result<bool, DatabaseError> {
        info!("Checking if it's the application's first run...");

        let identity_model = Model::Identity(Identity::default());
        let table_path = self.get_table_path(&identity_model.get_table_name());
        let result = !Path::new(&table_path).exists();

        info!(
            table_path = &table_path,
            is_first_run = result,
            "First run checked."
        );
        Ok(result)
    }
}
