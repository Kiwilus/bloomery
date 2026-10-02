use anyhow::Result;
use std::path::Path;
use std::process::Command;

// When you use build_file and dont parse an output directory with '--output' after 'build-file',
// a 'bin' directory will be created in the current working directory where the compiled files are stored.
// If you don't want to create a output directory, use the run-file command instead. This will run the Java file directly,
// without creating a output directory.
pub fn build_file(path: &Path, output_dir: &Path) -> Result<()> {
    if !path.exists() {
        error!("File not found at: {}", path.display());
    }

    std::fs::create_dir_all(output_dir)?;

    // The build-file command does not require a bloomery.toml file,
    let status = match Command::new("javac")
        .arg("-d")
        .arg(output_dir)
        .arg("-encoding")
        .arg("UTF-8")
        .arg(path)
        .status()
    {
        Ok(status) => status,
        Err(_) => error!("javac not found."),
    };

    if !status.success() {
        error!("Compilation failed");
    }

    success!("File built successfully");
    Ok(())
}
