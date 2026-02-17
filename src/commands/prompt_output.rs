use anyhow::{Context, Result};
use arboard::Clipboard;
use colored::Colorize;

pub fn output_prompt(prompt: &str, copy: bool, raw: bool) -> Result<()> {
    if copy {
        let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
        clipboard
            .set_text(prompt.to_owned())
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
