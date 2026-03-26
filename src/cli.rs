use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "handoff",
    version = env!("HANDOFF_VERSION"),
    about = "Lightweight autonomous dev loop manager",
    after_help = concat!(
        "Repository: https://github.com/semiherdogan/handoff\n",
        "Version: ",
        env!("HANDOFF_VERSION")
    )
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create or select a feature workspace
    Init {
        /// Feature name to initialize
        feature: Option<String>,
        #[arg(long, help = "Replace existing feature workspace")]
        force: bool,
    },
    /// Switch to an existing feature workspace
    Switch {
        /// Feature name to switch to
        feature: String,
    },
    /// Generate a continuation prompt (requires valid execution plan)
    Continue {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Generate a planning-only prompt (updates SPEC/DESIGN/STATE)
    Generate {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Generate an execution-only prompt (requires execution plan)
    Start {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Generate prompt to turn FEATURE.md into SPEC.md
    Spec {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Generate prompt to create DESIGN.md from FEATURE.md + SPEC.md
    Design {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Generate prompt to create STATE.md execution plan
    Tasks {
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Raw prompt generator for a specific kind
    Prompt {
        /// Type of prompt to generate
        target: Option<PromptKind>,
        #[arg(long, help = "Copy prompt to clipboard")]
        copy: bool,
        #[arg(long, help = "Output raw prompt without formatting")]
        raw: bool,
    },
    /// Show active feature status and execution plan state
    Status {
        #[arg(long, help = "Continuously monitor status")]
        follow: bool,
    },
    /// Validate the current execution plan
    Validate,
    /// Print the CLI build version
    Version,
    /// List all feature workspaces
    List,
    /// Remove non-active feature directories
    Clean {
        #[arg(long, help = "Also remove active feature and clear current")]
        force: bool,
    },
    /// Mark a feature as archived
    Archive {
        /// Feature name to archive
        feature: String,
    },
    /// Print shell completion script
    Completion {
        /// Target shell for completion script
        shell: CompletionShell,
    },
    /// Fetch and install the latest release from GitHub
    Upgrade,
    /// Copy embedded templates to .handoff/templates/ for customization
    Export {
        #[arg(long, help = "Overwrite existing template files")]
        force: bool,
    },
    /// Toggle .handoff/ in .git/info/exclude
    Ignore,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PromptKind {
    /// Planning-only prompt (updates SPEC/DESIGN/STATE)
    Generate,
    /// Execution-only prompt (requires execution plan)
    Start,
    /// Generate prompt to turn FEATURE.md into SPEC.md
    Spec,
    /// Generate prompt to create DESIGN.md from FEATURE.md + SPEC.md
    Design,
    /// Generate prompt to create STATE.md execution plan
    Tasks,
    /// Continuation prompt (requires valid execution plan)
    Continue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum CompletionShell {
    /// Bourne Again Shell (bash)
    Bash,
    /// Z Shell (zsh)
    Zsh,
    /// Friendly Interactive Shell (fish)
    Fish,
    /// PowerShell
    PowerShell,
    /// Elvish shell
    Elvish,
}
