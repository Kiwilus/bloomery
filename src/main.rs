use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::Deserialize;
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

        // Selected template
        #[arg(short, long, default_value = "default")]
        template: String,
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

// struct to give a file a path and a content
struct FileTemplate {
    path: &'static str,
    content: &'static str,
}

// struct to describe the project template
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => init(name, &template)?,
        Commands::Build => build()?,
        Commands::Run => {
            // I don't like that the code compiles again for now,
            // because I might want to run an old version; I also want to revise that
            build()?;
            run()?;
        }
    }

    Ok(())
}

/*
 * It should be possible to install a template locally yourself and then install and use it.
 * I think this is supposed to be achieved somehow with toml parsing unctions.
 */
// init function that creates the project
fn init(name: Option<String>, template_name: &str) -> Result<()> {
    let templates = get_templates();

    let template = templates.get(template_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown template: '{}'. Available templates: {:?}",
            template_name,
            templates.keys().collect::<Vec<_>>()
        )
    })?;

    let project_name = name.unwrap_or_else(|| "bloomery-project".to_string());
    let root = Path::new(&project_name);

    if root.exists() {
        bail!("'{}' already exists", project_name);
    }

    // create directories defined by the template
    for dir in template.dirs {
        fs::create_dir_all(root.join(dir))?;
    }

    // config structure dynamic per template
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

    // Write files defined by the template
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
