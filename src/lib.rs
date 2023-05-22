mod book_types;
mod cache;
mod mongodb;
mod test;

use std::{borrow::Borrow, collections::HashMap};

use crate::book_types::{Book, BookRecord, MongoStorable};
use anyhow::Result;
use bson::{from_document, to_document, Document};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cache::redis::RedisCache;
use crate::mongodb::atlas::Atlas;

pub struct Datastore {
    pub database: Atlas,
    pub cache: RedisCache,
}

#[derive(Debug, PartialEq)]
pub enum Cache<T> {
    Miss(T),
    Hit(T),
}

impl Datastore {
    pub async fn try_new(db_name: &str) -> Result<Self> {
        let atlas_connection = Atlas::try_new(db_name).await?;
        let redis_connection = RedisCache::try_new().await?;

        Ok(Self {
            database: atlas_connection,
            cache: redis_connection,
        })
    }

    pub async fn try_create<T>(
        &self,
        table: &str,
        record: T,
        cache_expiry: Option<usize>,
    ) -> Result<T>
    where
        T: Serialize + Borrow<Document> + Clone,
    {
        let _ = self.cache.try_cache(&record, cache_expiry).await?;

        let _ = self.database.try_insert_one(table, record.clone()).await?;

        Ok(record)
    }

    pub async fn try_create_many<T>(
        &self,
        table: &str,
        records: Vec<T>,
        cache_expiry: Option<usize>,
    ) -> Result<Vec<T>>
    where
        T: Serialize + Borrow<Document> + Clone,
    {
        let new_records = records.clone();

        for record in records.iter() {
            self.cache.try_cache(record, cache_expiry).await?;
        }

        let _ = self.database.try_insert_many(table, records).await?;
        Ok(new_records)
    }

    pub async fn try_read<T>(&self, table: &str, record_id: &str) -> Result<(Cache<T>)>
    where
        T: for<'de> Deserialize<'de>,
    {
        let redis_value = self.cache.try_read(&record_id).await;

        if let Ok(value) = redis_value {
            let json_value: Value = serde_json::from_str(value.as_str()).unwrap();
            let doc_value = to_document(&json_value)?;
            let cache_res = from_document::<T>(doc_value)?;
            Ok(Cache::Hit(cache_res))
        } else {
            let atlas_res = self.database.try_read_one::<T>(table, record_id).await?;
            let db_res = from_document::<T>(atlas_res)?;
            Ok(Cache::Miss(db_res))
        }
    }
    //
    pub async fn try_read_all(&self, table: &str) -> Result<Vec<Document>> {
        let redis_value = self.cache.try_read_all(table).await;

        //Set numerical limit of 100. Implement pagination
        let res = self.database.try_read_all(table).await?;
        Ok(res)
    }

    pub async fn try_update_one<T>(
        &self,
        table: &str,
        update_record: T,
        cache_expiry: Option<usize>,
    ) -> Result<T>
    where
        T: Serialize + MongoStorable + Clone,
    {
        let _ = self.cache.try_cache(&update_record, cache_expiry).await?;

        let update_record_id = &update_record.get_id().to_owned();
        let update_document = to_document(&update_record)?;

        let _ = self
            .database
            .try_update_one(table, update_record_id, update_document)
            .await?;
        Ok(update_record)
    }

    pub async fn try_update_many<T>(
        &self,
        table: &str,
        update_map: HashMap<String, Document>,
    ) -> Result<Vec<Document>>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        let records_vec: Vec<T> = update_map
            .clone()
            .into_iter()
            .map(|(_, document)| from_document(document).unwrap())
            .collect();

        let _ = self.cache.try_update_many(records_vec).await?;

        let response = self.database.try_update_many(table, update_map).await?;
        Ok(response)
    }

    pub async fn try_delete(&self, table: &str, record_id: &str) -> Result<()> {
        let _ = self.database.try_delete_one(table, record_id).await?;

        let _ = self.cache.try_delete(&record_id).await?;

        Ok(())
    }
    async fn try_delete_many(&self, table: &str, delete_ids: Vec<String>) -> Result<()> {
        let _ = self
            .database
            .try_delete_many(table, delete_ids.clone())
            .await?;

        let _ = self.cache.try_delete_many(delete_ids);
        Ok(())
    }

    async fn clear_datastore(&self, table: &str) -> Result<()> {
        let _ = self.database.try_delete_all(table).await?;
        let _ = self.cache.try_clear_cache();

        Ok(())
    }
    //Interface for Redis search
    pub async fn try_read_many(&self, table: &str, ids: Vec<String>) -> Result<Vec<Document>> {
        let res = self.database.try_read_documents_by_ids(table, ids).await?;
        Ok(res)
    }
}
