use anyhow::Result;
use clap::Parser;

#[macro_use]
mod macros;

mod cli;
mod commands;
mod config;
mod deps;
mod templates;

use cli::{Cli, Commands};

// main function, selection which function is called
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template } => {
            let template_name = match template {
                Some(t) => t,
                None => {
                    let global = config::load_global_config()?;
                    global.default_template
                }
            };
            commands::init::init(name, &template_name)?;
        }
        Commands::Install { name, path } => templates::external::install_template(name, &path)?,
        Commands::Build => commands::build::build()?,
        Commands::BuildFile { path, output } => commands::build_file::build_file(&path, &output)?,
        Commands::Run => {
            commands::build::build()?;
            commands::run::run()?;
        }
        Commands::RunFile { path } => {
            commands::run_file::run_file(&path)?;
        }
        Commands::Clean { clean_dirs } => {
            commands::clean::clean(&clean_dirs)?;
        }
        Commands::RunJar { manual_jar: path } => {
            commands::run_jar::run_jar(path)?;
        }
        Commands::BuildJar { output } => {
            commands::build_jar::package(&output)?;
        }
        Commands::Set { key, new_value } => {
            commands::set::set(&key, &new_value)?;
        }
        Commands::Deps { action } => match action {
            cli::DepsAction::Add { name, version } => commands::deps::add(&name, &version)?,
            cli::DepsAction::Remove { name } => commands::deps::remove(&name)?,
            cli::DepsAction::List => commands::deps::list()?,
            cli::DepsAction::AddLocal { path } => commands::deps::add_local(&path)?,
            cli::DepsAction::RemoveLocal { path } => commands::deps::remove_local(&path)?,
            cli::DepsAction::ListLocal => commands::deps::list_local()?,
        },
    }

    Ok(())
}
