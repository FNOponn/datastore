use anyhow::Result;
use bson::{doc, to_document, Document};
use dotenv::dotenv;
use mongodb::{
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Database,
};
use serde::Serialize;
use std::env;

#[derive(Debug)]
pub struct Atlas {
    pub client: Client,
    pub db: Database,
}

impl Atlas {
    pub async fn try_new() -> Result<Self> {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).await?;

        let db = client.database("fnchart");

        Ok(Self { client, db })
    }

    pub async fn try_create<T>(
        db: Database,
        collection_name: &str,
        upcoming_games: Vec<T>,
    ) -> Result<Vec<InsertOneResult>>
    where
        T: Sized + Serialize,
    {
        {
            let games_collection = db.collection::<Document>(collection_name);

            let mut result_vec: Vec<InsertOneResult> = Vec::new();

            for game in &upcoming_games {
                //insert_multiple
                let bson_game = to_document(&game)?;
                let insert_result = games_collection.insert_one(bson_game, None).await?;
                println!("New document ID: {}", &insert_result.inserted_id);
                result_vec.push(insert_result);
            }
            Ok(result_vec)
        }
    }

    pub async fn try_read(
        db: Database,
        collection_name: &str,
        read_key: String,
        read_value: String,
    ) -> Result<Document> {
        let games_collection = db.collection::<Document>(&collection_name);

        let query = doc! {
            "id": read_value
        };

        let find_result = games_collection.find_one(query, None).await?.unwrap();

        Ok(find_result)
    }

    pub async fn try_update(
        db: Database,
        collection_name: &str,
        game_id: String,
        update_key: String,
        update_value: String,
    ) -> Result<UpdateResult> {
        let games_collection = db.collection::<Document>(collection_name);

        let query = doc! {
            "id": game_id
        };

        let update = doc! {
                  "$set": { update_key: update_value }
        };

        let update_result = games_collection
            .update_one(query, update, None)
            .await
            .unwrap();

        Ok(update_result)
    }

    pub async fn try_delete_many(
        db: Database,
        collection_name: &str,
        delete_key: String,
        delete_value: String,
    ) -> Result<DeleteResult> {
        let games_collection = db.collection::<Document>(&collection_name);

        let query = doc! {
            delete_key: delete_value
        };

        let delete_result = games_collection.delete_many(query, None).await?;
        Ok(delete_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, test_data::TestData};

    #[tokio::test]
    async fn test_01_try_new_atlas_struct() {
        let atlas = Atlas::try_new().await.unwrap();
        println!("{:#?}", atlas);
    }

    #[tokio::test]
    async fn test_02_try_create() {
        let atlas = Atlas::try_new().await.unwrap();
        let test_data_struct = TestData::new();
        let collection_name = "users";
        let data = test_data_struct.data_1;
        let outcomes = serde_json::from_str::<Vec<api::Game>>(&data).unwrap();
        println!("{:#?}", outcomes);
        Atlas::try_create(atlas.db, collection_name, outcomes)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_03_try_read() {
        let atlas = Atlas::try_new().await.unwrap();
        let read_key = "id".to_string();
        let read_value = "9c950da2cbab6a4e71437182846961d4".to_string();
        let collection_name = "users";
        let find_response = Atlas::try_read(atlas.db, collection_name, read_key, read_value)
            .await
            .unwrap();
        println!("{:#?}", find_response);
    }

    #[tokio::test]
    async fn test_04_try_update() {
        let atlas = Atlas::try_new().await.unwrap();
        let collection_name = "users";
        let query_id = "9c950da2cbab6a4e71437182846961d4".to_string();
        let update_key = "sport_title".to_string();
        let update_value = "NFL".to_string();
        let update_result = Atlas::try_update(
            atlas.db,
            collection_name,
            query_id,
            update_key,
            update_value,
        )
        .await
        .unwrap();
        println!("{:#?}", update_result);
    }

    #[tokio::test]
    async fn test_05_try_delete() {
        let atlas = Atlas::try_new().await.unwrap();
        let collection_name = "users";
        let delete_key = "home_team".to_string();
        let delete_value = "Houston Rockets".to_string();
        let delete_result =
            Atlas::try_delete_many(atlas.db, collection_name, delete_key, delete_value)
                .await
                .unwrap();
        println!("{:#?}", delete_result);
    }
}
