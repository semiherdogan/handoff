use crate::cli::{Cli, CompletionShell};
use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;
use std::io;

pub fn run(shell: CompletionShell) -> Result<()> {
    let mut command = Cli::command();
    let shell = match shell {
        CompletionShell::Bash => clap_complete::Shell::Bash,
        CompletionShell::Zsh => clap_complete::Shell::Zsh,
        CompletionShell::Fish => clap_complete::Shell::Fish,
        CompletionShell::PowerShell => clap_complete::Shell::PowerShell,
        CompletionShell::Elvish => clap_complete::Shell::Elvish,
    };

    generate(shell, &mut command, "handoff", &mut io::stdout());
    Ok(())
}
