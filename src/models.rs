use crate::schema::books;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub book_rating: i32,
    pub genre: Option<String>,
    pub tags: Vec<Option<String>>,
}

#[derive(Deserialize)]
pub struct ProtectedNewBook {
    pub new_book: NewBook,
    pub key: String,
}

#[derive(Deserialize)]
pub struct UpdateBook {
    pub author: String,
    pub book_rating: i32,
    pub genre: Option<String>,
    pub tags: Vec<Option<String>>,
    pub available: bool,
    pub count: i32,
}

use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize, Clone)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub book_rating: i32,
    pub genre: Option<String>,
    pub tags: Vec<Option<String>>,
    pub available: bool,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct BookQuery {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub rating: Option<i32>,
    pub genres: Option<String>,
    pub tags: Option<String>,
    pub available: Option<bool>,
}
