mod database;
mod handlers;
mod models;

use crate::database::{AppState, Database};
use crate::handlers::get_classes;
use crate::handlers::post_classes::post_classes;

use axum::routing::{get, post};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let database = Database::Mock(vec![]);
    let state = Arc::new(Mutex::new(AppState { database }));

    let app = axum::Router::new()
        .route("/classes", get(get_classes))
        .route("/classes", post(post_classes))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
