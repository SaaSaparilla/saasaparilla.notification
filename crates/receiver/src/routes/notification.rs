use axum::http::StatusCode;
use axum::Json;

use crate::daos::kafka;

use saasaparilla_notification_common::types::notification::NotificationV1;

// TODO: push to kafka topic
pub(crate) async fn create_v1(
    Json(payload): Json<NotificationV1>,
) -> Result<Json<NotificationV1>, StatusCode> {
    println!("{:?}", payload);
    kafka::create_v1(&payload).map_err(|err| {
        println!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(payload))
}
