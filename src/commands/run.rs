use anyhow::Result;
use std::process::Command;

use crate::config::load_config;

// compiles java code and runs it directly
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
            crate::error!("java could not be started");
            std::process::exit(1);
        }
    };

    if !status.success() {
        error!("Execution failed");
        std::process::exit(1);
    }

    Ok(())
}
