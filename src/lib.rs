mod mongodb;
mod test;
use std::{borrow::Borrow, collections::HashMap};

use anyhow::Result;
use bson::{from_document, to_document, Document};
use serde::{Deserialize, Serialize};

use crate::mongodb::Atlas;

#[derive(Debug)]
pub struct Datastore {
    pub database: Atlas,
    // pub cache: Redis,
}

pub trait MongoStorable {
    //GQLModelCompatible
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
        //Logic to insert record in Redis

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
        //If in Redis already, skip below section
        let res = self
            .database
            .try_read_one::<T>(&self.database, table, record_id)
            .await
            .unwrap();
        let read_result = from_document::<T>(res).unwrap();
        Ok(read_result)
    }

    pub async fn try_read_all(&self, table: &str) -> Result<Vec<Document>> {
        //Set numerical limit of 100. Implement pagination
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
        update_map: HashMap<String, Document>,
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
