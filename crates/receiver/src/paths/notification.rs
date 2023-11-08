use axum::routing::{get, post};
use axum::Router;

use crate::routes;

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(routes::health))
        // TODO: add appropriate request validation to k8s Ingress and CiliumNetworkPolicy objects
        .route(
            "/api/v1/notification",
            post(routes::notification::create_v1),
        )
}
