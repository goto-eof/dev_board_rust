use config::{Config, ConfigError, File};
use log::debug;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub db_uri: String,
    pub server_port: u16,
}
impl Settings {
    pub fn init_configuration() -> Result<Self, ConfigError> {
        debug!("Initializing settings....");
        let environment = env::var("DEV_BOARD_ENV").unwrap_or_else(|_| "development".into());
        let filename = format!("configuration/{}", environment);
        debug!("loading setting file {}...", &filename);
        let settings = Config::builder()
            .add_source(File::with_name("configuration/default").required(true))
            .add_source(File::with_name(&filename).required(true))
            .build()?
            .try_deserialize();
        debug!("Settings loaded correctly: {:?}", settings);
        settings
    }
}
