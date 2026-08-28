use anyhow::Result;
use clap::Parser;

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
        Commands::Run => {
            commands::build::build()?;
            commands::run::run()?;
        }
    }

    Ok(())
}
