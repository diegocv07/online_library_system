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
        return Err(StatusCode::FORBIDDEN);
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
    let conn = &mut establish_connection();

    let results = books::dsl::books
        .filter(books::isbn.eq(book_isbn))
        .select(Book::as_select())
        .load(conn)
        .expect("Error loading book");

    if let Some(book) = results.first() {
        Ok(Json(book.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get(Path(id): Path<i32>) -> Result<Json<Book>, StatusCode> {
    let conn = &mut establish_connection();

    let results = books::dsl::books
        .filter(books::id.eq(id))
        .select(Book::as_select())
        .load(conn)
        .expect("Error loading book");

    if let Some(book) = results.first() {
        Ok(Json(book.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// READ
pub async fn list(Query(params): Query<BookQuery>) -> Json<Vec<Book>> {
    // println!("{params:?}");
    let conn = &mut establish_connection();

    let mut book_list = books::dsl::books
        .select(Book::as_select())
        .load(conn)
        .expect("Error loading books");

    book_list = filter_by_title(book_list, params.title);
    book_list = filter_by_author(book_list, params.author);
    book_list = filter_by_rating(book_list, params.book_rating);
    book_list = filter_by_language(book_list, params.language);
    book_list = filter_by_availability(book_list, params.available);
    book_list = filter_by_genres(book_list, params.genres);
    book_list = filter_by_tags(book_list, params.tags);

    Json(book_list)
}

// UPDATE
pub async fn update(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBook>,
) -> Result<(StatusCode, Json<Book>), StatusCode> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("No existing key");
    if !(payload.key == api_key) {
        return Err(StatusCode::FORBIDDEN);
    }

    let conn = &mut establish_connection();
    let result = books::dsl::books
        .filter(books::id.eq(id))
        .select(Book::as_select())
        .load(conn)
        .expect("Error loading book");

    let mut book_to_edit;
    if result.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    } else {
        book_to_edit = result[0].clone();
    }

    if let Some(author) = payload.author {
        book_to_edit.author = author;
    }
    if let Some(book_rating) = payload.book_rating {
        book_to_edit.book_rating = book_rating;
    }
    if let Some(available) = payload.available {
        book_to_edit.available = available;
    }
    if let Some(count) = payload.count {
        book_to_edit.count = count;
    }
    if let Some(description) = payload.description {
        book_to_edit.description = description;
    }
    if let Some(genres) = payload.genres {
        book_to_edit.genres = genres;
    }
    if let Some(tags) = payload.tags {
        book_to_edit.tags = tags;
    }

    let updated_book = diesel::update(books::dsl::books.filter(books::id.eq(id)))
        .set((
            books::author.eq(book_to_edit.author),
            books::book_rating.eq(book_to_edit.book_rating),
            books::available.eq(book_to_edit.available),
            books::count.eq(book_to_edit.count),
            books::description.eq(book_to_edit.description),
            books::genres.eq(book_to_edit.genres),
            books::tags.eq(book_to_edit.tags),
        ))
        .returning(Book::as_returning())
        .get_result(conn);

    if let Ok(book) = updated_book {
        Ok((StatusCode::OK, Json(book)))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// DELETE
pub async fn delete(
    Path(id): Path<i32>,
    Json(key): Json<String>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("No existing key");
    if !(key == api_key) {
        return Err(StatusCode::FORBIDDEN);
    }

    let conn = &mut establish_connection();

    let result = diesel::delete(books::dsl::books.filter(books::id.eq(id)))
        .returning(Book::as_returning())
        .get_result(conn);

    if let Ok(book) = result {
        Ok((
            StatusCode::OK,
            Json(format!("Deleted \'{}\' from library", book.title)),
        ))
    } else {
        Err(StatusCode::GONE)
    }
}
