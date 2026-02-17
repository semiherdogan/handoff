mod cli;
mod commands;
mod core;
mod templates;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use core::paths::AiPaths;

fn run() -> Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir()?;
    let paths = AiPaths::discover(&cwd);

    match cli.command {
        Command::Init { feature } => commands::init::run(&paths, feature.as_deref()),
        Command::Switch { feature } => commands::switch::run(&paths, &feature),
        Command::Continue { copy, raw } => commands::r#continue::run(&paths, copy, raw),
        Command::Prompt { copy, raw } => commands::r#continue::run(&paths, copy, raw),
        Command::List => commands::list::run(&paths),
        Command::Archive { feature } => commands::archive::run(&paths, &feature),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
