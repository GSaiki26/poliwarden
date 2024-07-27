// Libs
use super::Database;
use crate::{
    errors::DatabaseError,
    utils::{settings::DatabaseSettings, traits::Model},
};
use async_trait::async_trait;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

// Data
const DATABASE_FILEPATH: &str = "/app/vol/";

// Structs
pub struct FileDatabase {
    filepath: String,
}

// Implementations
impl FileDatabase {
    /**
    Read a file and return the contents as a vector of models.
    */
    fn read_file<T: Model>(&self) -> Result<Vec<T>, std::io::Error> {
        let filename = format!("{}{}.json", &self.filepath, T::get_table_name());
        let reader = BufReader::new(File::open(filename)?);
        let data: Vec<T> = serde_json::from_reader(reader)?;
        Ok(data)
    }

    /**
    Write a vector of models to a file.
    */
    fn write_file<T: Model, U: Into<Vec<T>>>(&self, data: U) -> Result<(), std::io::Error> {
        let filename = format!("{}{}.json", &self.filepath, T::get_table_name());
        let writer = BufWriter::new(File::create(filename)?);
        serde_json::to_writer(writer, &data.into())?;
        Ok(())
    }

    /**
    Get the index of a record in a table by its id.
    */
    fn get_record_index<T: Model, U: AsRef<str> + Send + Sync>(
        &self,
        table: &Vec<T>,
        id: &U,
    ) -> usize {
        table
            .iter()
            .position(|x| x.get_id() == id.as_ref())
            .unwrap()
    }
}

#[async_trait]
impl Database for FileDatabase {
    fn new(_settings: DatabaseSettings) -> Self {
        Self {
            filepath: String::from(DATABASE_FILEPATH),
        }
    }

    fn get_database_name() -> String {
        return String::from("filedb");
    }

    async fn connect(&self) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn migrate<T: Model>(&self) -> Result<(), DatabaseError> {
        let filename = format!("{}{}.json", &self.filepath, T::get_table_name());
        let file = File::create(filename)?;
        let data: Vec<T> = Vec::new();
        serde_json::to_writer(BufWriter::new(file), &data)?;
        Ok(())
    }

    async fn get<T: Model, U: AsRef<str> + Send + Sync>(
        &self,
        id: &U,
    ) -> Result<Option<T>, DatabaseError> {
        let table: Vec<T> = self.read_file()?;
        let result = table.iter().find(|x| x.get_id() == id.as_ref());
        Ok(result.cloned())
    }

    async fn get_all<T: Model>(&self) -> Result<Vec<T>, DatabaseError> {
        let table: Vec<T> = self.read_file()?;
        Ok(table)
    }

    async fn insert<T: Model>(&self, data: &T) -> Result<(), DatabaseError> {
        let mut table: Vec<T> = self.read_file()?;
        table.push(data.clone());
        self.write_file(table)?;
        Ok(())
    }

    async fn update<T: AsRef<str> + Send + Sync, U: Model>(
        &self,
        id: &T,
        data: &U,
    ) -> Result<Option<()>, DatabaseError> {
        let mut table: Vec<U> = self.read_file()?;
        let record_index = self.get_record_index(&table, id);
        table[record_index] = data.clone();
        self.write_file(table)?;
        Ok(Some(()))
    }

    async fn delete<T: Model, U: AsRef<str> + Send + Sync>(
        &self,
        id: &U,
    ) -> Result<(), DatabaseError> {
        let mut table: Vec<T> = self.read_file()?;
        let record_index = self.get_record_index(&table, id);
        table.swap_remove(record_index);
        self.write_file(table)?;
        Ok(())
    }

    async fn query<T: Model, U: AsRef<str> + Send + Sync>(
        &self,
        _query: &U,
    ) -> Result<Vec<T>, DatabaseError> {
        unimplemented!("Querying is not supported in the file database.");
    }

    async fn is_first_run(&self) -> Result<bool, DatabaseError> {
        let filename = format!("{}identity.json", &self.filepath);
        Ok(!Path::new(&filename).exists())
    }
}
