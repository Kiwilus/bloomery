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
    New {
        name: Option<String>,
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Initialize new Java project
    Init {
        /// Selected template like "default", "advanced" or custom installed template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// install a java project directory as a system-wide template
    Install {
        /// Name of the template to install
        #[arg(short, long)]
        name: String,
        /// Path to the directory to use as template, default is your current
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
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
    /// clean class_dir + clean_dirs (+ optional extra dirs)
    Clean {
        #[arg(value_name = "DIR")]
        extra_dirs: Vec<PathBuf>,
    },

    /// add something to the project
    Add {
        #[command(subcommand)]
        what: AddWhat,
    },
    /// remove something from the project
    Remove {
        #[command(subcommand)]
        what: RemoveWhat,
    },
    /// list configured items
    List {
        #[command(subcommand)]
        what: ListWhat,
    },
}

#[derive(Subcommand)]
pub enum AddWhat {
    Deps {
        name: String,
        version: Option<String>,
    },
    Local {
        path: String,
    },
    #[command(name = "clean-dir")]
    CleanDir {
        dirs: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum RemoveWhat {
    Deps {
        name: String,
        #[arg(long, short = 'd')]
        delete_jar: bool,
    },
    Local {
        path: String,
    },
    #[command(name = "clean-dir")]
    CleanDir {
        dirs: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum ListWhat {
    Deps,
    Local,
    #[command(name = "clean-dirs")]
    CleanDirs,
}
