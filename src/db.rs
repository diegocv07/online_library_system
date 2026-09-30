use crate::{
    models::{Book, NewBook},
    schema::books,
};
use axum::http::StatusCode;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

pub fn add_book(payload: NewBook) -> Result<Book, StatusCode> {
    let new_book = NewBook {
        title: payload.title,
        author: payload.author,
        isbn: payload.isbn,
        book_rating: payload.book_rating,
        genre: payload.genre,
        tags: payload.tags,
    };

    let conn = &mut establish_connection();

    diesel::insert_into(books::table)
        .values(new_book)
        .returning(Book::as_returning())
        .get_result(conn)
        .map_err(|_| StatusCode::BAD_REQUEST)
}
