use crate::schema::books;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub language: String,
    pub book_rating: i32,
    pub description: Option<String>,
    pub genres: Vec<Option<String>>,
    pub tags: Vec<Option<String>>,
}

#[derive(Deserialize)]
pub struct ProtectedNewBook {
    pub new_book: NewBook,
    pub key: String,
}

#[derive(Deserialize)]
pub struct UpdateBook {
    pub author: Option<String>,
    pub book_rating: Option<i32>,
    pub available: Option<bool>,
    pub count: Option<i32>,
    pub description: Option<Option<String>>,
    pub genres: Option<Vec<Option<String>>>,
    pub tags: Option<Vec<Option<String>>>,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct BookQuery {
    pub title: Option<String>,
    pub author: Option<String>,
    pub book_rating: Option<i32>,
    pub genres: Option<String>,
    pub tags: Option<String>,
    pub language: Option<String>,
    pub available: Option<bool>,
}

use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub language: String,
    pub book_rating: i32,
    pub available: bool,
    pub count: i32,
    pub description: Option<String>,
    pub genres: Vec<Option<String>>,
    pub tags: Vec<Option<String>>,
}
