use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

use super::build_file;
use crate::config::load_config;

// Read the package name from a Java file if present
fn get_full_name(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    let class_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| error!("Invalid Java file name"))?;

    for line in content.lines() {
        let line = line.trim();

        // Find the package declaration
        if line.starts_with("package ") && line.ends_with(';') {
            let package = line
                .trim_start_matches("package ")
                .trim_end_matches(';')
                .trim();

            return Ok(format!("{}.{}", package, class_name));
        }

        // stop once the code begins
        if !line.is_empty()
            && !line.starts_with("//")
            && !line.starts_with("/*")
            && !line.starts_with('*')
        {
            break;
        }
    }

    // when no package found use class name
    Ok(class_name.to_string())
}

pub fn run_file(path: &Path) -> Result<()> {
    // First build the file
    build_file::build_file(path)?;

    let config = load_config()?;
    let fqn = get_full_name(path)?;

    info!("Starting {} ...", fqn);

    // java command to run .class: java -cp <class_dir> <full_name>
    let status = match Command::new("java")
        .arg("-cp")
        .arg(&config.class_dir)
        .arg(&fqn)
        .status()
    {
        Ok(status) => status,
        Err(_) => error!("Java could not be started"),
    };

    if !status.success() {
        error!("Execution failed");
    }

    Ok(())
}
