use anyhow::Result;
use std::fs;
use std::path::Path;

// delete 'target' directory
pub fn clean() -> Result<()> {
    let target = Path::new("target");

    if target.exists() {
        if fs::remove_dir_all(target).is_err() {
            error!("Failed to remove target directory");
        }
        info!("Cleaned target/");
    } else {
        info!("Nothing to clean (target/ does not exist)");
    }

    Ok(())
}
