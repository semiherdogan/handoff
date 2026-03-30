use crate::core::paths::AiPaths;
use crate::core::state::{self, ExecutionPlanValidation};
use crate::core::workflow::{self, ArtifactStatus};
use anyhow::{Result, anyhow};

pub fn run(paths: &AiPaths) -> Result<()> {
    let snapshot = workflow::load_snapshot(paths)?;

    print_validation_report(&snapshot.summary, snapshot.validation, &snapshot.artifacts);

    if should_fail(snapshot.validation) {
        return Err(anyhow!(validation_error_message(snapshot.validation)));
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
    artifacts: &ArtifactStatus,
) {
    println!("Execution plan: {}", validation.status_label());
    println!("Details: {}", validation.summary_message());
    if let Some(reason) = workflow::blocked_reason(validation, artifacts) {
        println!("Why blocked: {reason}");
    }
    println!("Progress:");
    println!("- Current Step: {}", summary.current_step);
    println!("- Remaining steps: {}", summary.remaining_steps);
    println!("- Completed steps: {}", summary.completed_steps);
    println!("Artifacts:");
    println!("- FEATURE.md: {}", artifacts.feature);
    println!("- SPEC.md: {}", artifacts.spec);
    println!("- DESIGN.md: {}", artifacts.design);
    println!("- STATE.md: {}", artifacts.state);
    println!("- SESSION.md: {}", artifacts.session);
}

#[cfg(test)]
mod tests {
    use super::{should_fail, validation_error_message};
    use crate::core::state::ExecutionPlanValidation;
    use crate::core::workflow::{ArtifactStatus, blocked_reason};

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
        let diagnostics = ArtifactStatus {
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
            ExecutionPlanValidation::NoRemainingSteps.artifact_status_label(),
            "complete"
        );
    }
}
