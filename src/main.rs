use anyhow::Result;
use clap::Parser;

#[macro_use]
mod macros;

mod cli;
mod commands;
mod config;
mod templates;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => commands::init::init(name, &template)?,
        Commands::Install { name, path } => templates::external::install_template(name, &path)?,
        Commands::Build => commands::build::build()?,
        Commands::BuildFile { path } => commands::build_file::build_file(&path)?,
        Commands::Run => {
            commands::build::build()?;
            commands::run::run()?;
        }
        Commands::RunFile { path } => {
            commands::run_file::run_file(&path)?;
        }
        Commands::Clean => commands::clean::clean()?,
    }

    Ok(())
}
