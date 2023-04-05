#[cfg(test)]
mod datastore_tests {
    use std::collections::HashMap;

    use bson::{doc, from_document, to_document, Document};
    use serde::{Deserialize, Serialize};

    use crate::{Datastore, MongoStorable};

    use odds_api::{model::Game, test_data::TestData};

    #[derive(Debug, Deserialize, Serialize)]
    struct Book {
        name: String,
        author: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BookRecord {
        _id: String,
        data: Book,
    }

    impl MongoStorable for BookRecord {
        fn _id(&self) -> &String {
            &self._id
        }
    }

    #[tokio::test]
    async fn test_01_try_new_data_store_struct() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        println!("{:#?}", data_store);
    }

    #[tokio::test]
    async fn test_02_try_create_one() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();
        let table = "books";
        let new_record = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "Red Wall",
                "author": "Brian Jacques"
            }
        };
        let res = data_store.try_create(table, new_record).await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_create_many() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let test_data_struct = TestData::new();
        let table = "users";
        let data = test_data_struct.data_1;
        let outcomes = serde_json::from_str::<Vec<Game>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        data_store.try_create_many(table, outcomes).await.unwrap();
    }

    #[tokio::test]
    async fn test_04_try_read() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";
        let record_id = "7e9dd50fdb0b2d895dffc78a239997f7".to_string();

        let read_result = data_store
            .try_read::<Document>(table, record_id)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_all() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";
        let try_read_all_result = data_store.try_read_all(table).await.unwrap();
        println!("{:#?}", try_read_all_result);
    }

    #[tokio::test]
    async fn test_06_try_update_one() {
        //Double Check
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "books";

        let update_record = doc! {
            "_id": "7e9dd50fdb0b2d895dffc78a239997f7",
            "data": {
                "name": "Red Wall",
                "author": "Brian Jacques"
            }
        };

        let update_record = from_document::<BookRecord>(update_record).unwrap();

        let update_result = data_store
            .try_update_one(table, update_record)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_06_try_update_many() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

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

        let update_result = data_store.try_update_many(table, update_map).await.unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_07_try_delete() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let collection_name = "users";
        let record_id = "6423b7d647a9690332556b41".to_string();

        let res = data_store
            .try_delete(collection_name, record_id)
            .await
            .unwrap();

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_08_find_documents_by_ids() {
        let db_name = "fnchart";
        let data_store = Datastore::try_new(db_name).await.unwrap();

        let table = "users";

        let ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let update_result = data_store
            .try_read_documents_by_ids(table, ids)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }
}
