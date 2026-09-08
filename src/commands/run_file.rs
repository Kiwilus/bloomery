use anyhow::Result;
use std::path::Path;
use std::process::Command;

use super::build_file;

pub fn run_file(path: &Path) -> Result<()> {
    /*
     * At this point the /bin directory is hard coded.
     */

    // first building file
    build_file::build_file(path)?;

    // execute the build
    let file_name = match path.file_stem().and_then(|name| name.to_str()) {
        Some(name) => name,
        None => error!("Invalid Java file name"),
    };

    info!("Starting {} ...", file_name);

    // java command to run .class: java -cp bin <file_name_stored_in_variable>
    let status = match Command::new("java")
        .arg("-cp")
        .arg("bin")
        .arg(file_name)
        .status()
    {
        Ok(status) => status,
        Err(_) => error!("Java could not be started"),
    };

    if !status.success() {
        error!("Execution failed");
    }

    Ok(())
}
