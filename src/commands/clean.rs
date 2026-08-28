use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// delete 'target' directory
pub fn clean() -> Result<()> {
    let target = Path::new("target");

    if target.exists() {
        fs::remove_dir_all(target).context("Failed to remove target directory")?;
        println!("Cleaned target/");
    } else {
        println!("Nothing to clean (target/ does not exist)");
    }

    Ok(())
}
