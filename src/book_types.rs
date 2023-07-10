use std::borrow::Borrow;

use anyhow::Result;

use bson::{doc, to_document, Document};
use serde::{Deserialize, Serialize};

pub trait MongoStorable {
    type Data;

    fn get_id(&self) -> &str;

    fn get_data(&self) -> &Self::Data;

    fn get_bookstore_id(&self) -> &str;

    fn try_to_str(&self) -> Result<(String, String)>;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BookRecord {
    pub _id: String,
    pub data: Book,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub bookstore_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BookstoreRecord {
    pub _id: String,
    pub data: Bookstore,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Bookstore {
    pub name: String,
    pub address: String,
    pub number: String,
}

impl MongoStorable for BookRecord {
    type Data = Book;

    fn get_id(&self) -> &str {
        &self._id
    }

    fn get_data(&self) -> &Book {
        &self.data
    }

    fn get_bookstore_id(&self) -> &str {
        &self.data.bookstore_id
    }

    fn try_to_str(&self) -> Result<(String, String)> {
        let mut key = self.get_id().to_owned();
        key.insert_str(0, "book_");

        let book_data = self.get_data();
        let value = serde_json::to_string(&book_data)?;
        Ok((key, value))
    }
}

impl MongoStorable for BookstoreRecord {
    type Data = Bookstore;

    fn get_id(&self) -> &str {
        &self._id
    }

    fn get_data(&self) -> &Bookstore {
        &self.data
    }

    fn get_bookstore_id(&self) -> &str {
        &self.data.name
    }

    fn try_to_str(&self) -> Result<(String, String)> {
        let mut key = self.get_id().to_owned();
        key.insert_str(0, "book_");

        let book_data = self.get_data();
        let value = serde_json::to_string(&book_data)?;
        Ok((key, value))
    }
}
