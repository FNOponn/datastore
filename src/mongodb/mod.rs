use anyhow::Result;
use bson::{doc, Document};
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    options::InsertManyOptions,
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    Client, Database,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, env};

#[derive(Debug)]
pub struct Atlas {
    pub client: Client,
    pub db: Database,
}

//Do we need a Game ID?
//Can we have shape of id and data(one individual game or anything)?
//And if so, how do we query with above data shape?
//Abstract read to look up via by Game or Mongo ID

impl Atlas {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).await?;

        let db = client.database(db_name);

        Ok(Self { client, db })
    }

    pub async fn try_insert<T>(
        &self,
        database: &Atlas,
        table: &str,
        record: T,
    ) -> Result<InsertOneResult>
    where
        T: Borrow<Document>,
    {
        let collection = database.db.collection::<Document>(table);
        let insert_result = collection.insert_one(record, None).await?;

        Ok(insert_result)
    }

    pub async fn try_insert_many<T>(
        &self,
        database: &Atlas,
        table: &str,
        records: Vec<T>,
    ) -> Result<InsertManyResult>
    where
        T: Sized + Serialize + Borrow<Document>,
    {
        let collection = database.db.collection::<Document>(table);

        let options = InsertManyOptions::builder()
            .bypass_document_validation(true)
            .build();

        let insert_many_result = collection.insert_many(records, options).await?;
        self.try_delete_all(database, table).await;
        Ok(insert_many_result)
    }

    pub async fn try_read<T>(
        &self,
        database: &Atlas,
        table: &str,
        read_key: String,
        read_value: String,
    ) -> Result<Document>
    where
        T: for<'de> Deserialize<'de>,
    {
        let collection = database.db.collection::<Document>(table);

        //Abstract query into a function that returns a Document
        let query = doc! {
            read_key: read_value
        };
        let find_result = collection.find_one(query, None).await?.unwrap();
        Ok(find_result)
    }

    pub async fn try_read_all(&self, database: &Atlas, table: &str) -> Result<Vec<Document>> {
        let table_handle = database.db.collection::<Document>(table);

        let mut cursor = table_handle.find(doc! {}, None).await?;
        let mut read_res = Vec::new();

        while let Some(result) = cursor.next().await {
            if let Ok(document) = result {
                read_res.push(document);
            }
        }

        Ok(read_res)
    }

    pub async fn try_update(
        &self,
        database: &Atlas,
        table: &str,
        record_id: &str,
        update_key: String,
        update_value: String,
    ) -> Result<UpdateResult> {
        let table = database.db.collection::<Document>(table);

        let query = doc! {
            "_id": record_id
        };

        let update = doc! {
                  "$set": { update_key: update_value }
        };

        let update_result = table.update_one(query, update, None).await.unwrap();

        match &update_result.upserted_id {
            Some(upserted_id) => upserted_id.as_str().unwrap(),
            None => record_id,
        };

        Ok(update_result)
    }

    pub async fn try_delete(
        &self,
        database: &Atlas,
        table: &str,
        delete_key: String,
        delete_value: String,
    ) -> Result<Document> {
        let table = database.db.collection::<Document>(table);

        let query = doc! {
            delete_key: delete_value
        };

        let delete_result = table.find_one_and_delete(query, None).await?.unwrap();
        Ok(delete_result)
    }

    pub async fn try_delete_all(&self, database: &Atlas, table: &str) {
        let filter = doc! {};
        let table = database.db.collection::<Document>(table);
        table.delete_many(filter, None).await.unwrap();
    }
}

#[cfg(test)]
mod atlas_tests {
    use bson::to_document;

    use super::*;
    use odds_api::{model::Game, test_data::TestData};

    #[tokio::test]
    async fn test_01_try_new_atlas_struct() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        println!("{:#?}", atlas);
    }

    #[tokio::test]
    async fn test_02_try_insert_one() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";

        let test_data_struct = TestData::new();
        let data = test_data_struct.data_1;

        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let game = &outcomes[0];
        let res = atlas.try_insert(&atlas, table, game).await.unwrap();
        print!("{:#?}", res);

        let delete_key = "_id".to_string();
        let delete_value = "e40d079e6db5293e7e0aa22e0c857a85".to_string();

        let _ = atlas
            .try_delete(&atlas, table, delete_key, delete_value)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_03_try_insert_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let test_data_struct = TestData::new();
        let table = "users";
        let data = test_data_struct.data_1;
        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();
        atlas
            .try_insert_many(&atlas, table, outcomes)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_04_try_read() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";
        let read_key = "_id".to_string();
        let read_value = "9c950da2cbab6a4e71437182846961d4".to_string();

        let read_result = atlas
            .try_read::<Game>(&atlas, table, read_key, read_value)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_all() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";

        let read_result = atlas.try_read_all(&atlas, table).await.unwrap();
        println!("{:#?}", read_result)
    }

    #[tokio::test]
    async fn test_06_try_update() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table = "users";
        let record_id = &"56420b74c402bfccb04db2542d901054".to_string();
        let update_key = "sport_title".to_string();
        let update_value = "NFL".to_string();

        let update_result = atlas
            .try_update(&atlas, table, record_id, update_key, update_value)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_07_try_delete() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";

        let test_data_struct = TestData::new();
        let data = test_data_struct.data_1;

        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let game = &outcomes[0];

        let _ = atlas.try_insert(&atlas, table, game).await.unwrap();

        let delete_key = "_id".to_string();
        let delete_value = "e40d079e6db5293e7e0aa22e0c857a85".to_string();

        let delete_result = atlas
            .try_delete(&atlas, table, delete_key, delete_value)
            .await
            .unwrap();
        println!("{:#?}", delete_result);
    }

    #[tokio::test]
    async fn test_08_try_delete_all() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";
        let delete_all_result = atlas.try_delete_all(&atlas, table).await;
    }
}
