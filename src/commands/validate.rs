use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state::{self, ExecutionPlanValidation};
use crate::core::workspace;
use anyhow::{Context, Result, anyhow};
use std::fs;

pub fn run(paths: &AiPaths) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let summary = state::parse_state(&state_content);
    let validation = state::validate_execution_plan(&state_content);

    if should_fail(validation) {
        return Err(anyhow!(validation_error_message(validation)));
    }

    println!("Execution plan: {}", validation.status_label());
    println!("Details: {}", validation.summary_message());
    println!("Current Step: {}", summary.current_step);
    println!("Remaining steps: {}", summary.remaining_steps);
    println!("Completed steps: {}", summary.completed_steps);

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

#[cfg(test)]
mod tests {
    use super::{should_fail, validation_error_message};
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
}
