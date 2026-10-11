use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{Config, Dependencies, Paths, Project};
use crate::templates::{builtin::get_templates, external::load_external_template};

// write a fresh bloomery.toml to 'root'
fn write_config(
    root: &Path,
    name: &str,
    version: &str,
    main_class: &str,
    class_dir: &str,
) -> Result<()> {
    let config = Config {
        project: Project {
            name: name.to_string(),
            version: version.to_string(),
        },
        paths: Paths {
            main_class: main_class.to_string(),
            class_dir: class_dir.to_string(),
            clean_dirs: Vec::new(),
        },
        dependencies: Dependencies::default(),
    };

    let mut content = toml::to_string_pretty(&config)?;

    // Ensure [dependencies] exists before [dependencies.local]
    if let Some(pos) = content.find("[dependencies.local]") {
        content.insert_str(pos, "[dependencies]\n\n");
    } else {
        content.push_str("\n[dependencies]\n");
    }

    fs::write(root.join("bloomery.toml"), content)?;
    Ok(())
}

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

    // process extern templates
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

        write_config(
            root,
            &config_name,
            &ext_template.version,
            &ext_template.main_class,
            &ext_template.class_dir,
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

    write_config(
        root,
        &config_name,
        "0.1.0",
        template.main_class,
        template.class_dir,
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
