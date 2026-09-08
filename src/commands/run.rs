use anyhow::Result;
use std::process::Command;

use crate::config::load_config;

// run compiled java code
pub fn run() -> Result<()> {
    let config = load_config()?;
    info!("Starting {} ...", config.main_class);

    let status = match Command::new("java")
        .arg("-cp")
        .arg("target/classes")
        .arg(&config.main_class)
        .status()
    {
        Ok(status) => status,
        Err(_) => {
            error!("java could not be started")
        }
    };

    if !status.success() {
        error!("Execution failed");
    }

    Ok(())
}
