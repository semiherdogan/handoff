use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct StateSummary {
    pub current_step: String,
    pub completed_steps: usize,
    pub current_steps: usize,
    pub remaining_steps: usize,
    pub known_risks: Vec<String>,
    pub execution_plan_initialized: bool,
}

/// Validation outcome shared by guards and user-facing status reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPlanValidation {
    Ready,
    NotInitialized,
    MultipleCurrentSteps,
    NoRemainingSteps,
}

impl ExecutionPlanValidation {
    pub fn status_label(&self) -> &'static str {
        match self {
            Self::Ready => "valid",
            Self::NotInitialized => "not initialized",
            Self::MultipleCurrentSteps => "invalid",
            Self::NoRemainingSteps => "complete",
        }
    }

    pub fn summary_message(&self) -> &'static str {
        match self {
            Self::Ready => "Execution plan is valid.",
            Self::NotInitialized => "Execution plan has not been initialized yet.",
            Self::MultipleCurrentSteps => {
                "Execution plan is invalid because multiple current steps ([>]) were found."
            }
            Self::NoRemainingSteps => "Execution plan is complete. No remaining steps.",
        }
    }

    pub fn guard_message(&self) -> &'static str {
        match self {
            Self::Ready => "Execution plan is valid.",
            Self::NotInitialized => "Execution plan not initialized. Run `handoff start` first.",
            Self::MultipleCurrentSteps => {
                "Invalid execution plan: multiple current steps ([>]) found."
            }
            Self::NoRemainingSteps => "No remaining steps to continue.",
        }
    }
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
    let not_generated = execution_plan_section.lines().map(str::trim).any(|line| {
        line.eq_ignore_ascii_case("Not yet generated")
            || line.eq_ignore_ascii_case("Not yet generated.")
    });
    let has_any_step = completed_steps + pending_steps + current_steps > 0;

    StateSummary {
        current_step,
        completed_steps,
        current_steps,
        remaining_steps: pending_steps + current_steps,
        known_risks,
        execution_plan_initialized: !not_started && !not_generated && has_any_step,
    }
}

pub fn validate_execution_plan(content: &str) -> ExecutionPlanValidation {
    let summary = parse_state(content);

    if summary.current_steps > 1 {
        return ExecutionPlanValidation::MultipleCurrentSteps;
    }

    if summary.completed_steps > 0 && summary.remaining_steps == 0 {
        return ExecutionPlanValidation::NoRemainingSteps;
    }

    if summary.execution_plan_initialized {
        return ExecutionPlanValidation::Ready;
    }

    ExecutionPlanValidation::NotInitialized
}

pub fn ensure_execution_plan_initialized(content: &str) -> Result<()> {
    match validate_execution_plan(content) {
        ExecutionPlanValidation::Ready => Ok(()),
        validation => Err(anyhow!(validation.guard_message())),
    }
}

pub fn current_execution_plan_step(content: &str) -> Option<String> {
    let execution_plan_section = section_content(content, "Execution Plan")?;

    for line in execution_plan_section.lines() {
        if let Some(normalized) = normalize_step_prefix(line)
            && let Some(rest) = normalized.strip_prefix("[>]")
        {
            let step = rest.trim_start().to_owned();
            if !step.is_empty() {
                return Some(step);
            }
        }
    }

    None
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
    let normalized = normalize_step_prefix(line)?;

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

fn normalize_step_prefix(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();

    if let Some(rest) = trimmed.strip_prefix("- ") {
        return Some(rest.trim_start());
    }

    if let Some(rest) = trimmed.strip_prefix("* ") {
        return Some(rest.trim_start());
    }

    let mut split_idx = 0usize;
    for ch in trimmed.chars() {
        if ch.is_ascii_digit() {
            split_idx += ch.len_utf8();
        } else {
            break;
        }
    }

    if split_idx > 0 {
        let rest = &trimmed[split_idx..];
        if let Some(rest) = rest.strip_prefix(". ") {
            return Some(rest.trim_start());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionPlanValidation, current_execution_plan_step, ensure_execution_plan_initialized,
        parse_state, validate_execution_plan,
    };

    fn state_doc(current_step: &str, execution_plan: &str, risks: &str) -> String {
        format!(
            "# State\n\n# Current Step\n{current_step}\n\n# Execution Plan\n{execution_plan}\n\n# Risks\n{risks}\n"
        )
    }

    #[test]
    fn valid_plan_has_expected_counts_and_guard_passes() {
        let content = state_doc(
            "Implement parser",
            "- [x] bootstrap\n- [>] implement parser\n- [ ] add tests\n- [ ] validate",
            "- parser drift",
        );

        let summary = parse_state(&content);
        assert_eq!(summary.completed_steps, 1);
        assert_eq!(summary.current_steps, 1);
        assert_eq!(summary.remaining_steps, 3);
        assert_eq!(summary.known_risks, vec!["parser drift".to_string()]);
        assert!(ensure_execution_plan_initialized(&content).is_ok());
        assert_eq!(
            validate_execution_plan(&content),
            ExecutionPlanValidation::Ready
        );
    }

    #[test]
    fn done_only_plan_fails_with_deterministic_error() {
        let content = state_doc("Done", "- [x] one\n- [x] two", "None");

        let error = ensure_execution_plan_initialized(&content).unwrap_err();
        assert_eq!(error.to_string(), "No remaining steps to continue.");
        assert_eq!(
            validate_execution_plan(&content),
            ExecutionPlanValidation::NoRemainingSteps
        );
    }

    #[test]
    fn no_current_plan_with_pending_steps_passes_guard() {
        let content = state_doc("Pick next", "- [ ] one\n- [ ] two", "None");

        let summary = parse_state(&content);
        assert_eq!(summary.current_steps, 0);
        assert_eq!(summary.remaining_steps, 2);
        assert!(ensure_execution_plan_initialized(&content).is_ok());
    }

    #[test]
    fn multiple_current_steps_fail_with_deterministic_error() {
        let content = state_doc("Implement", "- [>] one\n- [>] two\n- [ ] three", "None");

        let error = ensure_execution_plan_initialized(&content).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Invalid execution plan: multiple current steps ([>]) found."
        );
        assert_eq!(
            validate_execution_plan(&content),
            ExecutionPlanValidation::MultipleCurrentSteps
        );
    }

    #[test]
    fn empty_execution_plan_fails_with_start_first_error() {
        let content = state_doc("Not started", "", "None");

        let error = ensure_execution_plan_initialized(&content).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Execution plan not initialized. Run `handoff start` first."
        );
        assert_eq!(
            validate_execution_plan(&content),
            ExecutionPlanValidation::NotInitialized
        );
    }

    #[test]
    fn mixed_formatting_is_parsed_for_list_and_numbered_markers_only() {
        let content = state_doc(
            "Formatting",
            "\t- [>] current\t\n  * [ ] pending   \n1. [x] completed\n[ ] raw should not count\nparagraph [x] inline should not count",
            "None",
        );

        let summary = parse_state(&content);
        assert_eq!(summary.current_steps, 1);
        assert_eq!(summary.completed_steps, 1);
        assert_eq!(summary.remaining_steps, 2);
        assert!(ensure_execution_plan_initialized(&content).is_ok());
    }

    #[test]
    fn current_execution_plan_step_returns_current_marker_text() {
        let content = state_doc(
            "Current",
            "- [x] one\n- [>] implement api\n- [ ] test",
            "None",
        );

        let current = current_execution_plan_step(&content);
        assert_eq!(current.as_deref(), Some("implement api"));
    }

    #[test]
    fn current_execution_plan_step_returns_none_when_no_current_marker() {
        let content = state_doc("Current", "- [x] one\n- [ ] two", "None");

        assert!(current_execution_plan_step(&content).is_none());
    }

    #[test]
    fn current_execution_plan_step_ignores_non_step_lines() {
        let content = state_doc(
            "Current",
            "\nnotes before plan\n- [>] implement follow mode\n- [ ] test",
            "None",
        );

        let current = current_execution_plan_step(&content);
        assert_eq!(current.as_deref(), Some("implement follow mode"));
    }
}
