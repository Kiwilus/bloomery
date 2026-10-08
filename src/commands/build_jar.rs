use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::commands::build::build;
use crate::config::load_config;
use crate::deps;

/// Package the project into a fat JAR (classes + all dependencies).
pub fn package(output_dir: &Path) -> Result<()> {
    // build first so the class files exist and ensure that managed FARs are downloaded
    build()?;

    let config = load_config()?;
    let dependency_jars = deps::resolve_classpath_jars(&config)?;

    fs::create_dir_all(output_dir)?;

    let jar_name = format!("{}.jar", config.project.name);
    let output_jar = output_dir.join(&jar_name);
    let class_dir = Path::new(&config.paths.class_dir);

    // temp directory for fat jar
    let temp_dir = PathBuf::from(".bloomery_package_tmp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // copy own class files into temp
    copy_dir_all(class_dir, &temp_dir)?;

    // unpack external jars and include them
    if !dependency_jars.is_empty() {
        let n = dependency_jars.len();
        info!(
            "adding {} external jar{}...",
            n,
            if n == 1 { "" } else { "s" }
        );

        for jar in &dependency_jars {
            info!("extracting {jar} ...");

            let jar_path = Path::new(jar)
                .canonicalize()
                .map_err(|_| error!("could not find jar: {jar}"))?;

            // jar xf <jar_path> <temp_dir>
            let status = Command::new("jar")
                .arg("xf")
                .arg(&jar_path)
                .current_dir(&temp_dir)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => {
                    error!("could not extract jar: {jar}");
                }
            }
        }
    }

    info!("creating {} ...", output_jar.display());

    // jar cfe <output_jar> <main_class> -C <temp_dir> .
    let status = Command::new("jar")
        .arg("cfe")
        .arg(&output_jar)
        .arg(&config.paths.main_class)
        .arg("-C")
        .arg(&temp_dir)
        .arg(".")
        .status();

    let _ = fs::remove_dir_all(&temp_dir);

    match status {
        Ok(s) if s.success() => {
            success!("Package created: {}", output_jar.display());
            if !dependency_jars.is_empty() {
                info!("(fat jar with all dependencies included)");
            }
        }
        _ => {
            error!("failed to create jar.");
        }
    }

    Ok(())
}

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
