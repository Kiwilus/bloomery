use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::config::load_config;

// runs a jar, either the one from the package command or one you specify
pub fn run_jar(jar: Option<String>) -> Result<()> {
    let jar_path = match jar {
        Some(j) => j,
        None => {
            let config = load_config()?;
            format!("{}.jar", config.project.name)
        }
    };

    if !Path::new(&jar_path).exists() {
        error!("jar not found: {}", jar_path);
    }

    info!("starting {} ...", jar_path);

    let status = match Command::new("java").arg("-jar").arg(&jar_path).status() {
        Ok(s) => s,
        Err(_) => {
            error!("could not start java");
        }
    };

    if !status.success() {
        error!("jar crashed / failed");
    }

    Ok(())
}
