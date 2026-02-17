use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "ai", version, about = "Lightweight autonomous dev loop manager")]
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
    Status,
    List,
    Archive {
        feature: String,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PromptKind {
    Start,
    Continue,
}
