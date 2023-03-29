mod mongodb;
use std::borrow::Borrow;

use anyhow::Result;
use bson::{from_document, to_document, Bson, Document};
use serde::{Deserialize, Serialize};

use crate::mongodb::Atlas;
#[derive(Debug)]
pub struct Datastore {
    pub database: Atlas,
    // pub cache: Redis,
}

pub trait HasId {
    fn _id(&self) -> &String;
}

impl Datastore {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        let atlas_connection = Atlas::try_new(db_name).await?;
        Ok(Self {
            database: atlas_connection,
        })
    }

    pub async fn try_create<T>(&self, table: &str, record: T) -> Result<Bson>
    where
        T: Serialize + Borrow<Document>,
    {
        let res = self
            .database
            .try_insert(&self.database, table, record)
            .await?;

        let new_record_id = res.inserted_id;
        println!("{:#?}", new_record_id);

        Ok(new_record_id)
    }

    pub async fn try_create_many<T>(&self, table: &str, records: Vec<T>) -> Result<Vec<String>>
    where
        T: Serialize + Borrow<Document>,
    {
        let res = self
            .database
            .try_insert_many(&self.database, table, records)
            .await?;

        let id_arr = res.inserted_ids.values().map(ToString::to_string).collect();

        print!("{:#?}", id_arr);

        Ok(id_arr)
    }

    pub async fn try_read<T>(&self, table: &str, read_key: String, read_value: String) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let res = self
            .database
            .try_read::<T>(&self.database, table, read_key, read_value)
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

    pub async fn try_update<T>(&self, table: &str, update_record: T) -> Result<T>
    where
        T: Serialize + HasId,
    {
        let update_record_id = update_record._id();

        let update_document = to_document(&update_record).unwrap();

        let _ = self
            .database
            .try_update(&self.database, table, update_record_id, update_document)
            .await?;
        Ok(update_record)
    }

    pub async fn try_delete(&self, table: &str, record_id: String) -> Result<Document> {
        let res = self
            .database
            .try_delete(&self.database, table, record_id)
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod datastore_tests {
    use bson::{doc, to_document};

    use super::*;
    use odds_api::{model::Game, test_data::TestData};

    //Double check if this is valid in order to hardwire _id to generic T
    #[derive(Debug, Serialize)]
    struct User {
        _id: String,
        name: String,
    }

    impl HasId for User {
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
        let table = "users";
        let test_record = doc! {
            "name": "Harry P",
            "author": "JKR"
        };
        let res = data_store.try_create(table, test_record).await.unwrap();
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

        let table = "users";
        let read_key = "_id".to_string();
        let read_value = "9c950da2cbab6a4e71437182846961d4".to_string();

        let read_result = data_store
            .try_read::<Game>(table, read_key, read_value)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_all() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";
        let try_read_all_result = data_store.try_read_all(table).await.unwrap();
        println!("{:#?}", try_read_all_result);
    }

    #[tokio::test]
    async fn test_06_try_update() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";

        let update_user = User {
            _id: "1".to_string(),
            name: "Roman".to_string(),
        };

        let update_result = data_store.try_update(table, update_user).await.unwrap();
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
}
