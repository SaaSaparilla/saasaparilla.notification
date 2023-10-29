use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NotificationV1 {
    notification_emitter_id: String,
    notification_recipient_id: String,
    // TODO: enum AT_LEAST_ONCE,AT_MOST_ONCE,EXACTLY_ONCE
    notification_delivery_semantics: String,
    notification_retries_remaining: isize,
    notification_content_type: String,
    notification_content: String,
}
