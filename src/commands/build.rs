use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::load_config;

fn find_java_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_java_files(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "java") {
            files.push(path);
        }
    }
    Ok(files)
}

// build function to compile java code
pub fn build() -> Result<()> {
    let config = load_config()?;
    println!("Building project '{}' v{}", config.name, config.version);

    let src_dir = Path::new("src");
    let java_files = find_java_files(src_dir)?;

    if java_files.is_empty() {
        bail!("No .java file found at src/");
    }

    fs::create_dir_all("target/classes")?;

    let status = Command::new("javac")
        .arg("-d")
        .arg("target/classes")
        .arg("-encoding")
        .arg("UTF-8")
        .args(&java_files)
        .status()
        .context("javac not found.")?;

    if !status.success() {
        bail!("Compilation failed");
    }

    println!("Build successfully ({} files)", java_files.len());
    Ok(())
}
