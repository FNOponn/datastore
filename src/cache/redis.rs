use anyhow::Result;
use bson::Document;
use dotenv::dotenv;

use mobc_redis::redis::{AsyncCommands, ToRedisArgs};
use mobc_redis::{redis, RedisConnectionManager};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Map, Value};
use std::collections::HashMap;
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

    pub async fn record_to_redis_map<T>(&self, record: T) -> Result<(String, Map<String, Value>)>
    where
        T: Serialize + MongoStorable,
    {
        let json_value = serde_json::to_value(&record)?;
        let json_object = json_value.as_object().unwrap();

        let key = json_object.get("_id").unwrap();
        let key_str = to_string(key).unwrap();

        let redis_book_entry = (key_str, json_object.clone());

        Ok(redis_book_entry)
    }

    //Every T should be RedisStorable(Conversions) and MongoStorable
    pub async fn try_cache_one<T>(
        &self,
        hash_key: &str,
        record: T,
        expiry_time: Option<usize>,
    ) -> Result<()>
    //T traitbound RediStorable
    where
        T: Serialize + MongoStorable,
    {
        let mut conn = self.pool.get().await?;

        let (field, value) = self.record_to_redis_map(record).await?;

        let book_id = &field[1..(field.len() - 1)];
        let book_record: String = serde_json::to_string(&value)?;

        let _: () = conn
            .hset(hash_key, book_id, book_record)
            .await
            .map_err(RedisCMDError)
            .unwrap();

        //Incorporate both in try_cache_one API and expire both
        //book_stores hash will consist of many records of book_id: Vec<StoreID>
        //store_books hash will consist of many records of store_id:Vec<BookID>

        //Enum that wraps the whole argument
        //If 1 to many, do additional steps,
        //1 to 1 dont do anything

        match hash_key {
            "books" => {
                // let bookstore_id = record.get_bookstore_id();
                let mut test = book_id.to_owned();
                test.insert(0, '"'); // Insert 'a' at the beginning
                test.push('"'); // Append 'a' at the end
                let existing_bookstore_ids: Option<String> =
                    conn.hget("book_stores", &test).await.unwrap();
                println!("{:#?} {:#?}", test, existing_bookstore_ids);

                // let new_bookstore_ids = match existing_bookstore_ids {
                //     Some(existing) => {
                //         let mut combined = existing;
                //         combined
                //     }
                //     None => (),
                // };

                // let test = vec![1, 2, 3];
                // let vector = serde_json::to_string(&test).unwrap();

                // let _: () = conn
                //     .hset("book_store_list", 1, &vector)
                //     .await
                //     .map_err(RedisCMDError)
                //     .unwrap();
            }
            _ => (),
        }

        // book_store_list

        // store_book_list

        if let Some(expiry_seconds) = expiry_time {
            let _ = conn
                .expire(hash_key, expiry_seconds)
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

        let redis_kv_records: Vec<(String, String)> = records
            .iter()
            .map(|record| (record.try_to_str().unwrap()))
            .collect();

        let _: () = redis::cmd("MSET")
            .arg(redis_kv_records)
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await?;

        Ok(())
    }

    pub async fn try_read<T>(&self, record_id: &str) -> Result<String>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.pool.get().await?;

        let read_res: String = conn.hget("books", record_id).await.map_err(RedisCMDError)?;

        Ok(read_res)
    }

    pub async fn try_read_many<T, U>(&self, ids: Vec<String>) -> Result<()>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let mut conn = self.pool.get().await?;

        let ids: Vec<String> = ids.iter().map(|id| format!("book_{}", id)).collect(); //Sure we still want this?

        let values: Vec<String> = conn.mget(ids).await.map_err(RedisCMDError)?;

        // for value in values {
        //     let test: T = serde_json::from_str(value.as_str())?;
        // }

        // let arr: Vec<Book> = values
        //     .iter()
        //     .map(|value| serde_json::from_str(value).unwrap())
        //     .collect();

        // let vec: Vec<BookRecord> = arr
        //     .into_iter()
        //     .map(|book: Book| BookRecord {
        //         _id: "123".to_owned(),
        //         data: book,
        //     })
        //     .collect();

        // println!("read_many_res: {:#?}", vec);

        // Ok(read_many_res);
        Ok(())
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
        T: Serialize + MongoStorable,
    {
        for record in updated_records.into_iter() {
            let _ = self.try_cache_one("books", record, None).await.unwrap();
        }

        Ok(())
    }

    pub async fn try_delete(&self, hash_key: &str, record_id: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;

        let _ = conn
            .hdel(hash_key, record_id)
            .await
            .map_err(RedisCMDError)?;
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
        book_types::{Book, BookRecord, Bookstore, BookstoreRecord},
        cache::redis::RedisCache,
    };

    #[tokio::test]
    #[ignore]
    async fn test_01_try_cache_read_one() {
        use super::*;
        let cache = RedisCache::try_new().await.unwrap();

        let book = BookRecord {
            _id: "b5e276f4924b4235d96e5d35b872b012".to_owned(),
            data: Book {
                name: "The Hobbit".to_owned(),
                author: "JRR Tolkien".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        cache.try_cache_one("books", book, None).await.unwrap();

        let cache_read_res: String = cache
            .try_read::<BookRecord>("b5e276f4924b4235d96e5d35b872b012")
            .await
            .unwrap();

        let expected_val = "{\"_id\":\"b5e276f4924b4235d96e5d35b872b012\",\"data\":{\"name\":\"The Hobbit\",\"author\":\"JRR Tolkien\",\"bookstore_id\":\"2b7245f77b1866f1fd422944eca23609\"}}";
        assert_eq!(expected_val, cache_read_res);
        cache.try_clear_cache().await.unwrap();
    }

    #[tokio::test]
    async fn test_02_try_cache_delete_one() {
        use super::*;
        let cache = RedisCache::try_new().await.unwrap();

        let book = BookRecord {
            _id: "633bd287609b5b5854509b614ef5bee0".to_owned(),
            data: Book {
                name: "Similarion".to_owned(),
                author: "JRR Tolkien".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        cache
            .try_cache_one("books", book.clone(), None)
            .await
            .unwrap();

        cache.try_delete("books", book._id.as_str()).await.unwrap();

        // let cache_read_res = cache
        //     .try_read::<BookRecord>("b5e276f4924b4235d96e5d35b872b012")
        //     .await
        //     .unwrap();
        //How do we handle a case where the read results in NOTHING?

        //Assert none?

        // println!("{}", cache_read_res);
    }

    #[tokio::test]
    #[ignore]
    async fn test10_try_clear_cache() {
        let cache = RedisCache::try_new().await.unwrap();
        cache.try_clear_cache().await.unwrap();

        let mut conn = cache.pool.get().await.unwrap();

        let cache_size: usize = redis::cmd("DBSIZE")
            .query_async(&mut conn as &mut redis::aio::Connection)
            .await
            .unwrap();

        assert_eq!(cache_size, 0)
    }
}

//

//         // cache.try_read::<BookRecord>("Mike").await.unwrap();

//         // cache
//         //     .try_cache_one("bookstores", test_bookstore_record, None)
//         //     .await
//         //     .unwrap();

//         // let res = cache
//         //     .try_read::<BookRecord>("504b899eb089aee16771b4df1d96d768")
//         //     .await
//         //     .unwrap();

//         // assert_eq!(res, test_record);

//         // cache
//         //     .try_delete("504b899eb089aee16771b4df1d96d768")
//         //     .await
//         //     .unwrap();

//         // let res = cache
//         //     .try_read::<BookRecord>("504b899eb089aee16771b4df1d96d768")
//         //     .await
//         //     .ok();
//         // assert_eq!(None, res);
//     }

//     #[tokio::test]
//     async fn test_02_try_cache_read_many() {
//         let cache = RedisCache::try_new().await.unwrap();

//         let record_1 = BookRecord {
//             _id: "a6b14a3123d327627a887a6a442d427f".to_owned(),
//             data: Book {
//                 name: "Foundation".to_owned(),
//                 author: "Isaac Asimov".to_owned(),
//                 bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
//             },
//         };

//         let record_2 = BookRecord {
//             _id: "bce451618c93a32e69e7a774504d994c".to_owned(),
//             data: Book {
//                 name: "Foundation and Empire".to_owned(),
//                 author: "Isaac Asimov".to_owned(),
//                 bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
//             },
//         };
//         let book_records = vec![record_1, record_2];
//         let book_record_ids = vec![
//             "a6b14a3123d327627a887a6a442d427f".to_owned(),
//             "bce451618c93a32e69e7a774504d994c".to_owned(),
//         ];

//         let _: () = cache.try_cache_many(book_records, None).await.unwrap();

//         let read_result = cache
//             .try_read_many::<Book, BookRecord>(book_record_ids)
//             .await
//             .unwrap();

//         // assert_eq!(read_result.len(), 2);

//         println!("{:#?}", read_result);

//         let _: () = cache.try_clear_cache().await.unwrap();
//     }

//     #[tokio::test]
//     async fn test_03_try_clear_cache() {
//         let cache = RedisCache::try_new().await.unwrap();

//         cache.try_clear_cache().await.unwrap();

//         let mut conn = cache.pool.get().await.unwrap();

//         let cache_size: usize = redis::cmd("DBSIZE")
//             .query_async(&mut conn as &mut redis::aio::Connection)
//             .await
//             .unwrap();

//         assert_eq!(cache_size, 0)
//     }

//     #[tokio::test]
//     async fn test_04_try_clear_cache() {
//         let cache = RedisCache::try_new().await.unwrap();

//         let mut conn = cache.pool.get().await.unwrap();

//         #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]

//         struct MyStruct {
//             field1: i32,
//             field2: String,
//             field3: bool,
//         }

//         let instance = MyStruct {
//             field1: 42,
//             field2: "Hello".to_string(),
//             field3: true,
//         };

//         // let json_record = serde_json::to_string(&record)?;
//         let json_value = serde_json::to_value(&instance).unwrap();
//         let json_object = json_value.as_object().unwrap();

//         let kv_tuple_vec: Vec<(String, String)> = json_object
//             .iter()
//             .map(|(k, v)| (k.clone().to_string(), serde_json::to_string(&v).unwrap()))
//             .collect();

//         println!("{:#?}", kv_tuple_vec);
//         // let trimmed_record_id = value_record["_id"].as_str().unwrap().trim_matches('"');

//         let mut key = String::from("book_");
//         // key.push_str(trimmed_record_id);
//         // Ok((key, json_record))

//         let _: () = conn
//             .hset_multiple(&key, &kv_tuple_vec)
//             .await
//             .map_err(RedisCMDError)
//             .unwrap();
//     }
// }
