use crate::models::Book;
use crate::schema::books::dsl::books;
use crate::{db::establish_connection, schema::books::id};
use diesel::dsl::max;
use diesel::prelude::*;
use std::option::Option;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub books: Arc<Mutex<Vec<Book>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let connection = &mut establish_connection();

        let results = books
            .select(max(id))
            .load::<Option<i32>>(connection)
            .expect("Error loading id");

        let max_id = results[0].unwrap_or(0);

        let seed = vec![Book {
            id: max_id + 1,
            title: "Game Programming Patterns".to_owned(),
            author: "Robert Nystrom".to_owned(),
            isbn: "978-0990582908".to_owned(),
            book_rating: 2,
            genre: Some("Education".to_owned()),
            tags: vec![],
            available: false,
            count: 0,
        }];

        AppState {
            next_id: Arc::new(Mutex::new(max_id as u32 + 1 + seed.len() as u32)),
            books: Arc::new(Mutex::new(seed)),
        }
    }
}
