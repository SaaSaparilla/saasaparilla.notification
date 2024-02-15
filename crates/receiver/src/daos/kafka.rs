use std::error::Error;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use once_cell::sync::Lazy;
use serde_json::json;

use saasaparilla_notification_common::types::notification::NotificationV1;

use crate::settings::SETTINGS;

/// The `PRODUCER` static variable is lazily initialized, so it should be called immediately on
/// startup before anything else is done.  It should be `unwrap()`ed at the call site to ensure that
/// the program crashes and exits if configuration cannot be loaded.
///
/// TODO: replace with [LazyLock](https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html) (and get rid of once_cell dependency) once it is stabilized
/// TODO: make `pub(crate)` once [RUST-10932](https://youtrack.jetbrains.com/issue/RUST-10932/False-negative-E0603-A-private-item-was-used-outside-its-scope) is fixed
pub static PRODUCER: Lazy<OnceLock<Mutex<Producer>>> = Lazy::new(|| {
    let settings = SETTINGS.get().unwrap();
    OnceLock::from(Mutex::from(
        Producer::from_hosts(vec![settings.kafka.hosts.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap(),
    ))
});

pub fn create_v1(notification: &NotificationV1) -> Result<(), Box<dyn Error>> {
    let settings = SETTINGS.get().ok_or("")?;
    let producer = PRODUCER.get().ok_or("")?;
    producer.lock()?.send(&Record::from_value(
        &settings.kafka.topic,
        json!(notification).to_string(),
    ))?;
    Ok(())
}
