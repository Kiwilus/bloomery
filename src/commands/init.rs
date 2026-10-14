use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::Build;
use crate::templates::{builtin::get_templates, external::load_external_template};

// Initialize a new project in an existing directory
pub fn init(template_name: &str) -> Result<()> {
    let root = Path::new(".");

    // do not re-initialize an already configured project.
    if root.join("bloomery.toml").exists() {
        error!("bloomery.toml already exists in the current directory");
    }

    // Use the current folder name as the project name in bloomery.toml
    let config_name = std::env::current_dir()?
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bloomery-project")
        .to_string();

    // process external templates
    if let Some(ext_template) = load_external_template(template_name)? {
        for dir in &ext_template.dirs {
            crate::templates::external::template_relative_path(dir)?;
        }
        for file in &ext_template.files {
            if file.path != "bloomery.toml" {
                crate::templates::external::template_relative_path(&file.path)?;
            }
        }

        for dir in &ext_template.dirs {
            fs::create_dir_all(root.join(dir))?;
        }

        crate::config::write_config(
            root,
            &config_name,
            &ext_template.version,
            &ext_template.main_class,
            &ext_template.class_dir,
            ext_template.clean_dirs.clone(),
            Build {
                release: ext_template
                    .release
                    .clone()
                    .or_else(|| Some("17".to_string())),
                jdk_home: ext_template
                    .jdk_home
                    .clone()
                    .or_else(|| std::env::var("JAVA_HOME").ok().filter(|s| !s.is_empty())),
            },
        )?;

        for file in &ext_template.files {
            if file.path == "bloomery.toml" {
                continue;
            }
            let path = crate::templates::external::template_relative_path(&file.path)?;
            let target = root.join(path);
            if target.exists() {
                continue;
            }
            if let Some(parent) = path.parent() {
                fs::create_dir_all(root.join(parent))?;
            }
            fs::write(target, &file.content)?;
        }

        info!(
            "Initialized bloomery in current directory (template '{}')",
            ext_template.name
        );
        return Ok(());
    }

    // process builtin temlates
    let builtin_templates = get_templates();
    let template = builtin_templates.get(template_name).ok_or_else(|| {
        error!(
            "Unknown template: '{}'. Built-in options: {:?}",
            template_name,
            builtin_templates.keys().collect::<Vec<_>>()
        )
    })?;

    for dir in template.dirs {
        fs::create_dir_all(root.join(dir))?;
    }

    crate::config::write_config(
        root,
        &config_name,
        "0.1.0",
        template.main_class,
        template.class_dir,
        Vec::new(),
        Build::default(),
    )?;

    for file in template.files {
        let target = root.join(file.path);
        if target.exists() {
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, file.content)?;
    }

    info!(
        "Initialized bloomery in current directory (template '{}')",
        template.name
    );
    Ok(())
}
