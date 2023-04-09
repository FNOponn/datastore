use anyhow::Result;

use dotenv::dotenv;
use serde::Serialize;
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::{borrow::Borrow, env};

//Does the value in cache need to be the entire BookRecordInput or just Booktokio::test]
//Double check return value for cache CRUD methods

use redis::{Client, Commands, Connection};

pub struct Redis {
    pub client: Client,
    pub connection: Connection,
}

impl Redis {
    pub async fn try_new() -> Result<Self> {
        dotenv().ok();

        let redis_uri = env::var("REDIS_URI").expect("Please set the REDIS_URI environment var!");

        let client = Client::open(redis_uri)?;
        let connection = client.get_connection()?;

        Ok(Self { client, connection })
    }

    pub async fn try_cache<T>(&mut self, record: T) -> Result<String>
    where
        T: Serialize,
    {
        let conn = &mut self.connection;
        let json_record = serde_json::to_string(&record)?;
        let value_record: Value = serde_json::from_str(&json_record).unwrap();

        let trimmed_record_id = value_record["_id"].as_str().unwrap().trim_matches('"');

        let mut key = String::from("book_");
        key.push_str(trimmed_record_id);

        // let value = &value_record["data"].to_string();

        let _: () = conn.set(&key, json_record)?;
        Ok(key)
    }

    pub async fn try_read(&mut self, key: &String) -> Result<String> {
        let conn = &mut self.connection;
        let cache_get_res: String = conn.get(key)?;
        Ok(cache_get_res)
    }

    pub async fn try_delete(&mut self, key: &String) -> Result<()> {
        let conn = &mut self.connection;
        let _ = conn.del(key)?;
        Ok(())
    }
}

//How to report errors?
//Layer above? Capturing data in a type and then storing it?

// impl Debug for Redis {
//     fn fmt(&mut self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut con = self.connection;
//         let info = redis::cmd("INFO").execute(&mut con);
//         Ok(())
//     }
// }

// When instantiating the Redis struct, invoke INFO as part of the construction process.
// and stuff the output of that INFO into Redis be it String or HashMap.

// Manually impl Debug for Redis
