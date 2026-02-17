use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct StateSummary {
    pub current_step: String,
    pub completed_steps: usize,
    pub remaining_steps: usize,
    pub known_risks: Vec<String>,
    pub execution_plan_initialized: bool,
}

pub fn parse_state(content: &str) -> StateSummary {
    let current_step_section = section_content(content, "Current Step").unwrap_or_default();
    let execution_plan_section = section_content(content, "Execution Plan").unwrap_or_default();
    let risks_section = section_content(content, "Risks").unwrap_or_default();

    let current_step = current_step_section
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("Unknown")
        .to_owned();

    let mut completed_steps = 0usize;
    let mut pending_steps = 0usize;
    let mut current_steps = 0usize;

    for line in execution_plan_section.lines() {
        if let Some(marker) = step_marker(line) {
            match marker {
                StepMarker::Completed => completed_steps += 1,
                StepMarker::Pending => pending_steps += 1,
                StepMarker::Current => current_steps += 1,
            }
        }
    }

    let known_risks = risks_section
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.eq_ignore_ascii_case("none"))
        .map(|line| line.trim_start_matches("- ").to_owned())
        .collect::<Vec<_>>();

    let not_started = current_step.eq_ignore_ascii_case("Not started");
    let not_generated = execution_plan_section
        .lines()
        .map(str::trim)
        .any(|line| line.eq_ignore_ascii_case("Not yet generated") || line.eq_ignore_ascii_case("Not yet generated."));
    let has_any_step = completed_steps + pending_steps + current_steps > 0;

    StateSummary {
        current_step,
        completed_steps,
        remaining_steps: pending_steps + current_steps,
        known_risks,
        execution_plan_initialized: !not_started && !not_generated && has_any_step,
    }
}

pub fn ensure_execution_plan_initialized(content: &str) -> Result<()> {
    let summary = parse_state(content);
    if summary.execution_plan_initialized {
        return Ok(());
    }

    Err(anyhow!(
        "Execution plan not initialized. Run `ai start` first."
    ))
}

fn section_content(content: &str, title: &str) -> Option<String> {
    let header = format!("# {title}");
    let mut in_section = false;
    let mut lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == header {
            in_section = true;
            continue;
        }

        if in_section && trimmed.starts_with("# ") {
            break;
        }

        if in_section {
            lines.push(line);
        }
    }

    if in_section {
        Some(lines.join("\n").trim().to_owned())
    } else {
        None
    }
}

enum StepMarker {
    Pending,
    Current,
    Completed,
}

fn step_marker(line: &str) -> Option<StepMarker> {
    let normalized = line.trim_start().trim_start_matches(['-', '*']).trim_start();

    if normalized.starts_with("[ ]") {
        Some(StepMarker::Pending)
    } else if normalized.starts_with("[>]") {
        Some(StepMarker::Current)
    } else if normalized.starts_with("[x]") || normalized.starts_with("[X]") {
        Some(StepMarker::Completed)
    } else {
        None
    }
}
