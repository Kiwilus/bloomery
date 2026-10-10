use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bloomery")]
#[command(about = "bloomery is a build system for Java, easy and just works")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// create new Java project
    Init {
        // naming process is optional
        name: Option<String>,

        // Selected template (e.g., "default", "flat", or custom installed template)
        #[arg(short, long)]
        template: Option<String>,
    },
    /// install a java project directory as a system-wide template
    Install {
        // Name of the template to install
        #[arg(short, long)]
        name: String,
        // Path to the directory to use as template, default is your current
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// clean, remove bin or target directory
    Clean {
        /// Additional directories to remove
        #[arg(value_name = "DIR")]
        clean_dirs: Vec<PathBuf>,
    },
    /// compilation process
    Build,
    /// build single file
    BuildFile {
        path: PathBuf,

        /// Dynamic directory where compiled files are stored
        /// when no output directory is given, the file will be compiled in your current directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// execution process
    Run,
    /// run single file
    RunFile { path: PathBuf },
    /// run .jar file
    RunJar { manual_jar: Option<String> },
    /// compile project into .jar file
    BuildJar {
        /// Dynamic directory where packaged jar is stored
        /// As in the build-file function, when no output directory is given, the file will be compiled in your current directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// set bloomery.toml configuration via CLI
    Set {
        /// key to set e.g. version, main_class
        key: String,
        /// this is your new value
        new_value: String,
    },
    /// manage dependencies via CLI
    Deps {
        #[command(subcommand)]
        action: DepsAction,
    },
}

#[derive(Subcommand)]
pub enum DepsAction {
    /// add a managed dependency
    Add {
        /// artifactID
        name: String,
        /// version
        version: Option<String>,
    },
    /// remove a managed dependency
    Remove { name: String },
    /// list managed dependencies
    List,
    /// add local JAR path
    AddLocal {
        /// path relative to project root
        path: String,
    },
    /// remove a local JAR path
    RemoveLocal { path: String },
    /// list local JAR paths
    ListLocal,
}
