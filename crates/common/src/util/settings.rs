use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::fmt::Debug;

pub fn retrieve_config<T: Debug + for<'de> Deserialize<'de>>(
    prefix: &str,
) -> Result<T, ConfigError> {
    //TODO: allow any option to be overridden pending https://github.com/mehcode/config-rs/issues/328
    //TODO: consider using figment+clap https://docs.rs/figment/latest/figment/#for-cli-application-authors
    let config_file_path = env::var(format!("{}_CONFIG_FILE_PATH", prefix))
        .or_else(|_| {
            env::args()
                .find(|item| item.starts_with("--config-file-path="))
                .and_then(|arg| Some(arg.split_once('=')?.1.to_string()))
                .ok_or(ConfigError::NotFound("config_file_path".to_string()))
        })
        .or_else(|_| Ok("config.toml".to_string()))?;

    let config = Config::builder()
        .add_source(File::with_name(&config_file_path))
        .add_source(
            Environment::with_prefix(prefix)
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
