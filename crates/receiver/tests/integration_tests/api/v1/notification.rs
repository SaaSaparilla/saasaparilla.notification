use axum::{
    body::{Body, HttpBody},
    http::{self, Request, StatusCode},
};
use tower::ServiceExt;
use saasparilla_notification_receiver::app;

#[tokio::test]
async fn create_notification_v1() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/notification")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from("{\"notification_emitter_id\":\"emitter_id\",\"notification_recipient_id\":\"recipient_id\",\"notification_delivery_semantics\":\"AT_LEAST_ONCE\",\"notification_retries_remaining\":3,\"notification_content_type\":\"content_type\",\"notification_content\":\"content\"}"))
                .unwrap())
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(b"{\"notification_emitter_id\":\"emitter_id\",\"notification_recipient_id\":\"recipient_id\",\"notification_delivery_semantics\":\"AT_LEAST_ONCE\",\"notification_retries_remaining\":3,\"notification_content_type\":\"content_type\",\"notification_content\":\"content\"}", &(response.into_body().data().await.unwrap().unwrap())[..]);
}
