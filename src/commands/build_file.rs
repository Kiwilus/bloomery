use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::config::load_config;

pub fn build_file(path: &Path) -> Result<()> {
    let file = Path::new(path);

    if !file.exists() {
        error!("File not found at: {}", path.display());
    }

    let config = load_config()?;

    std::fs::create_dir_all(&config.class_dir)?;

    let status = match Command::new("javac")
        .arg("-d")
        .arg(&config.class_dir) // no hard coded bin/ directory
        .arg("-encoding")
        .arg("UTF-8")
        .arg(file)
        .status()
    {
        Ok(status) => status,
        Err(_) => error!("javac not found."),
    };

    if !status.success() {
        error!("Compilation failed");
    }

    success!("File build successfully");
    Ok(())
}
