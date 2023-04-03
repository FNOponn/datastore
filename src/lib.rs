mod mongodb;
use std::{borrow::Borrow, collections::HashMap};

use anyhow::Result;
use bson::{from_document, to_document, Document};
use serde::{Deserialize, Serialize};

use crate::mongodb::Atlas;

pub struct Test {
    name: String,
}
#[derive(Debug)]
pub struct Datastore {
    pub database: Atlas,
    // pub cache: Redis,
}

pub trait MongoStorable {
    fn _id(&self) -> &String;
}

impl Datastore {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        let atlas_connection = Atlas::try_new(db_name).await?;
        Ok(Self {
            database: atlas_connection,
        })
    }

    pub async fn try_create<T>(&self, table: &str, record: T) -> Result<T>
    where
        T: Serialize + Borrow<Document> + Clone,
    {
        let res = self
            .database
            .try_insert_one(&self.database, table, record.clone())
            .await?;

        let new_record_id = res.inserted_id;
        println!("{:#?}", new_record_id);

        Ok(record)
    }

    pub async fn try_create_many<T>(&self, table: &str, records: Vec<T>) -> Result<Vec<T>>
    where
        T: Serialize + Borrow<Document> + Clone,
    {
        let new_records = records.clone();

        let _ = self
            .database
            .try_insert_many(&self.database, table, records)
            .await?;

        // let id_arr = res.inserted_ids.values().map(ToString::to_string).collect();

        // print!("{:#?}", id_arr);

        Ok(new_records)
    }

    pub async fn try_read<T>(&self, table: &str, record_id: String) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let res = self
            .database
            .try_read_one::<T>(&self.database, table, record_id)
            .await
            .unwrap();
        let read_result = from_document::<T>(res).unwrap();
        Ok(read_result)
    }

    pub async fn try_read_all(&self, table: &str) -> Result<Vec<Document>> {
        let res = self
            .database
            .try_read_all(&self.database, table)
            .await
            .unwrap();
        Ok(res)
    }

    pub async fn try_update_one<T>(&self, table: &str, update_record: T) -> Result<T>
    where
        T: Serialize + MongoStorable,
    {
        let update_record_id = update_record._id();

        let update_document = to_document(&update_record).unwrap();

        let _ = self
            .database
            .try_update_one(&self.database, table, update_record_id, update_document)
            .await?;
        Ok(update_record)
    }

    pub async fn try_update_many(
        &self,
        table: &str,
        update_map: HashMap<&str, Document>,
    ) -> Result<Vec<Document>> {
        let response = self
            .database
            .try_update_many(&self.database, table, update_map)
            .await?;
        Ok(response)
    }

    //Double confirm Delete input. ID vs whole record?
    //Confirm return nothing or bool?
    pub async fn try_delete(&self, table: &str, record_id: String) -> Result<()> {
        let res = self
            .database
            .try_delete_one(&self.database, table, record_id)
            .await?;
        Ok(())
    }

    pub async fn try_delete_many(&self, table: &str, delete_ids: Vec<String>) -> Result<()> {
        let res = self
            .database
            .try_delete_many(&self.database, table, delete_ids)
            .await?;
        Ok(())
    }

    pub async fn try_read_documents_by_ids(
        &self,
        table: &str,
        ids: Vec<String>,
    ) -> Result<Vec<Document>> {
        let res = self
            .database
            .try_read_documents_by_ids(&self.database, table, ids)
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod datastore_tests {
    use bson::{doc, to_document};

    use super::*;
    use odds_api::{model::Game, test_data::TestData};

    #[derive(Debug, Deserialize, Serialize)]
    struct Book {
        name: String,
        author: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BookRecord {
        _id: String,
        data: Book,
    }

    impl MongoStorable for BookRecord {
        fn _id(&self) -> &String {
            &self._id
        }
    }

    #[tokio::test]
    async fn test_01_try_new_data_store_struct() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        println!("{:#?}", data_store);
    }

    #[tokio::test]
    async fn test_02_try_create_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books";
        let new_record = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "Red Wall",
                "author": "Brian Jacques"
            }
        };
        let res = data_store.try_create(table, new_record).await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_create_many() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let test_data_struct = TestData::new();
        let table = "users";
        let data = test_data_struct.data_1;
        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        data_store.try_create_many(table, outcomes).await.unwrap();
    }

    #[tokio::test]
    async fn test_04_try_read() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";
        let record_id = "7e9dd50fdb0b2d895dffc78a239997f7".to_string();

        let read_result = data_store
            .try_read::<Document>(table, record_id)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_all() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";
        let try_read_all_result = data_store.try_read_all(table).await.unwrap();
        println!("{:#?}", try_read_all_result);
    }

    #[tokio::test]
    async fn test_06_try_update_one() {
        //Double Check
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";

        let update_record = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "Red Wall",
                "author": "Brian Jacques"
            }
        };

        let update_record = from_document::<BookRecord>(update_record).unwrap();

        let update_result = data_store
            .try_update_one(table, update_record)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_06_try_update_many() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let mut update_map = HashMap::new();

        let table = "books";

        let update_record1 = doc! {
            "_id": "0aa7ba9d4ef9dfacd6c1d4e545b86e87"
        ,
            "data": {
                "name": "Dagon",
                "author": "HP Lovecraft"
            }
        };

        let update_record2 = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "The Long Patrol",
                "author": "Brian Jacques"
            }
        };

        update_map.insert("0aa7ba9d4ef9dfacd6c1d4e545b86e87", update_record1);
        update_map.insert("7e9dd50fdb0b2d895dffc78a239997f7", update_record2);

        let update_result = data_store.try_update_many(table, update_map).await.unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_07_try_delete() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let collection_name = "users";
        let record_id = "6423b7d647a9690332556b41".to_string();

        let res = data_store
            .try_delete(collection_name, record_id)
            .await
            .unwrap();

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_08_find_documents_by_ids() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";

        let ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let update_result = data_store
            .try_read_documents_by_ids(table, ids)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }
}
