use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bloomery")]
#[command(about = "build system for Java, easy and just works")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // create new Java project
    Init {
        // naming process is optional
        name: Option<String>,

        // Selected template (e.g., "default", "flat", or custom installed template)
        #[arg(short, long)]
        template: Option<String>,
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
    // clean, remove bin or target directory
    Clean,
    // compilation process
    Build,
    // build single file
    BuildFile {
        path: PathBuf,
    },
    // execution process
    Run,
    // run single file
    RunFile {
        path: PathBuf,
    },
}
