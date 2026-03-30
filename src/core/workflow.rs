use crate::core::config;
use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::state::{ExecutionPlanValidation, StateSummary};
use crate::core::workspace;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactStatus {
    pub feature: String,
    pub spec: String,
    pub design: String,
    pub state: String,
    pub session: String,
}

impl ArtifactStatus {
    pub fn all_healthy(&self) -> bool {
        self.feature == "ready"
            && self.spec == "ready"
            && self.design == "ready"
            && matches!(self.state.as_str(), "planned" | "complete")
            && self.session == "ready"
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowSnapshot {
    pub feature_name: String,
    pub language: String,
    pub summary: StateSummary,
    pub validation: ExecutionPlanValidation,
    pub current_plan_step: Option<String>,
    pub artifacts: ArtifactStatus,
}

pub fn load_snapshot(paths: &AiPaths) -> Result<WorkflowSnapshot> {
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

    Ok(WorkflowSnapshot {
        feature_name,
        language: config.language,
        summary,
        validation,
        current_plan_step,
        artifacts,
    })
}

pub fn blocked_reason(
    validation: ExecutionPlanValidation,
    artifacts: &ArtifactStatus,
) -> Option<&'static str> {
    if artifacts.feature == "needs review" {
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

pub fn next_recommendation(summary: &StateSummary, artifacts: &ArtifactStatus) -> &'static str {
    if artifacts.feature == "needs review" {
        return "edit .handoff/current/FEATURE.md";
    }

    if !summary.execution_plan_initialized {
        return "run handoff run --copy";
    }

    if summary.remaining_steps == 0 {
        return "archive the feature or start a new one";
    }

    "run handoff run --copy"
}

fn artifact_statuses(feature_dir: &Path, state_content: &str) -> Result<ArtifactStatus> {
    let state_validation = state::validate_execution_plan(state_content);

    Ok(ArtifactStatus {
        feature: feature::classify_feature_artifact(feature_dir)?,
        spec: feature::classify_spec_artifact(feature_dir)?,
        design: feature::classify_design_artifact(feature_dir)?,
        state: state_validation.artifact_status_label().to_owned(),
        session: feature::classify_session_artifact(feature_dir)?,
    })
}

#[cfg(test)]
mod tests {
    use super::{ArtifactStatus, blocked_reason, next_recommendation};
    use crate::core::state::{ExecutionPlanValidation, StateSummary};

    #[test]
    fn next_recommendation_prioritizes_feature_editing_before_run() {
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
    fn next_recommendation_uses_run_for_ready_plan() {
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
    fn blocked_reason_prioritizes_feature_review() {
        let artifacts = ArtifactStatus {
            feature: "needs review".to_owned(),
            spec: "ready".to_owned(),
            design: "ready".to_owned(),
            state: "scaffolded".to_owned(),
            session: "ready".to_owned(),
        };

        assert_eq!(
            blocked_reason(ExecutionPlanValidation::NotInitialized, &artifacts),
            Some("FEATURE.md still contains scaffold content and needs review.")
        );
    }
}
