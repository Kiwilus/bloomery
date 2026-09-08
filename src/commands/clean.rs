use anyhow::Result;
use std::fs;
use std::path::Path;

/*
 * The clean command only deletes the target director,
 * but what if my .class files are stored in /bin?
 */

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
