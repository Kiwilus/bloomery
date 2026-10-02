use crate::config::load_config;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/*
 * if you have a bloomery.toml in your directory, use its class_dir.
 * when you don't have a bloomery.toml in your directory, the 'bin' folder will be deleted.
 *
 * in both use cases you can remove multiple directorys e.g. you have a bloomery.toml file where the class_dir is 'target'
 * and you want to remove the target directory and a other directory you have.
 * you can do it with blm clean <your_directory>. bloomery will clean the directory from the bloomery.toml and additionally the directory from your CLI input.
*/
pub fn clean(clean_dirs: &[PathBuf]) -> Result<()> {
    let default_dir = match load_config() {
        Ok(config) => PathBuf::from(config.class_dir),
        Err(_) => PathBuf::from("bin"),
    };

    // vector of directory or multiple directorys
    let mut dirs = vec![default_dir];
    dirs.extend_from_slice(clean_dirs);

    // remove every directory step by step
    for dir in dirs {
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
            info!("Cleaned {}/", dir.display());
        } else {
            info!("Nothing to clean ({} does not exist)", dir.display());
        }
    }

    Ok(())
}
