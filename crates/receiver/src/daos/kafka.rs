use std::error::Error;
use std::sync::{LazyLock, Mutex, OnceLock};
use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use once_cell::sync::Lazy;
use serde_json::json;

use saasaparilla_notification_common::types::notification::NotificationV1;

use crate::SETTINGS;

/// The `PRODUCER` static variable is lazily initialized, so it should be instantiated with
/// `assert!(panic::catch_unwind(|| &*PRODUCER).is_ok());` before anything else is done
pub(crate) static PRODUCER: LazyLock<Mutex<Producer>> = LazyLock::new(|| {
    Mutex::from(
        Producer::from_hosts(vec![(&SETTINGS).kafka.hosts.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap(),
    )
});

pub fn create_v1(notification: &NotificationV1) -> Result<(), Box<dyn Error>> {
    let settings = &SETTINGS;
    let producer = &PRODUCER;
    producer.lock()?.send(&Record::from_value(
        &settings.kafka.topic,
        json!(notification).to_string(),
    ))?;
    Ok(())
}
