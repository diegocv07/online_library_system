use std::env;

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use dotenvy::dotenv;

use crate::{
    api_handlers::filters::*,
    db::*,
    models::{Book, BookQuery, NewBook, ProtectedNewBook, UpdateBook},
    schema::books,
};

use diesel::prelude::*;

// CREATE
pub async fn add(
    Json(payload): Json<ProtectedNewBook>,
) -> Result<(StatusCode, Json<Book>), StatusCode> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("No existing key");
    if !(payload.key == api_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let payload = payload.new_book;

    let new_book = NewBook {
        title: payload.title,
        author: payload.author,
        isbn: payload.isbn,
        book_rating: payload.book_rating,
        genres: payload.genres,
        tags: payload.tags,
        description: payload.description,
        language: payload.language,
    };

    let conn = &mut establish_connection();

    let result = diesel::insert_into(books::table)
        .values(new_book)
        .returning(Book::as_returning())
        .get_result(conn);
    if let Ok(book) = result {
        Ok((StatusCode::CREATED, Json(book)))
    } else {
        println!("Error: {}", result.err().unwrap());
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn find_by_isbn(Path(book_isbn): Path<String>) -> Result<Json<Book>, StatusCode> {
    let connection = &mut establish_connection();

    let results = books::dsl::books
        .filter(books::isbn.eq(book_isbn))
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading book");

    if let Some(book) = results.get(0) {
        Ok(Json(book.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// READ
pub async fn list(Query(params): Query<BookQuery>) -> Json<Vec<Book>> {
    // println!("{params:?}");
    let connection = &mut establish_connection();

    let mut book_list = books::dsl::books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    book_list = filter_by_title(book_list, params.title);
    book_list = filter_by_author(book_list, params.author);
    book_list = filter_by_rating(book_list, params.rating);
    book_list = filter_by_language(book_list, params.language);
    book_list = filter_by_availability(book_list, params.available);
    book_list = filter_by_genres(book_list, params.genres);
    book_list = filter_by_tags(book_list, params.tags);

    Json(book_list)
}

// UPDATE
pub async fn update(
    Path(id): Path<u32>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, StatusCode> {
    let connection = &mut establish_connection();

    Err(StatusCode::NOT_IMPLEMENTED)
}

// DELETE
pub async fn delete(Path(id): Path<u32>) -> StatusCode {
    let connection = &mut establish_connection();

    StatusCode::NOT_IMPLEMENTED
}
