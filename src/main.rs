use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "bloomery")]
#[command(about = "build system for Java, easy and just works")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // create new Java project
    Init {
        // naming process is optional
        name: Option<String>,

        /// Selected template (e.g., "default", "flat", or custom installed template)
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    // install a directory as a system-wide template
    Install {
        // Name of the template to install
        #[arg(short, long)]
        name: String,
        // Path to the directory to use as template, default is '.'
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    // compilation process
    Build,
    // execution process
    Run,
}

// struct for the bloomery.toml file
#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: String,
    main_class: String,
}

// structs for custom ,installed templates, as toml
#[derive(Debug, Serialize, Deserialize)]
struct StoredFile {
    path: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredTemplate {
    name: String,
    dirs: Vec<String>,
    files: Vec<StoredFile>,
    main_class: String,
}

// struct to describe the project template
struct FileTemplate {
    path: &'static str,
    content: &'static str,
}

struct ProjectTemplate {
    name: &'static str,
    dirs: &'static [&'static str],
    files: &'static [FileTemplate],
    main_class: &'static str,
}

// Two templates; an advanced and a default one
fn get_templates() -> HashMap<&'static str, ProjectTemplate> {
    let mut map = HashMap::new();

    /*
     * This is a minimal template
     */
    map.insert(
        "default",
        ProjectTemplate {
            name: "default",
            dirs: &["src", "bin"],
            main_class: "Main",
            files: &[
                FileTemplate {
                    path: "src/Main.java",
                    content: "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n",
                },
                FileTemplate {
                    path: ".gitignore",
                    content: "/bin/\n*.class\n",
                },
            ],
        },
    );

    /*
     * This is a maven like template
     */
    map.insert(
        "advanced",
        ProjectTemplate {
            name: "advanced",
            dirs: &["src/main/java", "target/classes"],
            main_class: "Main",
            files: &[
                FileTemplate {
                    path: "src/main/java/Main.java",
                    content: "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n",
                },
                FileTemplate {
                    path: ".gitignore",
                    content: "/target/\n*.class\n",
                },
            ],
        },
    );

    map
}

// get template edirectory
fn get_templates_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "", "bloomery")
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    let templates_dir = proj_dirs.config_dir().join("templates");
    fs::create_dir_all(&templates_dir)?;
    Ok(templates_dir)
}

// main function
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => init(name, &template)?,
        Commands::Install { name, path } => install_template(name, &path)?,
        Commands::Build => build()?,
        Commands::Run => {
            build()?;
            run()?;
        }
    }

    Ok(())
}

// install custom the templates on the system
fn install_template(name: String, source_dir: &Path) -> Result<()> {
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

fn load_external_template(name: &str) -> Result<Option<StoredTemplate>> {
    let template_path = get_templates_dir()?.join(format!("{}.toml", name));
    if !template_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(template_path)?;
    let template: StoredTemplate = toml::from_str(&content)?;
    Ok(Some(template))
}

/*
 * It should be possible to install a template locally yourself and then install and use it.
 * I think this is supposed to be achieved somehow with toml parsing unctions.
 */
// init function that creates the project
fn init(name: Option<String>, template_name: &str) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "bloomery-project".to_string());
    let root = Path::new(&project_name);

    if root.exists() {
        bail!("'{}' already exists", project_name);
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

        println!(
            "Project created: {} (using installed template '{}')",
            project_name, ext_template.name
        );
        return Ok(());
    }

    // fallback to built-in templates
    let builtin_templates = get_templates();
    let template = builtin_templates.get(template_name).ok_or_else(|| {
        anyhow::anyhow!(
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

    println!(
        "Project created: {} with '{}' template",
        project_name, template.name
    );

    Ok(())
}

fn load_config() -> Result<Config> {
    let content = fs::read_to_string("bloomery.toml")
        .context("bloomery.toml not found. Are you in the project directory?")?;
    let config: Config = toml::from_str(&content).context("bloomery.toml could not be read")?;
    Ok(config)
}

fn find_java_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_java_files(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "java") {
            files.push(path);
        }
    }
    Ok(files)
}

// build function to compile java code
fn build() -> Result<()> {
    let config = load_config()?;
    println!("Building project '{}' v{}", config.name, config.version);

    let src_dir = Path::new("src");
    let java_files = find_java_files(src_dir)?;

    if java_files.is_empty() {
        bail!("No .java file found at src/");
    }

    fs::create_dir_all("target/classes")?;

    let status = Command::new("javac")
        .arg("-d")
        .arg("target/classes")
        .arg("-encoding")
        .arg("UTF-8")
        .args(&java_files)
        .status()
        .context("javac not found.")?;

    if !status.success() {
        bail!("Compilation failed");
    }

    println!("Build successfully ({} files)", java_files.len());
    Ok(())
}

// compiles java code and runs it directly
fn run() -> Result<()> {
    let config = load_config()?;
    println!("Starting {} ...", config.main_class);

    let status = Command::new("java")
        .arg("-cp")
        .arg("target/classes")
        .arg(&config.main_class)
        .status()
        .context("java could not be started")?;

    if !status.success() {
        bail!("Execution failed");
    }

    Ok(())
}
