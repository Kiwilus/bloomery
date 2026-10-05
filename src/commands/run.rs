use anyhow::Result;
use std::process::Command;

use crate::commands::build::build_classpath;
use crate::config::load_config;

// run compiled java code
pub fn run() -> Result<()> {
    let config = load_config()?;

    /*
    // make sure all JARs exists before start
    check_jars(&config)?;

    if !config.dependencies.jars.is_empty() {
        info!(
            "Including {} dependenc{} in runtime classpath",
            config.dependencies.jars.len(),
            if config.dependencies.jars.len() == 1 {
                "y"
            } else {
                "ies"
            }
        );
    }
    */

    let classpath = build_classpath(&config);

    info!("Starting {} ...", config.paths.main_class);

    let status = match Command::new("java")
        .arg("-cp")
        .arg(&classpath)
        .arg(&config.paths.main_class)
        .status()
    {
        Ok(status) => status,
        Err(_) => {
            error!("java could not be started");
        }
    };

    if !status.success() {
        error!("Execution failed");
    }

    Ok(())
}
