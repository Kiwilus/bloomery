use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn run_file(path: &Path) -> Result<()> {
    info!("Starting {} ...", path.display());

    // You don't need to compile a single Java file first.
    // Java can run the .java file directly.
    // java <path_to_your_java_file>
    let status = match Command::new("java").arg(path).status() {
        Ok(status) => status,
        Err(_) => error!("Java could not be started"),
    };

    if !status.success() {
        error!("Execution failed");
    }

    Ok(())
}
