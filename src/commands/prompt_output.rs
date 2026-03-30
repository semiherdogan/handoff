use anyhow::{Context, Result};
use arboard::Clipboard;
use colored::Colorize;

pub struct PromptSummary {
    pub title: String,
    pub what_happened: String,
    pub what_changed: String,
    pub next: String,
}

pub fn output_prompt_with_summary(
    prompt: &str,
    copy: bool,
    raw: bool,
    summary: Option<PromptSummary>,
) -> Result<()> {
    if copy {
        let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;
        clipboard
            .set_text(prompt.to_owned())
            .context("Failed to copy prompt to clipboard")?;
    }

    if !raw {
        let title = summary
            .as_ref()
            .map(|summary| summary.title.as_str())
            .unwrap_or("AI Prompt");
        println!("{}", title.bold().cyan());
        println!();

        if let Some(summary) = summary {
            println!("What happened");
            println!("- {}", summary.what_happened);
            println!("What changed");
            println!("- {}", summary.what_changed);
            println!("Next");
            println!("- {}", summary.next);
            println!();
        }
    }

    println!("{prompt}");

    if copy {
        println!();
        println!("{}", "Copied prompt to clipboard.".green());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::PromptSummary;

    fn render_summary(summary: &PromptSummary) -> String {
        format!(
            "What happened\n- {}\nWhat changed\n- {}\nNext\n- {}",
            summary.what_happened, summary.what_changed, summary.next
        )
    }

    #[test]
    fn prompt_summary_renders_three_required_sections() {
        let rendered = render_summary(&PromptSummary {
            title: "Planning Prompt".to_owned(),
            what_happened: "Prepared a planning prompt.".to_owned(),
            what_changed: "No files changed yet.".to_owned(),
            next: "Paste the prompt into your assistant.".to_owned(),
        });

        assert!(rendered.contains("What happened"));
        assert!(rendered.contains("What changed"));
        assert!(rendered.contains("Next"));
    }
}
