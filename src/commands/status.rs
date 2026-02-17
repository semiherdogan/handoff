use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::state::StateSummary;
use crate::core::workspace;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn run(paths: &AiPaths, follow: bool) -> Result<()> {
    if follow {
        return run_follow(paths);
    }

    let (feature_name, summary, _) = load_status(paths)?;
    print_standard_status(&feature_name, &summary);
    Ok(())
}

fn run_follow(paths: &AiPaths) -> Result<()> {
    let poll_interval = Duration::from_secs(2);
    let mut spinner_idx = 0usize;
    let terminal_columns = terminal_columns();

    loop {
        let (feature_name, summary, current_plan_step) = load_status(paths)?;

        if summary.remaining_steps == 0 {
            print!("\r\x1b[2K");
            io::stdout().flush().with_context(|| "Failed to flush stdout")?;
            print_standard_status(&feature_name, &summary);
            return Ok(());
        }

        let step = current_plan_step.unwrap_or_else(|| "No active [>] step".to_string());
        let frame = spinner_frame(spinner_idx);
        let follow_line = format_follow_line(frame, &step, terminal_columns);

        print!("\r\x1b[2K{follow_line}");
        io::stdout().flush().with_context(|| "Failed to flush stdout")?;

        spinner_idx = (spinner_idx + 1) % 4;
        thread::sleep(poll_interval);
    }
}

fn terminal_columns() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|columns| *columns > 0)
        .unwrap_or(80)
}

fn format_follow_line(frame: &str, step: &str, columns: usize) -> String {
    // Root cause: long step text wrapped to a second terminal row, so clearing only the current
    // row left repeated output artifacts instead of an in-place spinner update.
    let reserved = frame.len() + 1;
    let available = columns.saturating_sub(reserved);
    let display_step = truncate_for_width(step, available);
    format!("{frame} {display_step}")
}

fn truncate_for_width(text: &str, max_chars: usize) -> String {
    let text_len = text.chars().count();
    if text_len <= max_chars {
        return text.to_owned();
    }

    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }

    let mut truncated = String::with_capacity(max_chars);
    for c in text.chars().take(max_chars - 3) {
        truncated.push(c);
    }
    truncated.push_str("...");
    truncated
}

fn load_status(paths: &AiPaths) -> Result<(String, StateSummary, Option<String>)> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let feature_name = workspace::resolve_current_feature_name(paths)?;
    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let summary = state::parse_state(&state_content);
    let current_plan_step = state::current_execution_plan_step(&state_content);

    Ok((feature_name, summary, current_plan_step))
}

fn print_standard_status(feature_name: &str, summary: &StateSummary) {
    println!("Active feature: {feature_name}");
    println!("Current Step: {}", summary.current_step);
    println!("Remaining steps: {}", summary.remaining_steps);
    println!("Completed steps: {}", summary.completed_steps);

    if summary.known_risks.is_empty() {
        println!("Known risks: None");
    } else {
        println!("Known risks:");
        for risk in &summary.known_risks {
            println!("- {risk}");
        }
    }
}

fn spinner_frame(index: usize) -> &'static str {
    // Root cause: the previous pseudo-progress frames looked like partial completion,
    // which made follow mode feel noisy instead of indicating simple liveness.
    match index % 4 {
        0 => "[|]",
        1 => "[/]",
        2 => "[-]",
        _ => "[\\]",
    }
}

#[cfg(test)]
mod tests {
    use super::{format_follow_line, spinner_frame};

    #[test]
    fn spinner_frame_uses_clean_ascii_spinner_cycle() {
        assert_eq!(spinner_frame(0), "[|]");
        assert_eq!(spinner_frame(1), "[/]");
        assert_eq!(spinner_frame(2), "[-]");
        assert_eq!(spinner_frame(3), "[\\]");
    }

    #[test]
    fn spinner_frame_wraps_every_four_ticks() {
        assert_eq!(spinner_frame(4), "[|]");
        assert_eq!(spinner_frame(5), "[/]");
    }

    #[test]
    fn format_follow_line_truncates_step_to_terminal_width() {
        let line = format_follow_line("[|]", "abcdefghijklmnopqrstuvwxyz", 12);
        assert_eq!(line, "[|] abcde...");
    }

    #[test]
    fn format_follow_line_keeps_short_steps_unchanged() {
        let line = format_follow_line("[-]", "short step", 80);
        assert_eq!(line, "[-] short step");
    }
}
