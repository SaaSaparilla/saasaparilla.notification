use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use saasaparilla_notification_receiver::app;

#[tokio::test]
async fn health() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap())
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    assert!(response.into_body().data().await.is_none());
}
