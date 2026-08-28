use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::Deserialize;
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => init(name)?,
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

/* At this point, the project structure is still hard-coded and there is only one template.
 * It should be reworked into selectable templates so that I can have a structure like:
 * .
 * |__ bin
 * |__ src
 * |  |__ Main.java
 * |__ .gitignore
 *
 * Or something like this.
 *
 * It should also be possible to install a template locally yourself and then install and use it.
 * I think this is supposed to be achieved somehow with toml parsing unctions.
 */
fn init(name: Option<String>) -> Result<()> {
    let project_name = name.unwrap_or_else(|| "bloomery-project".to_string());
    let root = Path::new(&project_name);

    if root.exists() {
        bail!("'{}' already exists", project_name);
    }

    fs::create_dir_all(root.join("src/main/java"))?;
    fs::create_dir_all(root.join("target/classes"))?;

    // hardcoded too; revise!!!
    let config = format!(
        r#"name = "{}"
version = "0.1.0"
main_class = "Main"
"#,
        project_name
    );
    fs::write(root.join("bloomery.toml"), config)?;

    // Java file with Hello World program.
    let main_java = r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
"#;
    fs::write(root.join("src/main/java/Main.java"), main_java)?;

    println!("project created: {}", project_name);

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

fn build() -> Result<()> {
    let config = load_config()?;
    println!("Building project '{}' v{}", config.name, config.version);

    let src_dir = Path::new("src/main/java");
    let java_files = find_java_files(src_dir)?;

    if java_files.is_empty() {
        bail!("No .java file found at src/main/java");
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
