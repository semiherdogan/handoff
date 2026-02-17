use crate::cli::PromptKind;
use crate::core::paths::AiPaths;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::{Context, Result};
use arboard::Clipboard;
use colored::Colorize;

pub fn run(paths: &AiPaths, kind: Option<PromptKind>, copy: bool, raw: bool) -> Result<()> {
    let template_manager = TemplateManager::new(paths);
    let selected = kind.unwrap_or(PromptKind::Continue);

    let prompt = match selected {
        PromptKind::Start => prompts::start_prompt(&template_manager),
        PromptKind::Continue => prompts::continuation_prompt(&template_manager),
    };

    if copy {
        let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
        clipboard
            .set_text(prompt.clone())
            .context("Failed to copy prompt to clipboard")?;
    }

    if !raw {
        println!("{}", "AI Prompt".bold().cyan());
        println!();
    }

    println!("{prompt}");

    if copy {
        println!();
        println!("{}", "Copied prompt to clipboard.".green());
    }

    Ok(())
}
