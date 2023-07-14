#![warn(clippy::all)]

use axum::{Router, routing::{get, post}};

mod routes;
mod types;

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(routes::health))
        .route("/api/v1/notification", post(routes::notification::create_v1))
}
