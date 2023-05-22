#[cfg(test)]
mod atlas_tests {

    use std::collections::HashMap;

    use crate::mongodb::atlas::Atlas;

    use crate::book_types::{Book, BookRecord, Bookstore, BookstoreRecord};

    use bson::{doc, to_document, Document};
    use odds_api::test_data::TestData;

    #[tokio::test]
    async fn test_02_try_insert_one() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let test_data_struct = TestData::new();

        //Change to original struct here

        let data = test_data_struct.book;

        let records = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let insert_record = &records[0];

        let _ = atlas.try_insert_one(table, insert_record).await.unwrap();

        let record_id = "e40d079e6db5293e7e0aa22e0c857a85";

        let _ = atlas.try_delete_one(table, record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_03_try_insert_many() {
        //TODO: Assert and change
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let test_data_struct = TestData::new();
        let table = "books";

        let data = test_data_struct.book;
        let outcomes = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let res = atlas.try_insert_many(table, outcomes).await.unwrap();
        println!("{:#?}", res);

        // atlas.try_delete_all(&atlas, table).await;
    }

    //Mirror BookRecord for tests
    //Make a giant test
    //Tests should not be permanent
    //Read, check if empty, if empty, create, grab
    //One test that encapsulates CRUD
    #[tokio::test]
    async fn test_04_try_read() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";
        let record_id = "abe2c187d35b88402a28c99a113601e9".to_string();

        let read_result = atlas
            .try_read_one::<Document>(table, &record_id)
            .await
            .unwrap();

        println!("{:#?}", read_result);
    }

    #[tokio::test]
    async fn test_05_try_read_documents_by_ids() {
        //Create in one go
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";
        let ids = vec![
            "e40d079e6db5293e7e0aa22e0c857a85".to_string(),
            "0aa7ba9d4ef9dfacd6c1d4e545b86e87".to_string(),
        ];
        let res = atlas.try_read_documents_by_ids(table, ids).await.unwrap();
        print!("{:#?}", res);
        //Delete here
    }

    #[tokio::test]
    async fn test_6_try_read_all() {
        let db_name = "fnchart";

        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let read_all_result = atlas.try_read_all(table).await.unwrap();
        println!("{:#?}", read_all_result)
    }

    #[tokio::test]
    async fn test_07_try_update() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table = "books";
        let record_id = "56420b74c402bfccb04db2542d901054";
        let updated_record = doc! {
            "_id": record_id,
            "data": {
                "name": "A Very Harry Potter",
                "author": "JK Rowling"
            }
        };

        let update_result = atlas
            .try_update_one(table, record_id, updated_record)
            .await
            .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_08_try_update_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table = "books";
        let mut update_map = HashMap::new();
        let update_doc_1 = doc! {
            "_id": "abe2c187d35b88402a28c99a113601e9",
            "data": {
                "name": "The Stand",
                "author": "Stephen King"
            }
        };
        let update_doc_2 = doc! {
            "_id": "0aa7ba9d4ef9dfacd6c1d4e545b86e87",
            "data": {
                "name": "The Dunwich Horror",
                "author": "HP Lovecraft"
            }
        };
        update_map.insert("abe2c187d35b88402a28c99a113601e9".to_string(), update_doc_1);
        update_map.insert("0aa7ba9d4ef9dfacd6c1d4e545b86e87".to_string(), update_doc_2);

        let update_result = atlas.try_update_many(table, update_map).await.unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_09_try_delete_one() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let test_data_struct = TestData::new();
        let data = test_data_struct.book;

        let outcomes = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .take(1)
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let book = &outcomes[0];

        let _ = atlas.try_insert_one(table, book).await.unwrap();

        let record_id = "e40d079e6db5293e7e0aa22e0c857a85";

        let delete_result = atlas.try_delete_one(table, record_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_10_try_delete_many() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let table = "books";

        let delete_ids = vec![
            "abe2c187d35b88402a28c99a113601e9".to_string(),
            "0aa7ba9d4ef9dfacd6c1d4e545b86e87".to_string(),
        ];

        let _ = atlas.try_delete_many(table, delete_ids).await.unwrap();
    }

    #[tokio::test]
    async fn test_11_try_delete_all() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let table_name = "odds";
        let _ = atlas.try_delete_all(table_name).await;
        let table = atlas.db.collection::<Document>(table_name);

        let doc_count = table.count_documents(doc! {}, None).await.unwrap();

        assert_eq!(doc_count, 0)
    }

    #[tokio::test]
    async fn test_12_try_join() {
        //TODO: Assert and change
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();
        let test_data_struct = TestData::new();
        let table = "bookstore";

        let data = test_data_struct.bookstore;
        let outcomes = serde_json::from_str::<Vec<Document>>(&data)
            .unwrap()
            .iter()
            .map(|game| to_document(game).unwrap())
            .collect::<Vec<Document>>();

        let res = atlas.try_insert_many(table, outcomes).await.unwrap();

        atlas.try_delete_all(table).await;
    }

    #[tokio::test]
    async fn test_13_try_find_one_bookstore() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let test_record = BookRecord {
            _id: "03d15979ffd0df61cd6dd3d5a2fc4d04".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let test_bookstore = BookstoreRecord {
            _id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            data: Bookstore {
                name: "The Paper Hound".to_owned(),
                address: "344 W Pender St, Vancouver, BC V6B 1T1".to_owned(),
                number: "(604) 428-1344".to_owned(),
            },
        };

        // atlas.try_insert_one("bookstore", test_record);

        let assertion_value = doc! {
            "_id": "2b7245f77b1866f1fd422944eca23609",
            "data": doc! {
                "name":  "The Paper Hound",
                "address": "344 W Pender St, Vancouver, BC V6B 1T1",
                "number": "(604) 428-1344",
            }
        };

        let res = atlas
            .try_find_one_bookstore::<BookRecord>(test_record)
            .await
            .unwrap();

        assert_eq!(res, assertion_value);
    }

    #[tokio::test]
    async fn test_14_try_find_bookstores() {
        let db_name = "fnchart";
        let atlas = Atlas::try_new(db_name).await.unwrap();

        let test_record_1 = BookRecord {
            _id: "03d15979ffd0df61cd6dd3d5a2fc4d04".to_owned(),
            data: Book {
                name: "The Grapes of Wrath".to_owned(),
                author: "John Steinbeck".to_owned(),
                bookstore_id: "2b7245f77b1866f1fd422944eca23609".to_owned(),
            },
        };

        let test_record_2 = BookRecord {
            _id: "56420b74c402bfccb04db2542d901054".to_owned(),
            data: Book {
                name: "Down and Out In Paris and London".to_owned(),
                author: "George Orwell".to_owned(),
                bookstore_id: "35fa3010596b8866ec0673550d287fad".to_owned(),
            },
        };

        // let test: Vec<_> = vec![test_record];

        // let res = atlas
        //     .try_find_bookstore::<BookRecord>(test_record)
        //     .await
        //     .unwrap();

        // assert_eq!()
        // let res = atlas
        //     .find_bookstores::<BookRecord, Book>("bookstore", test)
        //     .await
        //     .unwrap();
    }
}
