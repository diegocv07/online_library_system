use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    db::*,
    models::{Book, NewBook, UpdateBook},
    schema::books,
    state::AppState,
};

use diesel::prelude::*;

pub async fn add(State(state): State<AppState>, Json(payload): Json<NewBook>) -> StatusCode {
    let mut state_books = state.books.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let result = add_book(payload);

    if let Ok(added_book) = result {
        // this can probably be removed
        *next_id += 1;
        (*state_books).push(added_book.clone());
        StatusCode::CREATED
    } else {
        StatusCode::BAD_REQUEST
    }
}

pub async fn find_by_isbn(Path(book_isbn): Path<String>) -> Result<Json<Book>, StatusCode> {
    // let state_books = state.books.lock().unwrap();
    // state_books
    //     .iter()
    //     .find(|t| t.isbn == book_isbn)
    //     .map(|t| Json(t.clone()))
    //     .ok_or(StatusCode::NOT_FOUND);

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

pub async fn list(State(state): State<AppState>) -> Json<Vec<Book>> {
    // TODO
    panic!();
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, StatusCode> {
    // TODO
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    // TODO
    StatusCode::NOT_IMPLEMENTED
}
