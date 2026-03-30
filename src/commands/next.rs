use crate::core::paths::AiPaths;
use crate::core::state::ExecutionPlanValidation;
use crate::core::workflow;
use crate::core::workflow::WorkflowSnapshot;
use anyhow::Result;

pub fn run(paths: &AiPaths) -> Result<()> {
    let snapshot = workflow::load_snapshot(paths)?;

    println!("Active feature: {}", snapshot.feature_name);
    println!("Current step: {}", snapshot.summary.current_step);
    println!("Next task: {}", next_task_label(&snapshot));
    println!(
        "Suggested command: {}",
        workflow::next_recommendation(&snapshot.summary, &snapshot.artifacts)
    );

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

#[cfg(test)]
mod tests {
    use super::next_task_label;
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
}
