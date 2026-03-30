use crate::core::paths::AiPaths;
use crate::core::state::{ExecutionPlanValidation, StateSummary};
use crate::core::workflow::{self, ArtifactStatus};
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

pub fn run(paths: &AiPaths, follow: bool) -> Result<()> {
    if follow {
        return run_follow(paths);
    }

    let snapshot = workflow::load_snapshot(paths)?;
    print_standard_status(
        &snapshot.feature_name,
        &snapshot.language,
        &snapshot.summary,
        snapshot.validation,
        &snapshot.artifacts,
    );
    Ok(())
}

fn run_follow(paths: &AiPaths) -> Result<()> {
    let status_refresh_interval = follow_status_refresh_interval();
    let spinner_tick_interval = follow_spinner_tick_interval();
    let mut spinner_idx = 0usize;
    let terminal_columns = terminal_columns();
    let mut last_status_refresh_at = Instant::now();
    let mut snapshot = workflow::load_snapshot(paths)?;

    loop {
        if snapshot.validation != ExecutionPlanValidation::Ready
            || snapshot.summary.remaining_steps == 0
        {
            print!("\r\x1b[2K");
            io::stdout()
                .flush()
                .with_context(|| "Failed to flush stdout")?;
            print_standard_status(
                &snapshot.feature_name,
                &snapshot.language,
                &snapshot.summary,
                snapshot.validation,
                &snapshot.artifacts,
            );
            return Ok(());
        }

        let step = snapshot
            .current_plan_step
            .as_deref()
            .unwrap_or("No active [>] step");
        let frame = spinner_frame(spinner_idx);
        let follow_line = format_follow_line(frame, step, terminal_columns);

        print!("\r\x1b[2K{follow_line}");
        io::stdout()
            .flush()
            .with_context(|| "Failed to flush stdout")?;

        spinner_idx = (spinner_idx + 1) % 4;
        thread::sleep(spinner_tick_interval);

        if last_status_refresh_at.elapsed() >= status_refresh_interval {
            snapshot = workflow::load_snapshot(paths)?;
            last_status_refresh_at = Instant::now();
        }
    }
}

fn follow_status_refresh_interval() -> Duration {
    Duration::from_secs(2)
}

fn follow_spinner_tick_interval() -> Duration {
    // Root cause: follow mode advanced the spinner only when status was reloaded every 2s,
    // so the loader appeared stalled instead of showing responsive liveness.
    Duration::from_millis(125)
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

fn print_standard_status(
    feature_name: &str,
    language: &str,
    summary: &StateSummary,
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) {
    println!("State memory");
    println!("- Active feature: {feature_name}");
    println!("- Workflow language: {language}");
    println!(
        "- Planning status: {}",
        planning_status(summary, validation, artifacts)
    );
    println!(
        "- Execution plan: {} ({})",
        validation.status_label(),
        validation.summary_message()
    );
    if let Some(reason) = workflow::blocked_reason(validation, artifacts) {
        println!("Why blocked: {reason}");
    }
    println!("Progress");
    println!("- Current Step: {}", summary.current_step);
    println!("- Remaining steps: {}", summary.remaining_steps);
    println!("- Completed steps: {}", summary.completed_steps);
    println!(
        "- State continuity: {}",
        continuity_signal(summary, validation, artifacts)
    );
    if !artifacts.all_healthy() {
        println!("Artifacts");
        println!("- FEATURE.md: {}", artifacts.feature);
        println!("- SPEC.md: {}", artifacts.spec);
        println!("- DESIGN.md: {}", artifacts.design);
        println!("- STATE.md: {}", artifacts.state);
        println!("- SESSION.md: {}", artifacts.session);
    }

    if summary.known_risks.is_empty() {
        println!("Known risks: None");
    } else {
        println!("Known risks");
        for risk in &summary.known_risks {
            println!("- {risk}");
        }
    }

    println!("Next");
    println!(
        "- Command: {}",
        workflow::next_recommendation(summary, artifacts)
    );
    println!("- Focus: {}", next_focus(summary, validation, artifacts));
}

fn planning_status(
    summary: &StateSummary,
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) -> &'static str {
    if artifacts.feature == "needs review" {
        return "feature needs review";
    }

    match validation {
        ExecutionPlanValidation::Ready => "ready to execute",
        ExecutionPlanValidation::NotInitialized => "needs generation",
        ExecutionPlanValidation::MultipleCurrentSteps => "invalid execution plan",
        ExecutionPlanValidation::NoRemainingSteps if summary.remaining_steps == 0 => {
            "execution complete"
        }
        ExecutionPlanValidation::NoRemainingSteps => "ready to archive",
    }
}

fn continuity_signal(
    summary: &StateSummary,
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) -> &'static str {
    if artifacts.feature == "needs review" {
        return "feature brief still needs review before the workflow can carry context forward";
    }

    match validation {
        ExecutionPlanValidation::NotInitialized => {
            "workspace scaffold exists, but planning state has not been generated yet"
        }
        ExecutionPlanValidation::Ready if summary.completed_steps == 0 => {
            "planning is ready and execution can start from the saved state"
        }
        ExecutionPlanValidation::Ready => {
            "execution can continue from the saved plan and session context"
        }
        ExecutionPlanValidation::MultipleCurrentSteps => {
            "continuity is blocked until STATE.md has exactly one [>] step"
        }
        ExecutionPlanValidation::NoRemainingSteps => {
            "execution history is preserved and the feature is ready to close out"
        }
    }
}

fn next_focus(
    summary: &StateSummary,
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) -> String {
    if artifacts.feature == "needs review" {
        return "Replace the placeholder brief in .handoff/current/FEATURE.md.".to_owned();
    }

    match validation {
        ExecutionPlanValidation::NotInitialized => {
            "Generate SPEC.md, optional DESIGN.md, STATE.md, and SESSION.md.".to_owned()
        }
        ExecutionPlanValidation::Ready => summary.current_step.clone(),
        ExecutionPlanValidation::MultipleCurrentSteps => {
            "Repair STATE.md so only one step is marked as [>].".to_owned()
        }
        ExecutionPlanValidation::NoRemainingSteps => {
            "Archive the feature or initialize the next one.".to_owned()
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
    use super::{
        continuity_signal, follow_spinner_tick_interval, follow_status_refresh_interval,
        format_follow_line, next_focus, planning_status, spinner_frame,
    };
    use crate::core::state::{ExecutionPlanValidation, StateSummary};
    use crate::core::workflow::{ArtifactStatus, blocked_reason, next_recommendation};
    use std::time::Duration;

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

    #[test]
    fn follow_spinner_tick_interval_is_faster_than_status_refresh() {
        assert!(follow_spinner_tick_interval() < follow_status_refresh_interval());
    }

    #[test]
    fn follow_spinner_tick_interval_is_quick_for_responsive_loader() {
        assert_eq!(follow_spinner_tick_interval(), Duration::from_millis(125));
    }

    #[test]
    fn next_recommendation_prioritizes_feature_editing_before_planning() {
        let summary = StateSummary {
            current_step: "Not started".to_owned(),
            completed_steps: 0,
            current_steps: 0,
            remaining_steps: 0,
            known_risks: Vec::new(),
            execution_plan_initialized: false,
        };
        let artifacts = ArtifactStatus {
            feature: "needs review".to_owned(),
            spec: "scaffolded".to_owned(),
            design: "scaffolded".to_owned(),
            state: "scaffolded".to_owned(),
            session: "needs review".to_owned(),
        };

        assert_eq!(
            next_recommendation(&summary, &artifacts),
            "edit .handoff/current/FEATURE.md"
        );
    }

    #[test]
    fn next_recommendation_suggests_continue_when_plan_exists() {
        let summary = StateSummary {
            current_step: "Implement API".to_owned(),
            completed_steps: 1,
            current_steps: 1,
            remaining_steps: 2,
            known_risks: Vec::new(),
            execution_plan_initialized: true,
        };
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "planned".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            next_recommendation(&summary, &artifacts),
            "run handoff run --copy"
        );
    }

    #[test]
    fn next_recommendation_suggests_generate_when_plan_is_missing() {
        let summary = StateSummary {
            current_step: "Not started".to_owned(),
            completed_steps: 0,
            current_steps: 0,
            remaining_steps: 0,
            known_risks: Vec::new(),
            execution_plan_initialized: false,
        };
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "scaffolded".to_owned(),
            state: "scaffolded".to_owned(),
            session: "needs review".to_owned(),
        };

        assert_eq!(
            next_recommendation(&summary, &artifacts),
            "run handoff run --copy"
        );
    }

    #[test]
    fn planning_status_reports_invalid_plan() {
        let summary = StateSummary {
            current_step: "Implement".to_owned(),
            completed_steps: 0,
            current_steps: 2,
            remaining_steps: 2,
            known_risks: Vec::new(),
            execution_plan_initialized: false,
        };
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "scaffolded".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            planning_status(
                &summary,
                ExecutionPlanValidation::MultipleCurrentSteps,
                &artifacts
            ),
            "invalid execution plan"
        );
    }

    #[test]
    fn blocked_reason_reports_uninitialized_state_plan() {
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "scaffolded".to_owned(),
            state: "scaffolded".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            blocked_reason(ExecutionPlanValidation::NotInitialized, &artifacts),
            Some("STATE.md does not contain a valid execution plan yet.")
        );
    }

    #[test]
    fn blocked_reason_reports_multiple_current_steps() {
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "scaffolded".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            blocked_reason(ExecutionPlanValidation::MultipleCurrentSteps, &artifacts),
            Some("STATE.md contains multiple current steps ([>]) and must be fixed.")
        );
        assert_eq!(
            blocked_reason(ExecutionPlanValidation::Ready, &artifacts),
            None
        );
    }

    #[test]
    fn continuity_signal_reports_resume_ready_state() {
        let summary = StateSummary {
            current_step: "Implement".to_owned(),
            completed_steps: 2,
            current_steps: 1,
            remaining_steps: 1,
            known_risks: Vec::new(),
            execution_plan_initialized: true,
        };
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "planned".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            continuity_signal(&summary, ExecutionPlanValidation::Ready, &artifacts),
            "execution can continue from the saved plan and session context"
        );
    }

    #[test]
    fn next_focus_prefers_feature_review_when_brief_is_placeholder() {
        let summary = StateSummary {
            current_step: "Not started".to_owned(),
            completed_steps: 0,
            current_steps: 0,
            remaining_steps: 0,
            known_risks: Vec::new(),
            execution_plan_initialized: false,
        };
        let artifacts = ArtifactStatus {
            feature: "needs review".to_owned(),
            spec: "scaffolded".to_owned(),
            design: "scaffolded".to_owned(),
            state: "scaffolded".to_owned(),
            session: "needs review".to_owned(),
        };

        assert_eq!(
            next_focus(
                &summary,
                ExecutionPlanValidation::NotInitialized,
                &artifacts
            ),
            "Replace the placeholder brief in .handoff/current/FEATURE.md."
        );
    }

    #[test]
    fn artifact_status_all_healthy_accepts_complete_state() {
        let artifacts = ArtifactStatus {
            feature: "ready".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "complete".to_owned(),
            session: "ready".to_owned(),
        };

        assert!(artifacts.all_healthy());
    }
}
