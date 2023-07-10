// #[cfg(test)]

// mod tests {
//     use super::*;
//     use crate::{
//         book_types::{Book, BookRecord},
//         cache::redis::RedisCache,
//     };

//     #[tokio::test]
//     async fn test_01_try_cache_read_delete_one() {
//         use super::*;
//         let cache = RedisCache::try_new().await.unwrap();

//         let record = BookRecord {
//             _id: "504b899eb089aee16771b4df1d96d768".to_owned(),
//             data: Book {
//                 name: "Lord of the Rings".to_owned(),
//                 author: "JRR Tolkien".to_owned(),
//                 bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
//             },
//         };

//         cache.try_cache(&record, None).await.unwrap();
//         let res = cache
//             .try_read("504b899eb089aee16771b4df1d96d768")
//             .await
//             .unwrap();

//         let assertion_value =
//             "{\"_id\":\"504b899eb089aee16771b4df1d96d768\",\"data\":{\"name\":\"Lord of the Rings\",\"author\":\"JRR Tolkien\",\"bookstore_id\":\"2b7245f77b1866f1fd422944eca23609\"}}";

//         assert_eq!(res.as_str(), assertion_value);

//         cache
//             .try_delete("504b899eb089aee16771b4df1d96d768")
//             .await
//             .unwrap();

//         let res = cache
//             .try_read("504b899eb089aee16771b4df1d96d768")
//             .await
//             .ok();
//         assert_eq!(None, res);
//     }

//     #[tokio::test]
//     async fn try_cache_read_many() {
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

//         let read_result = cache.try_read_many(book_record_ids).await.unwrap();

//         assert_eq!(read_result.len(), 2);

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
// }

//     #[tokio::test]
//     async fn test_01_try_cache_read_delete_one() {
//         use super::*;
//         let cache = RedisCache::try_new().await.unwrap();

//         let test_book_record = BookRecord {
//             _id: "504b899eb089aee16771b4df1d96d768".to_owned(),
//             data: Book {
//                 name: "Silmarillion".to_owned(),
//                 author: "JRR Tolkien".to_owned(),
//                 bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
//             },
//         };

//         let test_bookstore_record = BookstoreRecord {
//             _id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
//             data: Bookstore {
//                 name: "The Paper Hound".to_owned(),
//                 address: "344 W Pender St, Vancouver, BC V6B 1T1".to_owned(),
//                 number: "(604) 428-1344".to_owned(),
//             },
//         };

//         cache
//             .try_cache_one("books", test_book_record, None)
//             .await
//             .unwrap();

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
