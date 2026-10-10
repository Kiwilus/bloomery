use anyhow::Result;
use std::path::Path;

use crate::config::{load_config, save_config};

// Add or update a managed dependency
// if no version in the CLI is given, bloomery will search up the newest version and use it.
pub fn add(name: &str, version: Option<&str>) -> Result<()> {
    let mut config = load_config()?;

    let version = match version {
        Some(v) => v.to_string(),
        None => {
            info!("Resolving latest version for '{name}' ...");
            crate::deps::resolve_latest_version(name)?
        }
    };

    if let Some(old) = config.dependencies.managed.get(name) {
        info!("Updating dependency '{name}' from {old} to {version}");
    }

    config
        .dependencies
        .managed
        .insert(name.to_string(), version.clone());
    save_config(&config)?;
    success!("Added managed dependency {name} = \"{version}\"");
    Ok(())
}

// Remove a managed dependency
pub fn remove(name: &str) -> Result<()> {
    let mut config = load_config()?;

    if config.dependencies.managed.remove(name).is_none() {
        error!("Managed dependency '{}' not found", name);
    }

    save_config(&config)?;
    info!("Removed managed dependency '{}'", name);
    Ok(())
}

// list all managed dependencies
pub fn list() -> Result<()> {
    let config = load_config()?;

    if config.dependencies.managed.is_empty() {
        info!("No managed dependencies");
        return Ok(());
    }

    // don't like the INFO thing in this case
    println!("Managed dependencies:");

    // sort dependencies alphabetically by name.
    let mut entries: Vec<_> = config.dependencies.managed.iter().collect();
    entries.sort_by_key(|(k, _)| *k);

    for (name, version) in entries {
        println!("  {} = \"{}\"", name, version);
    }
    Ok(())
}

// Add a local JAR
pub fn add_local(path: &str) -> Result<()> {
    let mut config = load_config()?;

    // check, that the JAR file exists
    if !Path::new(path).exists() {
        error!("JAR not found: {}", path);
    }

    // avoid adding the same dependency twice
    if config.dependencies.local.jars.iter().any(|j| j == path) {
        info!("Local dependency '{}' already present", path);
        return Ok(());
    }

    config.dependencies.local.jars.push(path.to_string());
    save_config(&config)?;
    info!("Added local dependency '{}'", path);
    Ok(())
}

// Remove a local JAR
pub fn remove_local(path: &str) -> Result<()> {
    let mut config = load_config()?;

    let before = config.dependencies.local.jars.len();

    // Remove the specified path from the list.
    config.dependencies.local.jars.retain(|j| j != path);

    if config.dependencies.local.jars.len() == before {
        error!("Local dependency '{}' not found", path);
    }

    save_config(&config)?;
    info!("Removed local dependency '{}'", path);
    Ok(())
}

// List local JARs
pub fn list_local() -> Result<()> {
    let config = load_config()?;

    if config.dependencies.local.jars.is_empty() {
        info!("No local dependencies");
        return Ok(());
    }

    // don't like the INFO thing in this use case
    println!("Local dependencies:");

    // mark JARs that no longer exist
    for jar in &config.dependencies.local.jars {
        let status = if Path::new(jar).exists() {
            ""
        } else {
            "  (missing)"
        };
        println!("  {}{}", jar, status);
    }
    Ok(())
}
