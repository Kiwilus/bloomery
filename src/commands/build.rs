use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::{Config, load_config};
use crate::deps;

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
        } else if path.extension().is_some_and(|ext| ext == "java") {
            files.push(path);
        }
    }
    Ok(files)
}

// build function to compile java code and dependency support
pub fn build_classpath(config: &Config) -> Result<String> {
    let mut entries = Vec::new();

    // Put the output directory first
    entries.push(config.paths.class_dir.clone());
    entries.extend(deps::resolve_classpath_jars(config)?);

    // ; on windows and : not on windows
    let separator = if cfg!(windows) { ";" } else { ":" };
    Ok(entries.join(separator))
}

// checks that all specified JARs exist
pub fn check_jars(config: &Config) -> Result<()> {
    for jar in &config.dependencies.local.jars {
        if !Path::new(jar).exists() {
            error!("Dependency JAR not found: {}", jar);
        }
    }
    Ok(())
}

pub fn build() -> Result<()> {
    let config = load_config()?;
    info!(
        "Building project '{}' v{}",
        config.project.name, config.project.version
    );

    // Check dependencies before compiling anything
    check_jars(&config)?;

    // checks if 1 or many dependencies
    if !config.dependencies.local.jars.is_empty() {
        info!(
            "Including {} dependenc{} in classpath",
            config.dependencies.local.jars.len(),
            if config.dependencies.local.jars.len() == 1 {
                "y"
            } else {
                "ies"
            }
        );
    }

    // Find Java source files
    let src_dir = Path::new("src");
    let java_files = find_java_files(src_dir)?;

    if java_files.is_empty() {
        error!("No .java file found at src");
    }

    fs::create_dir_all(&config.paths.class_dir)?;

    // build the classpath
    let classpath = build_classpath(&config)?;

    // javac -cp <classpath> -d <class_dir> -encoding UTF-8 <java_files>
    let status = match Command::new("javac")
        .arg("-cp")
        .arg(&classpath)
        .arg("-d")
        .arg(&config.paths.class_dir)
        .arg("-encoding")
        .arg("UTF-8")
        .args(&java_files)
        .status()
    {
        Ok(status) => status,
        Err(_) => {
            error!("javac not found.");
        }
    };

    if !status.success() {
        error!("Compilation failed");
    }

    success!("Build successfully ({} files)", java_files.len());
    Ok(())
}
