use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// structs for custom, installed templates, as toml
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredTemplate {
    pub name: String,
    pub dirs: Vec<String>,
    pub files: Vec<StoredFile>,
    pub main_class: String,
}

// get template directory
pub fn get_templates_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "bloomery")
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    let templates_dir = proj_dirs.config_dir().join("templates");
    fs::create_dir_all(&templates_dir)?;
    Ok(templates_dir)
}

// install a directory as a system-wide template
pub fn install_template(name: String, source_dir: &Path) -> Result<()> {
    if !source_dir.exists() {
        bail!("Source directory '{}' does not exist", source_dir.display());
    }

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    collect_template_assets(source_dir, source_dir, &mut dirs, &mut files)?;

    let template_data = StoredTemplate {
        name: name.clone(),
        dirs,
        files,
        main_class: "Main".to_string(),
    };

    let target_path = get_templates_dir()?.join(format!("{}.toml", name));
    let toml_string =
        toml::to_string_pretty(&template_data).context("Failed to serialize template data")?;

    fs::write(&target_path, toml_string)?;
    println!(
        "Template '{}' installed successfully at {}",
        name,
        target_path.display()
    );

    Ok(())
}

fn collect_template_assets(
    base: &Path,
    current: &Path,
    dirs: &mut Vec<String>,
    files: &mut Vec<StoredFile>,
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(base)?.to_string_lossy().to_string();

        if path.is_dir() {
            dirs.push(relative);
            collect_template_assets(base, &path, dirs, files)?;
        } else if path.is_file() {
            if relative.starts_with(".git")
                || relative.starts_with("target")
                || relative.starts_with("bin")
            {
                continue;
            }
            let content = fs::read_to_string(&path).unwrap_or_default();
            files.push(StoredFile {
                path: relative,
                content,
            });
        }
    }
    Ok(())
}

pub fn load_external_template(name: &str) -> Result<Option<StoredTemplate>> {
    let template_path = get_templates_dir()?.join(format!("{}.toml", name));
    if !template_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(template_path)?;
    let template: StoredTemplate = toml::from_str(&content)?;
    Ok(Some(template))
}
