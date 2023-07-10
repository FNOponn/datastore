#[cfg(test)]
mod datastore_tests {
    use crate::book_types::{Book, BookRecord, MongoStorable};
    use crate::{Cache, CacheState, Datastore};
    use bson::{doc, from_document, to_document, Document};
    use std::{collections::HashMap, time::Duration};

    //create + delete (assert the deletion is successful)

    //update + delete something that does not exist

    #[tokio::test]
    // #[ignore]
    async fn test_01_try_create_read_one_from_redis() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books1";

        let book_record = BookRecord {
            _id: "03d15979ffd0df61cd6dd3d5a2fc4d04".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let _ = data_store
            .try_create_one(table, "books", book_record.clone(), None)
            .await
            .unwrap();

        let read_res = data_store
            .try_read::<BookRecord>(table, &book_record._id)
            .await
            .unwrap();

        let cache_state = read_res.state;

        let returned_book_record = read_res.data;

        assert_eq!(CacheState::Hit, cache_state);
        assert_eq!(book_record, returned_book_record);

        let _ = data_store.clear_datastore("books1").await.unwrap();
    }

    //Expiry time should be a property of the cache itself and set during constructor
    #[tokio::test]
    // #[ignore]
    async fn test_02_try_create_read_one_from_atlas() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books2";

        let book_record = BookRecord {
            _id: "cfa6fec292ddbe2004d6498d109f0225".to_owned(),
            data: Book {
                name: "East of Eden".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let _ = data_store
            .try_create_one(table, "books", book_record.clone(), Some(3))
            .await
            .unwrap();

        let wait_time = Duration::from_secs(3);
        std::thread::sleep(wait_time);

        let read_res = data_store
            .try_read::<BookRecord>(table, &book_record._id)
            .await
            .unwrap();

        let cache_state = read_res.state;

        let returned_book_record = read_res.data;

        assert_eq!(CacheState::Miss, cache_state);
        assert_eq!(book_record, returned_book_record);

        let _ = data_store.clear_datastore("books2").await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_create_update_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books";

        let book_record = BookRecord {
            _id: "a646dede9f45ef658f86da8d2ec13da4".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let _ = data_store
            .try_create_one(table, "books", book_record.clone(), None)
            .await
            .unwrap();

        let update_record = BookRecord {
            _id: "a646dede9f45ef658f86da8d2ec13da4".to_owned(),
            data: Book {
                name: "Of Mouse and Men".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let _ = data_store
            .try_update_one(table, "books", update_record.clone(), None)
            .await
            .unwrap();

        let read_res = data_store
            .try_read::<BookRecord>(table, &update_record._id)
            .await
            .unwrap();

        // println!("{:#?}", read_res);

        let returned_updated_book_record = read_res.data;

        assert_eq!(update_record, returned_updated_book_record);

        let _ = data_store.clear_datastore("books").await.unwrap();
    }

    #[tokio::test]
    async fn test_04_try_create_delete_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books4";

        let book_record = BookRecord {
            _id: "a5ec5ea231e0568f93aa88719159f4eb".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let _ = data_store
            .try_create_one(table, "books", book_record.clone(), None)
            .await
            .unwrap();

        let _ = data_store
            .try_delete(table, &book_record._id)
            .await
            .unwrap();

        let read_res = data_store
            .try_read::<BookRecord>(table, &book_record._id)
            .await
            .unwrap();

        println!("{:#?}", read_res);
        //Assert None, None
    }

    #[tokio::test]
    async fn test_10_clear_data_store() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let _ = data_store.clear_datastore("books").await.unwrap();
    }
}
