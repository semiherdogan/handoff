use clap::{Parser, Subcommand};

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
    Prompt {
        #[arg(long)]
        copy: bool,
        #[arg(long)]
        raw: bool,
    },
    List,
    Archive {
        feature: String,
    },
}
