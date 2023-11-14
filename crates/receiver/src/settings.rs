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
    pub host: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub kafka: Kafka,
    pub server: Server,
}

/// The `SETTINGS` static variable is lazily initialized, so it should be called immediately on
/// startup before anything else is done.  It should be `unwrap()`ed at the call site to ensure that
/// the program crashes and exits if configuration cannot be loaded.
///
/// TODO: replace with [LazyLock](https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html) (and get rid of once_cell dependency) once it is stabilized
///
/// TODO: make `pub(crate)` once [RUST-10932](https://youtrack.jetbrains.com/issue/RUST-10932/False-negative-E0603-A-private-item-was-used-outside-its-scope) is fixed
#[allow(unused)]
pub static SETTINGS: Lazy<OnceLock<Settings>> = Lazy::new(|| {
    OnceLock::from(
        settings::retrieve_config::<Settings>("SAASAPARILLA_NOTIFICATION_RECEIVER").unwrap(),
    )
});
