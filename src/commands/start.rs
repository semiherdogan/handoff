use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::workspace;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::{Context, Result};
use arboard::Clipboard;
use colored::Colorize;

pub fn run(paths: &AiPaths, copy: bool, raw: bool) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let template_manager = TemplateManager::new(paths);
    let prompt = prompts::start_prompt(&template_manager);

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
