use axum::http::StatusCode;
use axum::{extract, Json};

use saasaparilla_notification_common::types::notification::NotificationV1;

// TODO: phase 1 - http request to director
// TODO: phase 2 - push to kafka topic
pub(crate) async fn create_v1(
    extract::Json(payload): Json<NotificationV1>,
) -> Result<Json<NotificationV1>, StatusCode> {
    println!("{:?}", payload);
    Ok(Json(payload))
}
