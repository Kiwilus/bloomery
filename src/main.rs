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
        Commands::Init { template } => {
            let template_name = match template {
                Some(t) => t,
                None => config::load_global_config()?.default_template,
            };
            commands::init::init(&template_name)?;
        }
        Commands::New { name, template } => {
            let template_name = match template {
                Some(t) => t,
                None => config::load_global_config()?.default_template,
            };
            commands::new::new(name, &template_name)?;
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
        Commands::Clean { extra_dirs } => {
            commands::clean::clean(&extra_dirs)?;
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
        Commands::Add { what } => match what {
            cli::AddWhat::Deps { name, version } => commands::deps::add(&name, version.as_deref())?,
            cli::AddWhat::Local { path } => commands::deps::add_local(&path)?,
            cli::AddWhat::CleanDir { dirs } => commands::deps::add_clean_dir(&dirs)?,
        },
        Commands::Remove { what } => match what {
            cli::RemoveWhat::Deps { name, delete_jar } => {
                commands::deps::remove(&name, delete_jar)?
            }
            cli::RemoveWhat::Local { path } => commands::deps::remove_local(&path)?,
            cli::RemoveWhat::CleanDir { dirs } => commands::deps::remove_clean_dir(&dirs)?,
        },
        Commands::List { what } => match what {
            cli::ListWhat::Deps => commands::deps::list()?,
            cli::ListWhat::Local => commands::deps::list_local()?,
            cli::ListWhat::CleanDirs => commands::deps::list_clean_dirs()?,
        },
    }

    Ok(())
}
