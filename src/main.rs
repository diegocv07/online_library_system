use crate::state::AppState;

mod api_handlers;
mod db;
mod models;
mod routes;
mod schema;
mod state;

#[tokio::main]

async fn main() {
    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind Tcp port");

    println!("taskr running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect("Unable to start axum server");
}
