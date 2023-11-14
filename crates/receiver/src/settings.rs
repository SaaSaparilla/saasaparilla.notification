use once_cell::sync::Lazy;
use saasaparilla_notification_common::util::settings;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub network_interface: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Kafka {
    pub url: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub kafka: Kafka,
    pub server: Server,
}

//TODO: replace with LazyLock (and get rid of once_cell dependency) once it is stabilized
// https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html
#[allow(unused)]
pub static SETTINGS: Lazy<OnceLock<Settings>> = Lazy::new(|| {
    OnceLock::from(
        settings::retrieve_config::<Settings>("SAASAPARILLA_NOTIFICATION_RECEIVER").unwrap(),
    )
});
