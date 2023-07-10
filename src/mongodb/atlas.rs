use anyhow::{anyhow, Result};
use bson::{doc, from_document, to_document, Document};

use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    results::{InsertManyResult, InsertOneResult},
    Client, Database,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, env};

use crate::book_types::MongoStorable;

#[derive(Debug)]
pub struct Atlas {
    pub client: Client,
    pub db: Database,
}

impl Atlas {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        dotenv().ok();
        let mongo_uri =
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var");

        let client = Client::with_uri_str(mongo_uri).await?;
        let db = client.database(db_name);

        Ok(Self { client, db })
    }

    pub async fn try_insert_one<T>(&self, table: &str, record: T) -> Result<InsertOneResult>
    where
        T: Serialize,
    {
        let doc_record = to_document::<T>(&record)?;
        let collection = self.db.collection::<Document>(table);
        let insert_result = collection.insert_one(doc_record, None).await?;

        Ok(insert_result)
    }

    pub async fn try_insert_many<T>(&self, table: &str, records: Vec<T>) -> Result<InsertManyResult>
    where
        T: Sized + Serialize,
    {
        let collection = self.db.collection::<Document>(table);
        let mut session = self.client.start_session(None).await?;

        let doc_records: Vec<Document> = records
            .iter()
            .map(|record| to_document(record).unwrap())
            .collect();

        let insert_many_result = collection
            .insert_many_with_session(doc_records, None, &mut session)
            .await?;

        Ok(insert_many_result)
    }

    pub async fn try_read_one<T>(
        &self,
        table: &str,
        record_id: &str,
    ) -> Result<Document, anyhow::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let collection = self.db.collection::<Document>(table);

        let query = doc! {
            "_id": record_id
        };
        let find_result = collection
            .find_one(query, None)
            .await?
            .ok_or(anyhow!("Could not find record"));
        find_result
    }
    pub async fn try_read_documents_by_ids(
        &self,
        table: &str,
        ids: Vec<String>,
    ) -> Result<Vec<Document>> {
        let table = self.db.collection::<Document>(table);
        let mut session = self.client.start_session(None).await?;

        let filter = doc! { "_id": { "$in": ids } };
        let mut cursor = table.find_with_session(filter, None, &mut session).await?;
        let mut documents = Vec::new();

        while let Some(result) = cursor.next(&mut session).await {
            if let Ok(document) = result {
                documents.push(document);
            }
        }
        Ok(documents)
    }

    pub async fn try_read_all(&self, table: &str) -> Result<Vec<Document>> {
        let table_handle = self.db.collection::<Document>(table);
        let mut session = self.client.start_session(None).await?;

        let mut cursor = table_handle
            .find_with_session(doc! {}, None, &mut session)
            .await?;
        let mut read_res = Vec::new();

        while let Some(result) = cursor.next(&mut session).await {
            if let Ok(document) = result {
                read_res.push(document);
            }
        }

        Ok(read_res)
    }

    pub async fn try_update_one(
        &self,
        table: &str,
        update_record_id: &str,
        updated_record: Document,
    ) -> Result<Document> {
        let table = self.db.collection::<Document>(table);

        let query = doc! {
            "_id": update_record_id
        };

        let update = doc! {
            "$set": &updated_record
        };

        let update_result = table.update_one(query, update, None).await?;

        match &update_result.upserted_id {
            Some(upserted_id) => upserted_id.as_str().unwrap(),
            None => update_record_id,
        };

        Ok(updated_record)
    }

    pub async fn try_update_many(
        &self,
        table: &str,
        update_map: HashMap<String, Document>, //Type alias, Document to become T later
    ) -> Result<Vec<Document>> {
        let table = self.db.collection::<Document>(table);
        let mut session = self.client.start_session(None).await?;

        let mut updated_records: Vec<Document> = Vec::new();

        for (record_id, document) in update_map {
            let filter = doc! { "_id": record_id };
            let update_doc = doc! { "$set": &document };
            updated_records.push(document);
            table
                .update_one_with_session(filter, update_doc, None, &mut session)
                .await?;
        }

        Ok(updated_records)
    }

    pub async fn try_delete_one(&self, table: &str, record_id: &str) -> Result<Document> {
        let table = self.db.collection::<Document>(table);

        let query = doc! {
            "_id": record_id
        };

        let delete_result = table.find_one_and_delete(query, None).await?.unwrap();
        Ok(delete_result)
    }

    pub async fn try_delete_many(&self, table: &str, delete_ids: Vec<String>) -> Result<()> {
        let table = self.db.collection::<Document>(table);
        let mut session = self.client.start_session(None).await?;

        let filter = doc! {
            "_id": { "$in": delete_ids },
        };
        let res = table
            .delete_many_with_session(filter, None, &mut session)
            .await?;
        Ok(())
    }

    pub async fn try_delete_all(&self, table: &str) -> Result<()> {
        let table = self.db.collection::<Document>(table);
        table.delete_many(doc! {}, None).await?;
        Ok(())
    }

    pub async fn try_find_one_bookstore<T>(&self, record: T) -> Result<Document>
    where
        T: MongoStorable,
    {
        let bookstore_collection = self.db.collection::<Document>("bookstore");

        let record_bookstore_id = record.get_bookstore_id();

        let query = doc! { "_id": record_bookstore_id };

        let res = bookstore_collection
            .find_one(query, None)
            .await?
            .ok_or(anyhow!("Could not find bookstore"));
        res
    }

    //Implement pagination for any read method including below
    //Chagne Vec to Arr with limit
    pub async fn find_bookstores<T>(&self, records: Vec<&str>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let table = self.db.collection::<Document>("bookstores");

        let pipeline = vec![
            doc! {
                "$lookup": {
                    "from": "books",
                    "localField": "_id",
                    "foreignField": "data.bookstore_id",
                    "as": "res"
                }
            },
            doc! {
                "$match": {
                    "$and": [
                        {
                            "res._id": {
                                "$in": records
                            }
                        },
                        {
                            "$expr": {
                            "$ne": [{ "$size": "$res" }, 0]
                        }
                    }
                    ]
                }
            },
            doc! { "$project": { "res": 0 } },
        ];

        let mut bookstores = Vec::new();
        let mut cursor = table.aggregate(pipeline, None).await?;

        while let Some(result) = cursor.next().await {
            let record = from_document::<T>(result?)?;
            bookstores.push(record)
        }

        Ok(bookstores)
    }
}
