use anyhow::Result;
use std::fs;

use crate::config::{Config, load_config};

// set bloomery.toml values via CLI
// e.g. blm set main_class Main2.java
pub fn set(key: &str, new_value: &str) -> Result<()> {
    let mut config = load_config()?;

    match key {
        "version" => config.project.version = new_value.to_string(),
        "name" => config.project.name = new_value.to_string(),
        "main_class" => config.paths.main_class = new_value.to_string(),
        "class_dir" => config.paths.class_dir = new_value.to_string(),

        _ => error!(
            "Unknown key '{}'. Supported: version, name, main_class, class_dir",
            key
        ),
    }

    save_config(&config)?;
    info!("Set {} = {}", key, new_value);
    Ok(())
}

fn save_config(config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write("bloomery.toml", content)?;
    Ok(())
}
