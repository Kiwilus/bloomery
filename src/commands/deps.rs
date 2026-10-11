use anyhow::Result;
use std::fs;
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
        if old == &version {
            warn!("'{name}' = \"{version}\" is already in bloomery.toml");
            return Ok(());
        }
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
pub fn remove(name: &str, delete_jar: bool) -> Result<()> {
    let mut config = load_config()?;

    let Some(version) = config.dependencies.managed.remove(name) else {
        error!("Dependency '{}' not found", name);
    };

    save_config(&config)?;
    success!("Removed dependency '{}'", name);

    if delete_jar {
        let jar_path = Path::new("lib").join(format!("{name}-{version}.jar"));
        if jar_path.exists() {
            fs::remove_file(&jar_path)?;
            info!("Deleted {}", jar_path.display());
        } else {
            info!("JAR not found: {}", jar_path.display());
        }
    }

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
        warn!("Local dependency '{}' is already in bloomery.toml", path);
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

pub fn add_clean_dir(dirs: &[String]) -> Result<()> {
    let mut config = load_config()?;

    for dir in dirs {
        if config.paths.clean_dirs.iter().any(|d| d == dir) {
            warn!("'{}' is already in clean_dirs", dir);
            continue;
        }
        config.paths.clean_dirs.push(dir.clone());
        success!("Added clean_dir '{}'", dir);
    }

    save_config(&config)?;
    Ok(())
}

pub fn remove_clean_dir(dirs: &[String]) -> Result<()> {
    let mut config = load_config()?;

    for dir in dirs {
        let before = config.paths.clean_dirs.len();
        config.paths.clean_dirs.retain(|d| d != dir);
        if config.paths.clean_dirs.len() == before {
            error!("clean_dir '{}' not found", dir);
        }
        success!("Removed clean_dir '{}'", dir);
    }

    save_config(&config)?;
    Ok(())
}

pub fn list_clean_dirs() -> Result<()> {
    let config = load_config()?;

    if config.paths.clean_dirs.is_empty() {
        info!("No clean_dirs configured");
        return Ok(());
    }

    println!("clean_dirs:");
    for dir in &config.paths.clean_dirs {
        println!("  {}", dir);
    }
    Ok(())
}
