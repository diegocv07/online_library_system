use crate::api_handlers::books;
use crate::state::AppState;
use axum::{
    Router,
    http::{StatusCode, Uri},
    routing::get,
};

// TODO setup up api routes
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/books", get(books::list).post(books::add))
        .route(
            "/books/{value}",
            get(books::get).delete(books::delete).put(books::update),
        )
        .route("/books/isbn/{value}", get(books::find_by_isbn))
        .fallback(fallback)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
