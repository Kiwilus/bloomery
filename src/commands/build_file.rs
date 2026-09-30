use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn build_file(path: &Path) -> Result<()> {
    let file = Path::new(path);

    if !file.exists() {
        error!("File not found at: {}", path.display());
    }

    let bin_dir = "bin/";
    // hard coded bin directory
    std::fs::create_dir_all(bin_dir)?;

    // at this point the /bin directory is hard coded again,
    // because the run-file and build-file command should work withoit the bloomery.toml.
    let status = match Command::new("javac")
        .arg("-d")
        .arg(bin_dir)
        .arg("-encoding")
        .arg("UTF-8")
        .arg(file)
        .status()
    {
        Ok(status) => status,
        Err(_) => error!("javac not found."),
    };

    if !status.success() {
        error!("Compilation failed");
    }

    success!("File build successfully");
    Ok(())
}
