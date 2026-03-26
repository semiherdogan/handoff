use crate::core::config;
use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::state::{ExecutionPlanValidation, StateSummary};
use crate::core::workspace;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactStatus {
    feature: String,
    spec: String,
    design: String,
    state: String,
    session: String,
}

pub fn run(paths: &AiPaths, follow: bool) -> Result<()> {
    if follow {
        return run_follow(paths);
    }

    let (feature_name, language, summary, validation, _, artifacts) = load_status(paths)?;
    print_standard_status(&feature_name, &language, &summary, validation, &artifacts);
    Ok(())
}

fn run_follow(paths: &AiPaths) -> Result<()> {
    let status_refresh_interval = follow_status_refresh_interval();
    let spinner_tick_interval = follow_spinner_tick_interval();
    let mut spinner_idx = 0usize;
    let terminal_columns = terminal_columns();
    let mut last_status_refresh_at = Instant::now();
    let (
        mut feature_name,
        mut language,
        mut summary,
        mut validation,
        mut current_plan_step,
        mut artifacts,
    ) = load_status(paths)?;

    loop {
        if summary.remaining_steps == 0 {
            print!("\r\x1b[2K");
            io::stdout()
                .flush()
                .with_context(|| "Failed to flush stdout")?;
            print_standard_status(&feature_name, &language, &summary, validation, &artifacts);
            return Ok(());
        }

        let step = current_plan_step.as_deref().unwrap_or("No active [>] step");
        let frame = spinner_frame(spinner_idx);
        let follow_line = format_follow_line(frame, step, terminal_columns);

        print!("\r\x1b[2K{follow_line}");
        io::stdout()
            .flush()
            .with_context(|| "Failed to flush stdout")?;

        spinner_idx = (spinner_idx + 1) % 4;
        thread::sleep(spinner_tick_interval);

        if last_status_refresh_at.elapsed() >= status_refresh_interval {
            let (
                next_feature_name,
                next_language,
                next_summary,
                next_validation,
                next_plan_step,
                next_artifacts,
            ) = load_status(paths)?;
            feature_name = next_feature_name;
            language = next_language;
            summary = next_summary;
            validation = next_validation;
            current_plan_step = next_plan_step;
            artifacts = next_artifacts;
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

fn load_status(
    paths: &AiPaths,
) -> Result<(
    String,
    String,
    StateSummary,
    ExecutionPlanValidation,
    Option<String>,
    ArtifactStatus,
)> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let config = config::load(paths)?;
    let feature_name = workspace::resolve_current_feature_name(paths)?;
    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let summary = state::parse_state(&state_content);
    let validation = state::validate_execution_plan(&state_content);
    let current_plan_step = state::current_execution_plan_step(&state_content);
    let artifacts = artifact_statuses(&active_feature_path, &state_content)?;

    Ok((
        feature_name,
        config.language,
        summary,
        validation,
        current_plan_step,
        artifacts,
    ))
}

fn print_standard_status(
    feature_name: &str,
    language: &str,
    summary: &StateSummary,
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) {
    println!("Active feature: {feature_name}");
    println!("Language: {language}");
    println!(
        "Planning status: {}",
        planning_status(summary, validation, artifacts)
    );
    println!(
        "Execution plan validation: {} ({})",
        validation.status_label(),
        validation.summary_message()
    );
    println!("Current Step: {}", summary.current_step);
    println!("Remaining steps: {}", summary.remaining_steps);
    println!("Completed steps: {}", summary.completed_steps);
    println!("Artifacts:");
    println!("- FEATURE.md: {}", artifacts.feature);
    println!("- SPEC.md: {}", artifacts.spec);
    println!("- DESIGN.md: {}", artifacts.design);
    println!("- STATE.md: {}", artifacts.state);
    println!("- SESSION.md: {}", artifacts.session);

    if summary.known_risks.is_empty() {
        println!("Known risks: None");
    } else {
        println!("Known risks:");
        for risk in &summary.known_risks {
            println!("- {risk}");
        }
    }

    println!("Next: {}", next_recommendation(summary, artifacts));
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

fn artifact_statuses(feature_dir: &Path, state_content: &str) -> Result<ArtifactStatus> {
    Ok(ArtifactStatus {
        feature: classify_feature_or_session_artifact(
            &fs::read_to_string(feature_dir.join(feature::FEATURE_FILE)).with_context(|| {
                format!(
                    "Failed to read file: {}",
                    feature_dir.join(feature::FEATURE_FILE).display()
                )
            })?,
            "Describe the concrete objective of this feature.",
        ),
        spec: classify_generated_artifact(feature_dir, feature::SPEC_FILE, "Not yet generated.")?,
        design: classify_generated_artifact(
            feature_dir,
            feature::DESIGN_FILE,
            "Not yet generated.",
        )?,
        state: if state::ensure_execution_plan_initialized(state_content).is_ok() {
            "planned".to_owned()
        } else {
            "scaffolded".to_owned()
        },
        session: classify_feature_or_session_artifact(
            &fs::read_to_string(feature_dir.join(feature::SESSION_FILE)).with_context(|| {
                format!(
                    "Failed to read file: {}",
                    feature_dir.join(feature::SESSION_FILE).display()
                )
            })?,
            "None yet.",
        ),
    })
}

fn classify_generated_artifact(
    feature_dir: &Path,
    file_name: &str,
    placeholder: &str,
) -> Result<String> {
    let content = fs::read_to_string(feature_dir.join(file_name)).with_context(|| {
        format!(
            "Failed to read file: {}",
            feature_dir.join(file_name).display()
        )
    })?;

    Ok(if content.contains(placeholder) {
        "scaffolded".to_owned()
    } else {
        "ready".to_owned()
    })
}

fn classify_feature_or_session_artifact(content: &str, placeholder: &str) -> String {
    if content.contains(placeholder) {
        "needs review".to_owned()
    } else {
        "ready".to_owned()
    }
}

fn next_recommendation(summary: &StateSummary, artifacts: &ArtifactStatus) -> &'static str {
    if artifacts.feature == "needs review" {
        return "edit .handoff/current/FEATURE.md";
    }

    if !summary.execution_plan_initialized {
        return "run handoff generate --copy";
    }

    if summary.remaining_steps == 0 {
        return "archive the feature or start a new one";
    }

    "run handoff continue --copy"
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
        ArtifactStatus, classify_feature_or_session_artifact, follow_spinner_tick_interval,
        follow_status_refresh_interval, format_follow_line, next_recommendation, planning_status,
        spinner_frame,
    };
    use crate::core::state::{ExecutionPlanValidation, StateSummary};
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
    fn classify_feature_or_session_artifact_marks_placeholders_as_needing_review() {
        assert_eq!(
            classify_feature_or_session_artifact(
                "Describe the concrete objective of this feature.",
                "Describe the concrete objective of this feature."
            ),
            "needs review"
        );
        assert_eq!(
            classify_feature_or_session_artifact("Implemented API flow.", "None yet."),
            "ready"
        );
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
            "run handoff continue --copy"
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
            "run handoff generate --copy"
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
}
