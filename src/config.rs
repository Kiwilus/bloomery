use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/*
 * Part for the bloomery.toml configuration
 */
// struct for the bloomery.toml file
#[derive(Debug, Deserialize)]
pub struct Config {
    // project name
    pub name: String,

    // project version
    pub version: String,

    // main class
    pub main_class: String,

    // directory where the compiled classes are located
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

/*
 * Part for global configuration of the default template in bloomery/config.toml
 */
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    #[serde(default = "default_template")]
    pub default_template: String,
}

fn default_template() -> String {
    "default".to_string()
}

// Return the path to the global config file
pub fn global_config_path() -> anyhow::Result<PathBuf> {
    let proj = ProjectDirs::from("", "", "bloomery")
        .ok_or_else(|| error!("Could not determine config directory"))?;
    Ok(proj.config_dir().join("config.toml"))
}

// load config.toml
pub fn load_global_config() -> anyhow::Result<GlobalConfig> {
    let path = global_config_path()?;
    if !path.exists() {
        return Ok(GlobalConfig::default());
    }
    let content = fs::read_to_string(&path)?;
    Ok(toml::from_str(&content)?)
}

// write global config to the disk
pub fn save_global_config(cfg: &GlobalConfig) -> anyhow::Result<()> {
    let path = global_config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, toml::to_string_pretty(cfg)?)?;
    Ok(())
}
