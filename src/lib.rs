mod mongodb;
use std::borrow::Borrow;

use anyhow::Result;
use bson::{from_document, Document};
use serde::{Deserialize, Serialize};

use crate::mongodb::Atlas;
#[derive(Debug)]
pub struct Datastore {
    pub database: Atlas,
    // pub cache: Redis,
}

impl Datastore {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        let atlas_connection = Atlas::try_new(db_name).await?;
        Ok(Self {
            database: atlas_connection,
        })
    }

    pub async fn try_create<T>(&self, table: &str, records: Vec<T>) -> Result<Vec<String>>
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

    pub async fn try_update(
        &self,
        table: &str,
        record_id: String,
        update_key: String,
        update_value: String,
    ) -> Result<String> {
        let res = self
            .database
            .try_update(&self.database, table, &record_id, update_key, update_value)
            .await?;
        Ok(record_id)
        //Look more into upserted
    }

    pub async fn try_delete(
        &self,
        collection_name: &str,
        delete_key: String,
        delete_value: String,
    ) -> Result<()> {
        self.database
            .try_delete_many(&self.database, collection_name, delete_key, delete_value)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod datastore_tests {
    use bson::to_document;

    use super::*;
    use odds_api::{model::Game, test_data::TestData};

    #[tokio::test]
    async fn test_01_try_new_data_store_struct() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        println!("{:#?}", data_store);
    }

    #[tokio::test]
    async fn test_02_try_create() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let test_data_struct = TestData::new();
        let collection_name = "users";
        let data = test_data_struct.data_1;
        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        data_store
            .try_create(collection_name, outcomes)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_03_try_read() {
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
    async fn test_04_try_read_all() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";
        let try_read_all_result = data_store.try_read_all(table).await.unwrap();
        println!("{:#?}", try_read_all_result);
    }

    #[tokio::test]
    async fn test_04_try_update() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";
        let query_id = "e40d079e6db5293e7e0aa22e0c857a85".to_string();
        let update_key = "sport_title".to_string();
        let update_value = "AFL".to_string();
        data_store
            .try_update(table, query_id, update_key, update_value)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_05_try_delete() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let collection_name = "users";
        let delete_key = "home_team".to_string();
        let delete_value = "Houston Rockets".to_string();
        data_store
            .try_delete(collection_name, delete_key, delete_value)
            .await
            .unwrap();
    }
}
