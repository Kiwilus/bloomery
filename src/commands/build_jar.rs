use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::commands::build::{build, check_jars};
use crate::config::load_config;

// packages everything into a jar, including jars from the toml
pub fn package() -> Result<()> {
    // build first so the class files exist
    build()?;

    let config = load_config()?;
    check_jars(&config)?;

    let jar_name = format!("{}.jar", config.project.name);
    let class_dir = Path::new(&config.paths.class_dir);

    if !class_dir.exists() {
        error!(
            "class dir '{}' does not exist, did you build already?",
            config.paths.class_dir
        );
    }

    // temp directory for fat jar
    let temp_dir = PathBuf::from(".bloomery_package_tmp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // copy own class files into temp
    copy_dir_all(class_dir, &temp_dir)?;

    // unpack external jars and include them
    if !config.dependencies.jars.is_empty() {
        info!("adding {} external jars...", config.dependencies.jars.len());

        for jar in &config.dependencies.jars {
            info!("extracting {} ...", jar);

            // make path absolute so it still works when we change current_dir
            let jar_path = Path::new(jar)
                .canonicalize()
                .map_err(|_| error!("could not find jar: {}", jar))?;

            let status = Command::new("jar")
                .arg("xf")
                .arg(&jar_path)
                .current_dir(&temp_dir)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => {
                    error!("could not extract jar: {}", jar);
                }
            }
        }
    }

    // create the final jar
    info!("creating {} ...", jar_name);

    let status = Command::new("jar")
        .arg("cfe")
        .arg(&jar_name)
        .arg(&config.paths.main_class)
        .arg("-C")
        .arg(&temp_dir)
        .arg(".")
        .status();

    // clean up temp directory
    let _ = fs::remove_dir_all(&temp_dir);

    match status {
        Ok(s) if s.success() => {
            success!("Package created: {}", jar_name);
            if !config.dependencies.jars.is_empty() {
                info!("(fat jar with all dependencies included)");
            }
        }
        _ => {
            error!("failed to create jar. is 'jar' in your PATH?");
        }
    }

    Ok(())
}

// simple recursive copy
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
