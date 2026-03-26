use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state::{self, ExecutionPlanValidation};
use crate::core::workspace;
use anyhow::{Context, Result, anyhow};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactDiagnostics {
    feature: String,
    spec: String,
    design: String,
    state: String,
    session: String,
}

pub fn run(paths: &AiPaths) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let summary = state::parse_state(&state_content);
    let validation = state::validate_execution_plan(&state_content);
    let diagnostics = artifact_diagnostics(&active_feature_path, &state_content)?;

    print_validation_report(&summary, validation, &diagnostics);

    if should_fail(validation) {
        return Err(anyhow!(validation_error_message(validation)));
    }

    Ok(())
}

fn should_fail(validation: ExecutionPlanValidation) -> bool {
    matches!(
        validation,
        ExecutionPlanValidation::NotInitialized | ExecutionPlanValidation::MultipleCurrentSteps
    )
}

fn validation_error_message(validation: ExecutionPlanValidation) -> &'static str {
    match validation {
        ExecutionPlanValidation::NotInitialized => {
            "Execution plan not initialized. Run `handoff generate` first."
        }
        ExecutionPlanValidation::MultipleCurrentSteps => {
            "Invalid execution plan: multiple current steps ([>]) found."
        }
        ExecutionPlanValidation::Ready => "Execution plan is valid.",
        ExecutionPlanValidation::NoRemainingSteps => "No remaining steps to continue.",
    }
}

fn print_validation_report(
    summary: &state::StateSummary,
    validation: ExecutionPlanValidation,
    diagnostics: &ArtifactDiagnostics,
) {
    println!("Execution plan: {}", validation.status_label());
    println!("Details: {}", validation.summary_message());
    if let Some(reason) = blocked_reason(validation, diagnostics) {
        println!("Why blocked: {reason}");
    }
    println!("Progress:");
    println!("- Current Step: {}", summary.current_step);
    println!("- Remaining steps: {}", summary.remaining_steps);
    println!("- Completed steps: {}", summary.completed_steps);
    println!("Artifacts:");
    println!("- FEATURE.md: {}", diagnostics.feature);
    println!("- SPEC.md: {}", diagnostics.spec);
    println!("- DESIGN.md: {}", diagnostics.design);
    println!("- STATE.md: {}", diagnostics.state);
    println!("- SESSION.md: {}", diagnostics.session);
}

fn blocked_reason(
    validation: ExecutionPlanValidation,
    diagnostics: &ArtifactDiagnostics,
) -> Option<&'static str> {
    if diagnostics.feature == "needs review" {
        return Some("FEATURE.md still contains scaffold content and needs review.");
    }

    match validation {
        ExecutionPlanValidation::NotInitialized => {
            Some("STATE.md does not contain a valid execution plan yet.")
        }
        ExecutionPlanValidation::MultipleCurrentSteps => {
            Some("STATE.md contains multiple current steps ([>]) and must be fixed.")
        }
        ExecutionPlanValidation::Ready | ExecutionPlanValidation::NoRemainingSteps => None,
    }
}

fn artifact_diagnostics(
    feature_dir: &std::path::Path,
    state_content: &str,
) -> Result<ArtifactDiagnostics> {
    Ok(ArtifactDiagnostics {
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
        state: classify_state_artifact(state::validate_execution_plan(state_content)).to_owned(),
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

fn classify_state_artifact(validation: ExecutionPlanValidation) -> &'static str {
    match validation {
        ExecutionPlanValidation::Ready => "planned",
        ExecutionPlanValidation::NoRemainingSteps => "complete",
        ExecutionPlanValidation::NotInitialized => "scaffolded",
        ExecutionPlanValidation::MultipleCurrentSteps => "invalid",
    }
}

fn classify_generated_artifact(
    feature_dir: &std::path::Path,
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

#[cfg(test)]
mod tests {
    use super::{
        ArtifactDiagnostics, blocked_reason, classify_state_artifact, should_fail,
        validation_error_message,
    };
    use crate::core::state::ExecutionPlanValidation;

    #[test]
    fn validate_fails_only_for_invalid_or_uninitialized_plans() {
        assert!(should_fail(ExecutionPlanValidation::NotInitialized));
        assert!(should_fail(ExecutionPlanValidation::MultipleCurrentSteps));
        assert!(!should_fail(ExecutionPlanValidation::Ready));
        assert!(!should_fail(ExecutionPlanValidation::NoRemainingSteps));
    }

    #[test]
    fn validate_uses_generate_guidance_for_uninitialized_plan() {
        assert_eq!(
            validation_error_message(ExecutionPlanValidation::NotInitialized),
            "Execution plan not initialized. Run `handoff generate` first."
        );
    }

    #[test]
    fn validate_blocked_reason_prioritizes_feature_review() {
        let diagnostics = ArtifactDiagnostics {
            feature: "needs review".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "scaffolded".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            blocked_reason(ExecutionPlanValidation::NotInitialized, &diagnostics),
            Some("FEATURE.md still contains scaffold content and needs review.")
        );
    }

    #[test]
    fn validate_classifies_finished_plan_as_complete() {
        assert_eq!(
            classify_state_artifact(ExecutionPlanValidation::NoRemainingSteps),
            "complete"
        );
    }
}
