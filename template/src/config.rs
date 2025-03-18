use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
