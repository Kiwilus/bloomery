use crate::config::load_config;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/*
 * if you have a bloomery.toml in your directory, use its class_dir + clean_dirs.
 * when you don't have a bloomery.toml in your directory, the 'bin' folder will be deleted.
 *
 * in both use cases you can remove multiple directorys e.g. you have a bloomery.toml file where the class_dir is 'target'
 * and you want to remove the target directory and a other directory you have.
 * you can do it with blm clean <your_directory>. bloomery will clean the directory from the bloomery.toml and additionally the directory from your CLI input.
 */
pub fn clean(cli_dirs: &[PathBuf]) -> Result<()> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    match load_config() {
        Ok(config) => {
            // always clean class_dir
            dirs.push(PathBuf::from(&config.paths.class_dir));

            // clean_dirs
            for dir in &config.paths.clean_dirs {
                dirs.push(PathBuf::from(dir));
            }
        }
        Err(_) => {
            // fallback if no bloomery.toml
            dirs.push(PathBuf::from("bin"));
        }
    }

    // add CLI arguments
    dirs.extend_from_slice(cli_dirs);

    // checking
    dirs.sort();
    dirs.dedup();

    // removing
    for dir in dirs {
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
            info!("Cleaned {}", dir.display());
        } else {
            info!("Nothing to clean ({} does not exist)", dir.display());
        }
    }

    Ok(())
}
