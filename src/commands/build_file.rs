use anyhow::Result;
use std::path::Path;
use std::process::Command;

// When you use build_file, a 'bin' directory will be created where the compiled file is stored.
// If you don't want to create a 'bin' directory, you can use the run-file command instead.
// This will run the Java file directly without creating a 'bin' directory.
pub fn build_file(path: &Path) -> Result<()> {
    let file = Path::new(path);

    if !file.exists() {
        error!("File not found at: {}", path.display());
    }

    let bin_dir = "bin/";
    // hard coded bin directory
    std::fs::create_dir_all(bin_dir)?;

    // at this point the /bin directory is hard coded again,
    // because the build-file command should work withoit the bloomery.toml.
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
