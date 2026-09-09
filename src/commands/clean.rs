use crate::config::load_config;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn clean() -> Result<()> {
    // If a bloomery.toml exists, use its class_dir.
    // If not e.g. after a 'build-file' call, remove "bin" folder because it is hardcoded in build-file.
    let target_dir = match load_config() {
        Ok(config) => config.class_dir,
        Err(_) => "bin".to_string(),
    };

    let class_path = Path::new(&target_dir);

    if class_path.exists() {
        if fs::remove_dir_all(class_path).is_ok() {
            info!("Cleaned {}/", class_path.display());
        } else {
            error!("Failed to remove {}/", class_path.display());
        }
    } else {
        info!("Nothing to clean ({} does not exist)", class_path.display());
    }

    Ok(())
}
