use crate::schema::books;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable, Clone)]
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

// use crate::schema::books;

// #[derive(Insertable)]
// #[diesel(table_name = books)]
// pub struct NewBook<'a> {
//     pub title: &'a str,
//     pub author: &'a str,
//     pub isbn: &'a str,
//     pub book_rating: &'a str,
//     pub genre: Option<&'a str>,
//     pub tags: Option<Vec<&'a str>>,
//     pub count: i32,
// }
