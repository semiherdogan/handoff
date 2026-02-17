use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "handoff",
    version,
    about = "Lightweight autonomous dev loop manager",
    after_help = concat!(
        "Repository: https://github.com/semiherdogan/handoff\n",
        "Version: ",
        env!("CARGO_PKG_VERSION")
    )
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init {
        feature: Option<String>,
        #[arg(long)]
        force: bool,
    },
    Switch {
        feature: String,
    },
    Continue {
        #[arg(long)]
        copy: bool,
        #[arg(long)]
        raw: bool,
    },
    Start {
        #[arg(long)]
        copy: bool,
        #[arg(long)]
        raw: bool,
    },
    Prompt {
        target: Option<PromptKind>,
        #[arg(long)]
        copy: bool,
        #[arg(long)]
        raw: bool,
    },
    Status {
        #[arg(long)]
        follow: bool,
    },
    Version,
    List,
    Clean,
    Archive {
        feature: String,
    },
    Completion {
        shell: CompletionShell,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PromptKind {
    Start,
    Continue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
