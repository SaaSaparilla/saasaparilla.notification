use axum::Router;
use axum::routing::{get, post};

use crate::routes;

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(routes::health))
        .route("/api/v1/notification", post(routes::notification::create_v1))
}