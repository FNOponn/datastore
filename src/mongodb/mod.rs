use anyhow::Result;
use bson::{doc, Document};
use dotenv::dotenv;
use futures_util::stream::StreamExt;
use mongodb::{
    options::InsertManyOptions,
    results::{DeleteResult, InsertManyResult, InsertOneResult},
    Client, Database,
};

use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, env};

#[derive(Debug)]
pub struct Atlas {
    pub client: Client,
    pub db: Database,
}

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
        Ok(insert_many_result)
    }

    pub async fn try_read<T>(
        &self,
        database: &Atlas,
        table: &str,
        read_key: String, //Hard code to _id?
        read_value: String,
    ) -> Result<Document>
    where
        T: for<'de> Deserialize<'de>,
    {
        let collection = database.db.collection::<Document>(table);

        let query = doc! {
            read_key: read_value
        };
        let find_result = collection.find_one(query, None).await?.unwrap();
        Ok(find_result)
    }
    pub async fn try_read_documents_by_ids(
        &self,
        database: &Atlas,
        table: &str,
        ids: Vec<String>,
    ) -> Result<Vec<Document>> {
        let table = database.db.collection::<Document>(table);

        let filter = doc! { "_id": { "$in": ids } };
        let mut cursor = table.find(filter, None).await?;
        let mut documents = Vec::new();

        while let Some(result) = cursor.next().await {
            if let Ok(document) = result {
                documents.push(document);
            }
        }
        Ok(documents)
    }

    //Transaction-rize? Atomicity vs one by one?
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

    pub async fn try_update_one(
        &self,
        database: &Atlas,
        table: &str,
        update_record_id: &String,
        updated_record: Document,
    ) -> Result<Document> {
        let table = database.db.collection::<Document>(table);

        let query = doc! {
            "_id": update_record_id
        };

        let update = doc! {
            "$set": &updated_record
        };

        let update_result = table.update_one(query, update, None).await.unwrap();

        match &update_result.upserted_id {
            Some(upserted_id) => upserted_id.as_str().unwrap(),
            None => update_record_id,
        };

        Ok(updated_record)
    }

    //See above comment
    pub async fn try_update_many(
        &self,
        database: &Atlas,
        table: &str,
        update_map: HashMap<&str, Document>,
    ) -> Result<Vec<Document>> {
        let table = database.db.collection::<Document>(table);

        let mut updated_records: Vec<Document> = Vec::new();

        for (record_id, document) in update_map {
            let filter = doc! { "_id": record_id };
            let ref_doc = &document;
            let update_doc = doc! { "$set": ref_doc };
            updated_records.push(document);
            table.update_one(filter, update_doc, None).await?;
        }
        Ok(updated_records)
    }

    pub async fn try_delete(
        &self,
        database: &Atlas,
        table: &str,
        record_id: String,
    ) -> Result<Document> {
        let table = database.db.collection::<Document>(table);

        let query = doc! {
            "_id": record_id
        };

        let delete_result = table.find_one_and_delete(query, None).await?.unwrap();
        Ok(delete_result)
    }

    pub async fn try_delete_many(
        &self,
        database: &Atlas,
        table: &str,
        delete_ids: Vec<String>,
    ) -> Result<()> {
        let table = database.db.collection::<Document>(table);

        let filter = doc! {
            "_id": { "$in": delete_ids },
        };
        let res = table.delete_many(filter, None).await?;
        println!("{:#?}", res);
        Ok(())
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
    // use book::BookRecord;
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
        let table = "books";

        let test_data_struct = TestData::new();
        let data = test_data_struct.data_3;

        let records = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let insert_record = &records[0];

        let res = atlas
            .try_insert(&atlas, table, insert_record)
            .await
            .unwrap();
        print!("{:#?}", res);

        let record_id = "e40d079e6db5293e7e0aa22e0c857a85".to_string();

        let _ = atlas.try_delete(&atlas, table, record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_insert_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let test_data_struct = TestData::new();
        let table = "books";

        let data = test_data_struct.data_3;
        let outcomes = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let res = atlas
            .try_insert_many(&atlas, table, outcomes)
            .await
            .unwrap();
        println!("{:#?}", res);

        atlas.try_delete_all(&atlas, table).await;
    }

    #[tokio::test]
    async fn test_04_try_read() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";
        let read_key = "_id".to_string();
        let read_value = "9c950da2cbab6a4e71437182846961d4".to_string();

        let read_result = atlas
            .try_read::<Game>(&atlas, table, read_key, read_value)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_documents_by_ids() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";
        let ids = vec![
            "e40d079e6db5293e7e0aa22e0c857a85".to_string(),
            "2".to_string(),
            "3".to_string(),
        ];
        let res = atlas
            .try_read_documents_by_ids(&atlas, table, ids)
            .await
            .unwrap();
        print!("{:#?}", res);
    }

    #[tokio::test]
    async fn test_6_try_read_all() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let read_all_result = atlas.try_read_all(&atlas, table).await.unwrap();
        println!("{:#?}", read_all_result)
    }

    #[tokio::test]
    async fn test_07_try_update() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table = "books";
        let record_id = &"56420b74c402bfccb04db2542d901054".to_string();
        let updated_record = doc! {
            "_id": record_id,
            "data": {
                "name": "Not Harry Potter",
                "author": "JK Rowling"
            }
        };

        let update_result = atlas
            .try_update_one(&atlas, table, record_id, updated_record)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_08_try_update_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table = "books";
        let mut update_map = HashMap::new();
        let update_doc_1 = doc! {
            "_id": "abe2c187d35b88402a28c99a113601e9",
            "data": {
                "name": "The Stand",
                "author": "Stephen King"
            }
        };
        let update_doc_2 = doc! {
            "_id": "0aa7ba9d4ef9dfacd6c1d4e545b86e87",
            "data": {
                "name": "The Dunwich Horror",
                "author": "HP Lovecraft"
            }
        };
        update_map.insert("abe2c187d35b88402a28c99a113601e9", update_doc_1);
        update_map.insert("0aa7ba9d4ef9dfacd6c1d4e545b86e87", update_doc_2);

        let update_result = atlas
            .try_update_many(&atlas, table, update_map)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_09_try_delete() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let test_data_struct = TestData::new();
        let data = test_data_struct.data_3;

        let outcomes = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let book = &outcomes[0];

        let _ = atlas.try_insert(&atlas, table, book).await.unwrap();

        let record_id = "e40d079e6db5293e7e0aa22e0c857a85".to_string();

        let delete_result = atlas.try_delete(&atlas, table, record_id).await.unwrap();
        println!("{:#?}", delete_result);
    }

    #[tokio::test]
    async fn test_10_try_delete_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let delete_ids = vec![
            "abe2c187d35b88402a28c99a113601e9".to_string(),
            "0aa7ba9d4ef9dfacd6c1d4e545b86e87".to_string(),
        ];

        let _ = atlas
            .try_delete_many(&atlas, table, delete_ids)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_11_try_delete_all() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "users";
        let _ = atlas.try_delete_all(&atlas, table).await;
    }
}
