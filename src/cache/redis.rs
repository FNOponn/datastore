use anyhow::Result;
use dotenv::dotenv;

use mobc_redis::redis::{AsyncCommands, ToRedisArgs};
use mobc_redis::{redis, RedisConnectionManager};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use std::{env, println};
use thiserror::Error;

use crate::book_types::{Book, BookRecord, MongoStorable};
use crate::cache::redis::MobcError::*;

#[derive(Error, Debug)]
pub enum MobcError {
    #[error("could not get redis connection from pool : {0}")]
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(mobc_redis::redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(mobc_redis::redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(mobc_redis::redis::RedisError),
}

pub type MobcPool = mobc::Pool<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub struct RedisCache {
    pub pool: MobcPool,
}

impl RedisCache {
    pub async fn try_new() -> Result<Self> {
        let pool = RedisCache::connect().await?;

        Ok(Self { pool })
    }

    async fn connect() -> Result<MobcPool> {
        dotenv().ok();
        let redis_uri = env::var("REDIS_URI").expect("Please set the REDIS_URI environment var!");

        let client = redis::Client::open(redis_uri).map_err(RedisClientError)?;
        let manager = RedisConnectionManager::new(client);

        Ok(mobc::Pool::builder()
            .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .build(manager))
    }

    pub async fn get_info(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;

        let res: String = redis::cmd("INFO")
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await?;

        println!("{:#?}", res);

        Ok(())
    }

    pub async fn record_to_redis_map<T>(&self, record: T) -> Result<(String, String)>
    where
        T: Serialize,
    {
        let json_record = serde_json::to_string(&record)?;
        let value_record: Value = serde_json::from_str(&json_record)?;

        let trimmed_record_id = value_record["_id"].as_str().unwrap().trim_matches('"');

        let mut key = String::from("book_");
        key.push_str(trimmed_record_id);
        Ok((key, json_record))
    }

    //Every T should be RedisStorable(Conversions) and MongoStorable
    pub async fn try_cache<T>(&self, record: &T, expiry_time: Option<usize>) -> Result<()>
    //T traitbound RediStorable
    where
        T: Serialize,
    {
        let mut conn = self.pool.get().await?;

        let (key, json_record) = self.record_to_redis_map(&record).await?;

        let _ = conn.set(&key, json_record).await.map_err(RedisCMDError)?;

        if let Some(expiry_seconds) = expiry_time {
            let _ = conn
                .expire(&key, expiry_seconds)
                .await
                .map_err(RedisCMDError)?;
        }

        Ok(())
    }

    pub async fn try_cache_many<T>(&self, records: Vec<T>, expiry_time: Option<usize>) -> Result<()>
    where
        T: Serialize + MongoStorable,
    {
        let mut conn = self.pool.get().await?;

        let tuple_records: Vec<(String, String)> = records
            .iter()
            .map(|record| (record.try_to_str().unwrap()))
            .collect();

        println!("{:#?}", tuple_records);

        let _: () = redis::cmd("MSET")
            .arg(tuple_records)
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await?;

        Ok(())
    }

    pub async fn try_read(&self, record_id: &str) -> Result<String> {
        let mut conn = self.pool.get().await?;

        let mut key = String::from("book_");
        key.push_str(record_id);
        let cache_get_res: String = conn.get(key).await.map_err(RedisCMDError)?;
        Ok(cache_get_res)
    }

    pub async fn try_read_many(&self, ids: Vec<String>) -> Result<Vec<String>> {
        let mut conn = self.pool.get().await?;

        let ids: Vec<String> = ids.iter().map(|id| format!("book_{}", id)).collect();

        let values: Vec<String> = conn.mget(ids).await.map_err(RedisCMDError)?;

        Ok(values)
    }

    pub async fn try_read_all(&self, table_name: &str) -> Result<Vec<String>> {
        let mut conn = self.pool.get().await?;
        let prefix = "book_";
        let keys: Vec<String> = conn
            .keys(format!("{}*", prefix))
            .await
            .map_err(RedisCMDError)?;
        let values: Vec<String> = conn.mget(keys).await.map_err(RedisCMDError)?;
        Ok(values)
    }

    pub async fn try_update_many<T>(&self, updated_records: Vec<T>) -> Result<()>
    where
        T: Serialize,
    {
        for record in updated_records.into_iter() {
            let _ = self.try_cache(&record, None).await.unwrap();
        }

        Ok(())
    }

    pub async fn try_delete(&self, record_id: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let mut key = String::from("book_");
        key.push_str(record_id);
        let _ = conn.del(key).await.map_err(RedisCMDError)?;
        Ok(())
    }

    pub async fn try_delete_many(&self, delete_ids: Vec<String>) -> Result<()> {
        //Double check input
        let mut conn = self.pool.get().await?;

        let delete_keys: Vec<String> = delete_ids.iter().map(|id| format!("book_{}", id)).collect();

        let _: () = conn.del(delete_keys).await?;

        Ok(())
    }

    pub async fn try_clear_cache(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;

        let _: () = redis::cmd("FLUSHALL")
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await?;

        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{
        book_types::{Book, BookRecord},
        cache::redis::RedisCache,
    };
    use serde::{Deserialize, Serialize};

    #[tokio::test]
    async fn test_01_try_cache_read_delete_one() {
        use super::*;
        let cache = RedisCache::try_new().await.unwrap();

        let record = BookRecord {
            _id: "1".to_owned(),
            data: Book {
                name: "Lord of the Rings".to_owned(),
                author: "JRR Tolkien".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        cache.try_cache(&record, None).await.unwrap();
        let res = cache.try_read("1").await.unwrap();

        let assertion_value =
            "{\"_id\":\"1\",\"data\":{\"name\":\"Lord of the Rings\",\"author\":\"JRR Tolkien\",\"bookstore_id\":\"2b7245f77b1866f1fd422944eca23609\"}}";

        assert_eq!(res.as_str(), assertion_value);

        cache.try_delete("1").await.unwrap();

        let res = cache.try_read("1").await.ok();
        assert_eq!(None, res);
    }

    #[tokio::test]
    async fn test_02_try_cache_read_delete_multiple() {
        use super::*;

        let cache = RedisCache::try_new().await.unwrap();

        // cache.try_cache(&record_1, None).await.unwrap();
        // cache.try_cache(&record_2, None).await.unwrap();

        // let ids = vec!["1".to_string(), "2".to_string()];

        // let res = cache.try_read_many(ids).await.unwrap();

        // let exp_res = [
        //     "{\"_id\":\"1\",\"data\":{\"name\":\"Lord of the Rings\",\"author\":\"JRR Tolkien\"}}",
        //     "{\"_id\":\"2\",\"data\":{\"name\":\"The Hobbit\",\"author\":\"JRR Tolkien\"}}",
        // ];
        // assert_eq!(res, exp_res);

        // let record_1 = BookRecord {
        //     _id: "1".to_string(),
        //     data: Book {
        //         name: "Foundation".to_string(),
        //         author: "Isaac Asimov".to_string(),
        //     },
        // };

        // let record_2 = BookRecord {
        //     _id: "2".to_string(),
        //     data: Book {
        //         name: "Foundation and Empire".to_string(),
        //         author: "Isaac Asimov".to_string(),
        //     },
        // };
        // let updated_records = vec![record_1, record_2];

        // cache.try_update_many(updated_records).await.unwrap();

        // let updated_ids = vec!["1".to_string(), "2".to_string()];

        // let res = cache.try_read_many(updated_ids).await.unwrap();

        // let exp_res = [
        //     "{\"_id\":\"1\",\"data\":{\"name\":\"Foundation\",\"author\":\"Isaac Asimov\"}}",
        //     "{\"_id\":\"2\",\"data\":{\"name\":\"Foundation and Empire\",\"author\":\"Isaac Asimov\"}}",
        // ];
        // assert_eq!(res, exp_res);
    }
    #[tokio::test]
    async fn test_03_try_clear_cache() {
        let cache = RedisCache::try_new().await.unwrap();

        cache.try_clear_cache().await.unwrap();

        let mut conn = cache.pool.get().await.unwrap();

        let cache_size: usize = redis::cmd("DBSIZE")
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await
            .unwrap();

        assert_eq!(cache_size, 0)
    }

    #[tokio::test]
    async fn try_cache_read_delete_many() {
        let cache = RedisCache::try_new().await.unwrap();

        let record_1 = BookRecord {
            _id: "1".to_owned(),
            data: Book {
                name: "Foundation".to_owned(),
                author: "Isaac Asimov".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let record_2 = BookRecord {
            _id: "2".to_owned(),
            data: Book {
                name: "Foundation and Empire".to_owned(),
                author: "Isaac Asimov".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };
        let updated_records = vec![record_1, record_2];

        let _: () = cache.try_cache_many(updated_records, None).await.unwrap();

        //Assert that the read value from Redis is equal to above BOTH books above
    }
}
