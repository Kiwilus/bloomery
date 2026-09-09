use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

// struct for the bloomery.toml file
#[derive(Debug, Deserialize)]
pub struct Config {
    // project name
    pub name: String,

    // project version
    pub version: String,

    // main class
    pub main_class: String,

    // directory where the compilated classes are located
    #[serde(default = "default_class_dir")]
    pub class_dir: String,
}

// default class directory
fn default_class_dir() -> String {
    "target/classes".to_string()
}

pub fn load_config() -> Result<Config> {
    let content = fs::read_to_string("bloomery.toml")
        .context("bloomery.toml not found. Are you in the project directory?")?;
    let config: Config = toml::from_str(&content).context("bloomery.toml could not be read")?;
    Ok(config)
}
