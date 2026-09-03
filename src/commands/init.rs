use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::templates::{builtin::get_templates, external::load_external_template};

// init function that creates the project
pub fn init(name: Option<String>, template_name: &str) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "bloomery-project".to_string());
    let root = Path::new(&project_name);

    if root.exists() {
        warn!("'{}' already exists", project_name);
    }

    // trying to load custom system-wide template first
    if let Some(ext_template) = load_external_template(template_name)? {
        for dir in &ext_template.dirs {
            fs::create_dir_all(root.join(dir))?;
        }

        let config = format!(
            r#"# name of your project
name = "{}"
# Version of your project
version = "0.1.0"
# class, which will run
main_class = "{}"
"#,
            project_name, ext_template.main_class
        );
        fs::write(root.join("bloomery.toml"), config)?;

        for file in &ext_template.files {
            if file.path == "bloomery.toml" {
                continue; // prevents overwriting generated config
            }
            if let Some(parent) = Path::new(&file.path).parent() {
                fs::create_dir_all(root.join(parent))?;
            }
            fs::write(root.join(&file.path), &file.content)?;
        }

        info!(
            "Project created: {} (using installed template '{}')",
            project_name, ext_template.name
        );
        return Ok(());
    }

    // fallback to built-in templates
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

    let config = format!(
        r#"# name of the project
name = "{}"
# Version of the project
version = "0.1.0"
# class to run
main_class = "{}"
"#,
        project_name, template.main_class
    );
    fs::write(root.join("bloomery.toml"), config)?;

    for file in template.files {
        fs::write(root.join(file.path), file.content)?;
    }

    info!(
        "Project created: {} with '{}' template",
        project_name, template.name
    );

    Ok(())
}
