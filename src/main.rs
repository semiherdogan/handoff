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
        Command::Init { feature, force } => commands::init::run(&paths, feature.as_deref(), force),
        Command::Switch { feature } => commands::switch::run(&paths, &feature),
        Command::Run { copy, raw } => commands::run::run(&paths, copy, raw),
        Command::Continue { copy, raw } => commands::r#continue::run(&paths, copy, raw),
        Command::Generate { copy, raw } => commands::generate::run(&paths, copy, raw),
        Command::Start { copy, raw } => commands::start::run(&paths, copy, raw),
        Command::Spec { copy, raw } => commands::spec::run(&paths, copy, raw),
        Command::Design { copy, raw } => commands::design::run(&paths, copy, raw),
        Command::Tasks { copy, raw } => commands::tasks::run(&paths, copy, raw),
        Command::Prompt { target, copy, raw } => commands::prompt::run(&paths, target, copy, raw),
        Command::Status { follow } => commands::status::run(&paths, follow),
        Command::Next => commands::next::run(&paths),
        Command::Validate => commands::validate::run(&paths),
        Command::Version => commands::version::run(),
        Command::List => commands::list::run(&paths),
        Command::Clean { force } => commands::clean::run(&paths, force),
        Command::Archive { feature } => commands::archive::run(&paths, &feature),
        Command::Completion { shell } => commands::completion::run(shell),
        Command::Upgrade => commands::upgrade::run(),
        Command::Export { force } => commands::export::run(&paths, force),
        Command::Ignore => commands::ignore::run(),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
