use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    db::*,
    models::{Book, NewBook, UpdateBook},
    schema::books,
    state::AppState,
};

use diesel::prelude::*;

pub async fn add(Json(payload): Json<NewBook>) -> Result<(StatusCode, Json<Book>), StatusCode> {
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

// lists books in the database and filters them if a correct query is given
pub async fn list(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Book>> {
    if params.is_empty() {
        println!("No query was given");
    }
    let connection = &mut establish_connection();

    // SET FILTERS
    let filter_title = params.get("title").map_or(None, |title| {
        Some(
            title
                .to_ascii_lowercase()
                .split('_')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        )
    });

    let filter_author = params.get("author").map_or(None, |author| {
        Some(author.to_ascii_lowercase().replace("_", " "))
    });

    let filter_rating = params
        .get("rating")
        .map_or(None, |rating| Some(rating.parse::<i32>().unwrap_or(0)));

    let filter_genre = params.get("genre").map_or(None, |genre| {
        Some(
            genre
                .to_ascii_lowercase()
                .split(',')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        )
    });

    let filter_tags = params.get("tags").map_or(None, |tags| {
        Some(
            tags.to_ascii_lowercase()
                .split(',')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        )
    });

    let filter_available = params
        .get("available")
        .map_or(None, |avail| Some(avail.parse::<bool>().unwrap_or(true)));

    let mut book_list = books::dsl::books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    // APPLY FILTERS
    if let Some(filter) = filter_title {
        for i in &filter {
            println!("title filtered by: {}", i);
        }
        book_list = book_list
            .into_iter()
            .filter(|b| {
                let binding = b.title.to_ascii_lowercase();
                let title_words: Vec<&str> = binding
                    .split(' ')
                    .filter(|&word| filter.contains(&word.to_owned().to_ascii_lowercase()))
                    .collect();
                title_words.len() == filter.len()
            })
            .collect();
    }
    if let Some(filter) = filter_author {
        book_list = book_list
            .into_iter()
            .filter(|b| b.author.to_ascii_lowercase() == *filter)
            .collect();
    }
    if let Some(filter) = filter_rating {
        book_list = book_list
            .into_iter()
            .filter(|b| b.book_rating == filter)
            .collect();
    }
    if let Some(filter) = filter_genre {
        book_list = book_list
            .into_iter()
            .filter(|b| {
                b.genre.is_some()
                    && filter.contains(&b.genre.as_ref().unwrap().to_ascii_lowercase())
            })
            .collect();
    }
    if let Some(filter) = filter_tags {
        book_list = book_list
            .into_iter()
            .filter(|b| {
                let matched_tags: Vec<&Option<String>> = b
                    .tags
                    .iter()
                    .filter(|&tag| {
                        tag.is_some()
                            && filter.contains(&tag.as_ref().unwrap().to_ascii_lowercase())
                    })
                    .collect();
                matched_tags.len() == filter.len()
            })
            .collect();
    }
    if let Some(filter) = filter_available {
        book_list = book_list
            .into_iter()
            .filter(|b| b.available == filter)
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
