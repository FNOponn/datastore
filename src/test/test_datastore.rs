#[cfg(test)]
mod datastore_tests {
    use crate::book_types::{Book, BookRecord, MongoStorable};
    use crate::{Cache, Datastore};
    use bson::{doc, from_document, to_document, Document};
    use odds_api::test_data::TestData;
    use std::{collections::HashMap, time::Duration};

    //create(assert both exists in atlas + redis) + delete the whole thing

    //create + update(only assert the updated value) delete everything

    //create + delete (assert the deletion is successful)

    //update + delete something that does not exist

    #[tokio::test]
    async fn test_04_try_create_read_from_redis_and_delete_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books";
        let record_id = "7e9dd50fdb0b2d895dffc78a239997f7";
        let bookstore_id = "af4ce4eef1f90ea40e71";

        let new_record = doc! {
            "_id": &record_id,
            "bookstore_id": bookstore_id,
            "data": {
                "name": "Red Wall",
                "author": "Brian Jacques"
            }
        };

        let _ = data_store
            .try_create(table, new_record.clone(), None)
            .await
            .unwrap();

        let read_res = data_store
            .try_read::<BookRecord>(table, &record_id)
            .await
            .unwrap();

        let assert_record: BookRecord = from_document::<BookRecord>(new_record).unwrap();

        assert_eq!(read_res, Cache::Hit(assert_record));

        let _ = data_store.try_delete(table, record_id).await.unwrap();
    }

    //Expiry time should be a property of the cache itself and set during constructor
    #[tokio::test]
    async fn test_05_try_create_read_from_atlas_and_delete_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books";
        let record_id = "7e9dd50fdb0b2d895dffc78a239997f7";

        let new_record = doc! {
            "_id": &record_id,
            "data": {
                "name": "The Long Patrol",
                "author": "Brian Jacques"
            }
        };

        let _ = data_store
            .try_create(table, new_record.clone(), Some(3))
            .await
            .unwrap();

        let wait_time = Duration::from_secs(3);
        std::thread::sleep(wait_time);

        let read_res = data_store
            .try_read::<BookRecord>(table, record_id)
            .await
            .unwrap();

        let assert_record: BookRecord = from_document::<BookRecord>(new_record).unwrap();

        assert_eq!(read_res, Cache::Miss(assert_record));

        let _ = data_store.try_delete(table, record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_06_try_create_many_read_from_redis_and_delete_many() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let test_data_struct = TestData::new();
        let table = "books";
        let data = test_data_struct.book;
        let records = serde_json::from_str::<Vec<BookRecord>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        data_store
            .try_create_many(table, records, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_08_try_read_all() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";
        let try_read_all_result = data_store.try_read_all(table).await.unwrap();
        println!("{:#?}", try_read_all_result);
    }

    #[tokio::test]
    async fn test_09_try_update_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";

        //Create a struct here and then convert it into a document

        let original_record = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "East of Eden",
                "author": "John Steinbeck"
            }
        };

        let update_record = BookRecord {
            _id: "7e9dd50fdb0b2d895dffc78a239997f7".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "ABC".to_owned(),
            },
        };

        //   let original_record = doc! {
        //     "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
        //     "data": {
        //         "name": "East of Eden",
        //         "author": "John Steinbeck"
        //     }
        // };

        // let update_record = doc! {
        //     "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
        //     "data": {
        //         "name": "The Grapes of Wrath",
        //         "author": "John Steinbeck"
        //     }
        // };

        // let _ = data_store
        //     .try_create(table, original_record, None)
        //     .await
        //     .unwrap();

        // let update_record: BookRecord = from_document::<BookRecord>(update_record).unwrap();

        // let update_result = data_store
        //     .try_update_one(table, update_record.clone(), None)
        //     .await
        //     .unwrap();

        //Extract value from DB + Cache and assert?

        let _ = data_store
            .try_delete(table, &update_record._id)
            .await
            .unwrap();
    }

    //Big CRUD test here
    #[tokio::test]
    async fn test_10_try_update_many() {
        let db_name = "fnchart";
        let mut data_store = Datastore::try_new(db_name).await.unwrap();

        let mut update_map = HashMap::new();

        let table = "books";

        let update_record1 = doc! {
            "_id": "0aa7ba9d4ef9dfacd6c1d4e545b86e87"
        ,
            "data": {
                "name": "Dagon",
                "author": "HP Lovecraft"
            }
        };

        let update_record2 = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "The Long Patrol",
                "author": "Brian Jacques"
            }
        };

        update_map.insert(
            "0aa7ba9d4ef9dfacd6c1d4e545b86e87".to_string(),
            update_record1,
        );
        update_map.insert(
            "7e9dd50fdb0b2d895dffc78a239997f7".to_string(),
            update_record2,
        );

        let update_result = data_store
            .try_update_many::<BookRecord>(table, update_map)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_11_try_delete() {
        let db_name = "fnchart";
        let mut data_store = Datastore::try_new(db_name).await.unwrap();

        let collection_name = "users";
        let record_id = "6423b7d647a9690332556b41";

        let res = data_store
            .try_delete(collection_name, record_id)
            .await
            .unwrap();

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_12_read_documents_by_ids() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";

        let ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let update_result = data_store.try_read_many(table, ids).await.unwrap();
        println!("{:#?}", update_result);
    }
}
