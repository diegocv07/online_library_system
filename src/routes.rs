use crate::api_handlers::books;
use crate::state::AppState;
use axum::{Router, routing::get};

// TODO setup up api routes
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/books", get(books::list).post(books::add))
        .route(
            "/books/isbn/{value}",
            get(books::find_by_isbn)
                .delete(books::delete)
                .patch(books::update),
        )
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
