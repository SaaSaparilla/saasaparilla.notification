use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use std::env;
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

//TODO: replace with LazyLock once it is stabilized https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html
pub static SETTINGS: Lazy<OnceLock<Settings>> =
    Lazy::new(|| OnceLock::from(Settings::new().unwrap()));

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        //TODO: allow any option to be overridden pending https://github.com/mehcode/config-rs/issues/328
        //TODO: consider using figment+clap https://docs.rs/figment/latest/figment/#for-cli-application-authors
        let config_file_path = env::var("SAASAPARILLA_NOTIFICATION_RECEIVER_CONFIG_FILE_PATH")
            .or_else(|_| {
                env::args()
                    .find(|item| item.starts_with("--config-file-path="))
                    .and_then(|arg| Some(arg.split_once("=")?.1.to_string()))
                    .ok_or(ConfigError::NotFound("config_file_path".to_string()))
            })
            .or_else(|_| Ok("config.toml".to_string()))?;

        let config = Config::builder()
            .add_source(File::with_name(&config_file_path))
            .add_source(
                Environment::with_prefix("SAASAPARILLA_NOTIFICATION_RECEIVER")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()?;

        if config.get_bool("debug")? {
            println!("settings: {:?}", config);
        }

        config.try_deserialize()
    }
}
