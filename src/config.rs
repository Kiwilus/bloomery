use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

// struct for the bloomery.toml file
#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub main_class: String,
}

pub fn load_config() -> Result<Config> {
    let content = fs::read_to_string("bloomery.toml")
        .context("bloomery.toml not found. Are you in the project directory?")?;
    let config: Config = toml::from_str(&content).context("bloomery.toml could not be read")?;
    Ok(config)
}
