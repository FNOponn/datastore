#[cfg(test)]

mod redis_test {
    use crate::cache::redis::Redis;
    use redis::Commands;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BookRecordInput {
        pub _id: String, //Revise later
        pub data: Book,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Book {
        pub name: String,
        pub author: String,
    }

    //Is this test necessary?
    // #[tokio::test]
    // async fn test_01_try_new_redis() {
    //     let redis_cache = Redis::try_new().await.unwrap();
    // }

    async fn test_02_try_cache_read_delete() {
        let mut cache = Redis::try_new().await.unwrap();

        let record = BookRecordInput {
            _id: "1".to_string(),
            data: Book {
                name: "Lord of the Rings".to_string(),
                author: "JRR Tolkien".to_string(),
            },
        };

        let cache_record_id = cache.try_cache(&record).await.unwrap();

        let get_res: String = cache.try_read(&cache_record_id).await.unwrap();

        let book_record_json = serde_json::to_string(&record).unwrap();

        assert_eq!(get_res, book_record_json);

        cache.try_delete(&cache_record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_update_redis() {
        let mut cache = Redis::try_new().await.unwrap();

        let record_1 = BookRecordInput {
            _id: "2".to_string(),
            data: Book {
                name: "Lord of the Rings".to_string(),
                author: "JRR Tolkien".to_string(),
            },
        };

        cache.try_cache(&record_1).await.unwrap();

        let record_2 = BookRecordInput {
            _id: "2".to_string(),
            data: Book {
                name: "The Hobbit".to_string(),
                author: "JRR Tolkien".to_string(),
            },
        };

        let cache_record_id = cache.try_cache(&record_2).await.unwrap();

        let get_res: String = cache.try_read(&cache_record_id).await.unwrap();

        let record_json = serde_json::to_string(&record_2).unwrap();

        assert_eq!(get_res, record_json);

        cache.try_delete(&cache_record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_04_try_delete_redis() {
        let mut cache = Redis::try_new().await.unwrap();

        let record = BookRecordInput {
            _id: "3".to_string(),
            data: Book {
                name: "The Silmarillion".to_string(),
                author: "JRR Tolkien".to_string(),
            },
        };

        let cache_record_id = cache.try_cache(&record).await.unwrap();
        cache.try_delete(&cache_record_id).await.unwrap();

        let record_exists: bool = cache.connection.exists("book_record_3").unwrap();
        assert!(!record_exists);
    }
}
