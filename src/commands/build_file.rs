use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn build_file(path: &Path) -> Result<()> {
    let file = Path::new(path);

    if !file.exists() {
        error!("File not found at: {}", path.display());
    }

    /*
     * This creates a hard coded bin directory
     * the clean command will delete it automaticly, if no bloomery.toml is found in your root folder.
     */
    std::fs::create_dir_all("bin")?;

    // build file command: javac -d bin -encoding UTF-8 <file_stored_in_variable>
    let status = match Command::new("javac")
        .arg("-d")
        .arg("bin")
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
