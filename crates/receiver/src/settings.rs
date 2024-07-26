use std::sync::LazyLock;

use serde::Deserialize;

use saasaparilla_notification_common::util::settings;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub network_interface: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Kafka {
    pub hosts: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub kafka: Kafka,
    pub server: Server,
}

/// The `SETTINGS` static variable is lazily initialized, so it should be instantiated with
/// `assert!(panic::catch_unwind(|| &*SETTINGS).is_ok());` before anything else is done
pub(crate) static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    settings::retrieve_config::<Settings>("SAASAPARILLA_NOTIFICATION_RECEIVER").unwrap()
});
