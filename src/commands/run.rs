use crate::commands::prompt_output;
use crate::commands::start::build_start_prompt_context;
use crate::core::config;
use crate::core::paths::AiPaths;
use crate::core::state::ExecutionPlanValidation;
use crate::core::workflow::{self, WorkflowSnapshot};
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::{Result, anyhow};

pub fn run(paths: &AiPaths, copy: bool, raw: bool) -> Result<()> {
    let snapshot = workflow::load_snapshot(paths)?;
    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let prompt_options = prompts::PromptOptions {
        language_instruction: config.workflow_language_instruction(),
    };
    let prepared = prepare_run_prompt(&snapshot, &template_manager, &prompt_options)?;

    prompt_output::output_prompt_with_summary(
        &prepared.prompt,
        copy,
        raw,
        Some(prompt_output::PromptSummary {
            title: "Run Prompt".to_owned(),
            what_happened: format!(
                "Loaded the active state and selected the '{}' prompt mode automatically.",
                prepared.mode_label
            ),
            what_changed: format!(
                "No repository files changed. Active feature: '{}'. Workflow language: '{}'. Current step: '{}'.",
                snapshot.feature_name, snapshot.language, snapshot.summary.current_step
            ),
            next: format!(
                "Paste this prompt into your AI assistant. Current next task: {}",
                prepared.next_task
            ),
        }),
    )
}

struct PreparedRunPrompt {
    prompt: String,
    mode_label: &'static str,
    next_task: String,
}

fn prepare_run_prompt(
    snapshot: &WorkflowSnapshot,
    template_manager: &TemplateManager,
    prompt_options: &prompts::PromptOptions,
) -> Result<PreparedRunPrompt> {
    if snapshot.artifacts.feature == "needs review" {
        return Err(anyhow!(
            "FEATURE.md still contains scaffold content and needs review."
        ));
    }

    match snapshot.validation {
        ExecutionPlanValidation::NotInitialized => Ok(PreparedRunPrompt {
            prompt: prompts::generate_prompt(template_manager, prompt_options),
            mode_label: "generate",
            next_task:
                "Generate SPEC.md, optional DESIGN.md, STATE.md, and SESSION.md from FEATURE.md."
                    .to_owned(),
        }),
        ExecutionPlanValidation::Ready if snapshot.summary.completed_steps == 0 => {
            Ok(PreparedRunPrompt {
                prompt: prompts::start_prompt(
                    template_manager,
                    &build_start_prompt_context(
                        snapshot.artifacts.spec == "ready",
                        snapshot.artifacts.design == "ready",
                    ),
                    prompt_options,
                ),
                mode_label: "start",
                next_task: active_step_label(snapshot),
            })
        }
        ExecutionPlanValidation::Ready => Ok(PreparedRunPrompt {
            prompt: prompts::continuation_prompt(template_manager, prompt_options),
            mode_label: "continue",
            next_task: active_step_label(snapshot),
        }),
        ExecutionPlanValidation::MultipleCurrentSteps => Err(anyhow!(
            "Invalid execution plan: multiple current steps ([>]) found."
        )),
        ExecutionPlanValidation::NoRemainingSteps => {
            Err(anyhow!("No remaining steps to continue."))
        }
    }
}

fn active_step_label(snapshot: &WorkflowSnapshot) -> String {
    snapshot
        .current_plan_step
        .clone()
        .unwrap_or_else(|| snapshot.summary.current_step.clone())
}

#[cfg(test)]
mod tests {
    use super::{active_step_label, prepare_run_prompt};
    use crate::core::state::{ExecutionPlanValidation, StateSummary};
    use crate::core::test_utils::make_temp_base;
    use crate::core::workflow::{ArtifactStatus, WorkflowSnapshot};
    use crate::templates::manager::TemplateManager;
    use crate::templates::prompts::PromptOptions;
    use std::fs;

    fn snapshot(validation: ExecutionPlanValidation) -> WorkflowSnapshot {
        WorkflowSnapshot {
            feature_name: "demo".to_owned(),
            language: "English".to_owned(),
            summary: StateSummary {
                current_step: "Implement parser".to_owned(),
                completed_steps: 0,
                current_steps: 1,
                remaining_steps: 2,
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
    fn run_uses_generate_mode_when_plan_is_missing() {
        let base = make_temp_base("run-generate");
        let manager = TemplateManager::new(&crate::core::paths::AiPaths::discover(&base));
        let prepared = prepare_run_prompt(
            &snapshot(ExecutionPlanValidation::NotInitialized),
            &manager,
            &PromptOptions {
                language_instruction: "Write prose in English.".to_owned(),
            },
        )
        .expect("should build generate prompt");

        assert_eq!(prepared.mode_label, "generate");
        assert!(prepared.prompt.contains("planning generation mode"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn run_uses_continue_mode_after_progress_exists() {
        let base = make_temp_base("run-continue");
        let manager = TemplateManager::new(&crate::core::paths::AiPaths::discover(&base));
        let mut snapshot = snapshot(ExecutionPlanValidation::Ready);
        snapshot.summary.completed_steps = 2;
        let prepared = prepare_run_prompt(
            &snapshot,
            &manager,
            &PromptOptions {
                language_instruction: "Write prose in English.".to_owned(),
            },
        )
        .expect("should build continue prompt");

        assert_eq!(prepared.mode_label, "continue");
        assert!(
            prepared
                .prompt
                .contains("continuing an autonomous development session")
        );

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn active_step_label_falls_back_to_current_step_summary() {
        let mut snapshot = snapshot(ExecutionPlanValidation::Ready);
        snapshot.current_plan_step = None;

        assert_eq!(active_step_label(&snapshot), "Implement parser");
    }
}
