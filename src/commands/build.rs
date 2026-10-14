use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use crate::config::{Config, load_config};
use crate::deps;

// finds all Java files in a directory.
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

// Convert a Java file path into expected class file path
fn expected_class_file(java_file: &Path, src_dir: &Path, class_dir: &Path) -> PathBuf {
    let relative = java_file.strip_prefix(src_dir).unwrap_or(java_file);
    let mut class_rel = relative.to_path_buf();
    class_rel.set_extension("class");
    class_dir.join(class_rel)
}

// Checks if a Java file must be compiled again
fn needs_recompile(java_file: &Path, class_file: &Path) -> Result<bool> {
    if !class_file.exists() {
        return Ok(true);
    }

    let java_mtime = fs::metadata(java_file)?
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let class_mtime = fs::metadata(class_file)?
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH);

    Ok(java_mtime > class_mtime)
}

// return only the Java files that need to be compiled
fn collect_files_to_compile(
    java_files: &[PathBuf],
    src_dir: &Path,
    class_dir: &Path,
) -> Result<Vec<PathBuf>> {
    let mut to_compile = Vec::new();

    for java in java_files {
        let class_file = expected_class_file(java, src_dir, class_dir);
        if needs_recompile(java, &class_file)? {
            to_compile.push(java.clone());
        }
    }

    Ok(to_compile)
}

// build the classpath for the compiler and dependencies
pub fn build_classpath(config: &Config) -> Result<String> {
    let mut entries = Vec::new();

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

// Builds the project by compiling Java source files.
pub fn build() -> Result<()> {
    let config = load_config()?;
    info!(
        "Building project '{}' v{}",
        config.project.name, config.project.version
    );

    check_jars(&config)?;

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

    let src_dir = Path::new("src");
    let class_dir = Path::new(&config.paths.class_dir);

    let java_files = find_java_files(src_dir)?;

    if java_files.is_empty() {
        error!("No .java file found at src");
    }

    fs::create_dir_all(class_dir)?;

    // Compile only new or modified source files
    let to_compile = collect_files_to_compile(&java_files, src_dir, class_dir)?;

    if to_compile.is_empty() {
        success!("Nothing to compile");
        return Ok(());
    }

    info!(
        "Compiling {} of {} source file(s)...",
        to_compile.len(),
        java_files.len()
    );

    let classpath = build_classpath(&config)?;

    // javac -cp <classpath> -d <class_dir> -encoding UTF-8 [--release N] <files>
    let mut cmd = Command::new(config.build.javac());
    cmd.arg("-cp")
        .arg(&classpath)
        .arg("-d")
        .arg(class_dir)
        .arg("-encoding")
        .arg("UTF-8");

    if let Some(ref release) = config.build.release {
        cmd.arg("--release").arg(release);
        info!("Targeting Java {}", release);
    }

    cmd.args(&to_compile);

    let status = match cmd.status() {
        Ok(status) => status,
        Err(_) => {
            error!("{} not found.", config.build.javac());
        }
    };

    if !status.success() {
        error!("Compilation failed");
    }

    success!(
        "Build successfully ({} file{})",
        to_compile.len(),
        if to_compile.len() == 1 { "" } else { "s" }
    );
    Ok(())
}
