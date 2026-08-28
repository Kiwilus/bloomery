use anyhow::{Context, Result, bail};
use std::process::Command;

use crate::config::load_config;

// compiles java code and runs it directly
pub fn run() -> Result<()> {
    let config = load_config()?;
    println!("Starting {} ...", config.main_class);

    let status = Command::new("java")
        .arg("-cp")
        .arg("target/classes")
        .arg(&config.main_class)
        .status()
        .context("java could not be started")?;

    if !status.success() {
        bail!("Execution failed");
    }

    Ok(())
}
