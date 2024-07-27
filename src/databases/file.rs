// Libs
use crate::{
    errors::DatabaseError,
    traits::{database::Database, Model, ModelProperties, SerdeModel},
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
    fn read_file(&self, table_name: &str) -> Result<Vec<Model>, std::io::Error> {
        let filename = format!("{}{}.json", &self.filepath, table_name);
        let reader = BufReader::new(File::open(filename)?);
        let data: Vec<Model> = serde_json::from_reader(reader)?;
        Ok(data)
    }

    /**
    Write a vector of models to a file.
    */
    fn write_file(&self, table_name: &str, data: Vec<Model>) -> Result<(), std::io::Error> {
        let filename = format!("{}{}.json", &self.filepath, table_name);
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
        return String::from("filedb");
    }

    async fn connect(&self) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn get(&self, table_name: &str, id: &str) -> Result<Option<Model>, DatabaseError> {
        let table: Vec<Model> = self.read_file(table_name)?;
        let model_index = self.get_record_index(&table, id);
        Ok(model_index.map(|index| table[index].clone()))
    }

    async fn get_all(&self, table_name: &str) -> Result<Vec<Model>, DatabaseError> {
        Ok(self.read_file(&table_name)?)
    }

    async fn insert(&self, table_name: &str, data: &Model) -> Result<(), DatabaseError> {
        let mut table: Vec<Model> = self.read_file(&table_name)?;
        table.push(data.clone());
        self.write_file(&table_name, table)?;
        Ok(())
    }

    async fn update(
        &self,
        table_name: &str,
        id: &str,
        data: &Model,
    ) -> Result<Option<()>, DatabaseError> {
        let mut table: Vec<Model> = self.read_file(&table_name)?;
        match self.get_record_index(&table, id) {
            None => return Ok(None),
            Some(record_index) => table[record_index] = data.clone(),
        };
        self.write_file(&table_name, table)?;
        Ok(Some(()))
    }

    async fn delete(&self, table_name: &str, id: &str) -> Result<Option<()>, DatabaseError> {
        let mut table: Vec<Model> = self.read_file(&table_name)?;
        match self.get_record_index(&table, id) {
            None => return Ok(None),
            Some(record_index) => table.swap_remove(record_index),
        };
        self.write_file(&table_name, table)?;
        Ok(Some(()))
    }

    async fn query(&self, _query: &str) -> Result<Vec<Model>, DatabaseError> {
        unimplemented!("Querying is not supported in the file database.");
    }

    async fn is_first_run(&self) -> Result<bool, DatabaseError> {
        let filename = format!("{}identity.json", &self.filepath);
        Ok(!Path::new(&filename).exists())
    }
}
