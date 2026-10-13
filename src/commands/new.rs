use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::config::{Config, Dependencies, Paths, Project};
use crate::templates::{builtin::get_templates, external::load_external_template};

// write fresh bloomery.toml into 'root'
fn write_config(
    root: &Path,
    name: &str,
    version: &str,
    main_class: &str,
    class_dir: &str,
    clean_dirs: Vec<String>,
) -> Result<()> {
    let config = Config {
        project: Project {
            name: name.to_string(),
            version: version.to_string(),
        },
        paths: Paths {
            main_class: main_class.to_string(),
            class_dir: class_dir.to_string(),
            clean_dirs,
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

// create a new project in a new directory
pub fn new(name: Option<String>, template_name: &str) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "bloomery-project".to_string());
    let root = Path::new(&project_name);

    // Extract project name from the path so long paths are supported
    // means to use only the last path component as the project name in bloomery.toml
    let config_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&project_name);

    // checking if such directory exists
    if root.exists() {
        error!("'{}' already exists", project_name);
    }

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

        fs::create_dir_all(root)?;
        for dir in &ext_template.dirs {
            fs::create_dir_all(root.join(dir))?;
        }

        write_config(
            root,
            config_name,
            &ext_template.version,
            &ext_template.main_class,
            &ext_template.class_dir,
            ext_template.clean_dirs.clone(),
        )?;

        for file in &ext_template.files {
            if file.path == "bloomery.toml" {
                continue;
            }
            let path = crate::templates::external::template_relative_path(&file.path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(root.join(parent))?;
            }
            fs::write(root.join(path), &file.content)?;
        }

        info!(
            "Project created: {} (using installed template '{}')",
            project_name, ext_template.name
        );
        return Ok(());
    }

    // process built in templates
    let builtin_templates = get_templates();
    let template = builtin_templates.get(template_name).ok_or_else(|| {
        error!(
            "Unknown template: '{}'. Built-in options: {:?}",
            template_name,
            builtin_templates.keys().collect::<Vec<_>>()
        )
    })?;

    fs::create_dir_all(root)?;
    for dir in template.dirs {
        fs::create_dir_all(root.join(dir))?;
    }

    write_config(
        root,
        config_name,
        "0.1.0",
        template.main_class,
        template.class_dir,
        Vec::new(),
    )?;

    for file in template.files {
        fs::write(root.join(file.path), file.content)?;
    }

    info!(
        "Project created: {} with '{}' template",
        project_name, template.name
    );

    Ok(())
}
