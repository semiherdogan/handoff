use crate::core::paths::AiPaths;
use crate::core::state::ExecutionPlanValidation;
use crate::core::workflow;
use crate::core::workflow::WorkflowSnapshot;
use anyhow::Result;

pub fn run(paths: &AiPaths) -> Result<()> {
    let snapshot = workflow::load_snapshot(paths)?;

    println!("State memory");
    println!(
        "- Loaded active feature '{}' with workflow language '{}'.",
        snapshot.feature_name, snapshot.language
    );
    println!(
        "- Execution plan: {} ({})",
        snapshot.validation.status_label(),
        snapshot.validation.summary_message()
    );
    println!(
        "- Progress snapshot: {} completed, {} remaining.",
        snapshot.summary.completed_steps, snapshot.summary.remaining_steps
    );
    println!("Next task");
    println!("- {}", next_task_label(&snapshot));
    if let Some(reason) = workflow::blocked_reason(snapshot.validation, &snapshot.artifacts) {
        println!("Why blocked");
        println!("- {reason}");
    }
    println!("Next command");
    println!(
        "- {}",
        workflow::next_recommendation(&snapshot.summary, &snapshot.artifacts)
    );
    println!("Run mode");
    println!("- {}", run_mode_label(&snapshot));

    Ok(())
}

fn next_task_label(snapshot: &WorkflowSnapshot) -> String {
    if snapshot.artifacts.feature == "needs review" {
        return "Review and replace the placeholder content in .handoff/current/FEATURE.md."
            .to_owned();
    }

    match snapshot.validation {
        ExecutionPlanValidation::NotInitialized => {
            "Generate the planning artifacts from the reviewed feature brief.".to_owned()
        }
        ExecutionPlanValidation::Ready => snapshot
            .current_plan_step
            .clone()
            .unwrap_or_else(|| snapshot.summary.current_step.clone()),
        ExecutionPlanValidation::MultipleCurrentSteps => {
            "Fix STATE.md so exactly one step is marked as [>].".to_owned()
        }
        ExecutionPlanValidation::NoRemainingSteps => {
            "Execution plan is complete. Archive the feature or start a new one.".to_owned()
        }
    }
}

fn run_mode_label(snapshot: &WorkflowSnapshot) -> &'static str {
    if snapshot.artifacts.feature == "needs review" {
        return "No prompt yet. Review FEATURE.md first.";
    }

    match snapshot.validation {
        ExecutionPlanValidation::NotInitialized => "handoff run will emit a planning prompt.",
        ExecutionPlanValidation::Ready if snapshot.summary.completed_steps == 0 => {
            "handoff run will emit a start prompt."
        }
        ExecutionPlanValidation::Ready => "handoff run will emit a continuation prompt.",
        ExecutionPlanValidation::MultipleCurrentSteps => {
            "No prompt yet. Fix STATE.md so only one [>] step exists."
        }
        ExecutionPlanValidation::NoRemainingSteps => {
            "No prompt needed. The execution plan is already complete."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{next_task_label, run_mode_label};
    use crate::core::state::{ExecutionPlanValidation, StateSummary};
    use crate::core::workflow::{ArtifactStatus, WorkflowSnapshot};

    fn snapshot(validation: ExecutionPlanValidation) -> WorkflowSnapshot {
        WorkflowSnapshot {
            feature_name: "demo".to_owned(),
            language: "English".to_owned(),
            summary: StateSummary {
                current_step: "Current step summary".to_owned(),
                completed_steps: 0,
                current_steps: 1,
                remaining_steps: 1,
                known_risks: Vec::new(),
                execution_plan_initialized: validation == ExecutionPlanValidation::Ready,
            },
            validation,
            current_plan_step: Some("Implement parser".to_owned()),
            artifacts: ArtifactStatus {
                feature: "ready".to_owned(),
                spec: "ready".to_owned(),
                design: "ready".to_owned(),
                state: "planned".to_owned(),
                session: "ready".to_owned(),
            },
        }
    }

    #[test]
    fn next_task_uses_current_plan_step_when_ready() {
        assert_eq!(
            next_task_label(&snapshot(ExecutionPlanValidation::Ready)),
            "Implement parser"
        );
    }

    #[test]
    fn next_task_reports_plan_generation_when_uninitialized() {
        assert_eq!(
            next_task_label(&snapshot(ExecutionPlanValidation::NotInitialized)),
            "Generate the planning artifacts from the reviewed feature brief."
        );
    }

    #[test]
    fn run_mode_reports_continuation_for_in_progress_feature() {
        let mut snapshot = snapshot(ExecutionPlanValidation::Ready);
        snapshot.summary.completed_steps = 2;

        assert_eq!(
            run_mode_label(&snapshot),
            "handoff run will emit a continuation prompt."
        );
    }
}
