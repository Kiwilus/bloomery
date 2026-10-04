use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::config::Config;

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
    #[serde(default = "default_version")]
    pub version: String,
    pub main_class: String,
    #[serde(default = "default_class_dir")]
    pub class_dir: String,
}

fn default_class_dir() -> String {
    "target/classes".to_string()
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn validate_template_name(name: &str) -> Result<()> {
    let path = Path::new(name);
    if name.is_empty()
        || path.components().count() != 1
        || !matches!(path.components().next(), Some(Component::Normal(_)))
    {
        error!("Template name must be a single file name");
    }
    Ok(())
}

pub fn template_relative_path(path: &str) -> Result<&Path> {
    let path = Path::new(path);
    if path.as_os_str().is_empty()
        || !path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        error!("Template paths must be relative and must not contain '.' or '..'");
    }
    Ok(path)
}

fn load_template_config(source_dir: &Path) -> Result<(String, String, String)> {
    let path = source_dir.join("bloomery.toml");
    if !path.exists() {
        return Ok((default_version(), "Main".to_string(), "bin".to_string()));
    }

    let content =
        fs::read_to_string(&path).with_context(|| format!("Could not read {}", path.display()))?;
    let config: Config =
        toml::from_str(&content).with_context(|| format!("Could not parse {}", path.display()))?;
    Ok((
        config.project.version,
        config.paths.main_class,
        config.paths.class_dir,
    ))
}

// get template directory
pub fn get_templates_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "bloomery")
        .ok_or_else(|| error!("Could not determine config directory"))?;
    let templates_dir = proj_dirs.config_dir().join("templates");
    fs::create_dir_all(&templates_dir)?;
    Ok(templates_dir)
}

// install a directory as a system-wide template
pub fn install_template(name: String, source_dir: &Path) -> Result<()> {
    validate_template_name(&name)?;
    if !source_dir.exists() {
        error!("Source directory '{}' does not exist", source_dir.display());
    }
    if !source_dir.is_dir() {
        error!("Source path '{}' is not a directory", source_dir.display());
    }

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    collect_template_assets(source_dir, source_dir, &mut dirs, &mut files)?;
    let (version, main_class, class_dir) = load_template_config(source_dir)?;

    let template_data = StoredTemplate {
        name: name.clone(),
        dirs,
        files,
        version,
        main_class,
        class_dir,
    };

    let target_path = get_templates_dir()?.join(format!("{}.toml", name));
    let toml_string = match toml::to_string_pretty(&template_data) {
        Ok(data) => data,
        Err(_) => {
            error!("Failed to serialize template data");
        }
    };

    fs::write(&target_path, toml_string)?;
    success!(
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
        let file_type = entry.file_type()?;
        let relative = path.strip_prefix(base)?.to_string_lossy().to_string();

        if file_type.is_symlink() {
            error!(
                "Template contains unsupported symbolic link: {}",
                path.display()
            );
        }

        if file_type.is_dir() {
            if current == base
                && matches!(entry.file_name().to_str(), Some(".git" | "target" | "bin"))
            {
                continue;
            }
            dirs.push(relative);
            collect_template_assets(base, &path, dirs, files)?;
        } else if file_type.is_file() {
            let content = fs::read_to_string(&path)
                .with_context(|| error!("Template file is not valid UTF-8: {}", path.display()))?;
            files.push(StoredFile {
                path: relative,
                content,
            });
        }
    }
    Ok(())
}

pub fn load_external_template(name: &str) -> Result<Option<StoredTemplate>> {
    validate_template_name(name)?;
    let template_path = get_templates_dir()?.join(format!("{}.toml", name));
    if !template_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(template_path)?;
    let template: StoredTemplate = toml::from_str(&content)?;
    Ok(Some(template))
}
