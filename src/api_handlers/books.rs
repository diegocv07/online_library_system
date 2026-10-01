use std::env;

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use dotenvy::dotenv;

use crate::{
    db::*,
    models::{Book, BookQuery, NewBook, ProtectedNewBook, UpdateBook},
    schema::books,
};

use diesel::prelude::*;

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
        genre: payload.genre,
        tags: payload.tags,
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

pub async fn list(Query(params): Query<BookQuery>) -> Json<Vec<Book>> {
    // println!("{params:?}");
    let connection = &mut establish_connection();

    let mut book_list = books::dsl::books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    if let Some(title) = params.title {
        let word_filter: Vec<String> = title.split("_").map(|word| word.to_owned()).collect();
        book_list = book_list
            .into_iter()
            .filter(|b| {
                let binding = b.title.to_ascii_lowercase();
                let title_words: Vec<&str> = binding
                    .split(' ')
                    .filter(|&word| word_filter.contains(&word.to_owned().to_ascii_lowercase()))
                    .collect();
                title_words.len() == word_filter.len()
            })
            .collect();
    }
    if let Some(author) = params.author {
        let author = author.replace("_", " ");
        book_list = book_list
            .into_iter()
            .filter(|b| b.author.to_ascii_lowercase() == *author)
            .collect();
    }
    if let Some(rating) = params.rating {
        book_list = book_list
            .into_iter()
            .filter(|b| b.book_rating == rating)
            .collect();
    }
    if let Some(genres) = params.genres {
        let genres: Vec<String> = genres.split(",").map(|word| word.to_owned()).collect();
        book_list = book_list
            .into_iter()
            .filter(|b| {
                b.genre.is_some()
                    && genres.contains(&b.genre.as_ref().unwrap().to_ascii_lowercase())
            })
            .collect();
    }
    if let Some(tags) = params.tags {
        let tags: Vec<String> = tags.split(",").map(|word| word.to_owned()).collect();
        book_list = book_list
            .into_iter()
            .filter(|b| {
                let matched_tags: Vec<&Option<String>> = b
                    .tags
                    .iter()
                    .filter(|&tag| {
                        tag.is_some() && tags.contains(&tag.as_ref().unwrap().to_ascii_lowercase())
                    })
                    .collect();
                matched_tags.len() == tags.len()
            })
            .collect();
    }
    if let Some(available) = params.available {
        book_list = book_list
            .into_iter()
            .filter(|b| b.available == available)
            .collect();
    }

    Json(book_list)
}

pub async fn update(
    Path(id): Path<u32>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, StatusCode> {
    let connection = &mut establish_connection();

    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete(Path(id): Path<u32>) -> StatusCode {
    let connection = &mut establish_connection();

    StatusCode::NOT_IMPLEMENTED
}
