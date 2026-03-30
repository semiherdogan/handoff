use crate::commands::prompt_output;
use crate::core::config;
use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::workspace;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::{Context, Result, anyhow};
use std::fs;

pub fn run(paths: &AiPaths, copy: bool, raw: bool) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let spec_exists = feature::file_exists(&active_feature_path, feature::SPEC_FILE);
    let design_exists = feature::file_exists(&active_feature_path, feature::DESIGN_FILE);
    ensure_start_ready(&state_content)?;

    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let context = build_start_prompt_context(spec_exists, design_exists);
    let prompt = prompts::start_prompt(
        &template_manager,
        &context,
        &prompts::PromptOptions {
            language_instruction: config.language_instruction(),
        },
    );

    prompt_output::output_prompt(&prompt, copy, raw)
}

fn ensure_start_ready(state_content: &str) -> Result<()> {
    match state::ensure_execution_plan_initialized(state_content) {
        Ok(()) => Ok(()),
        Err(error)
            if error.to_string()
                == "Execution plan not initialized. Run `handoff start` first." =>
        {
            Err(anyhow!(
                "Execution plan not ready. Run `handoff generate` first."
            ))
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn build_start_prompt_context(
    spec_exists: bool,
    design_exists: bool,
) -> prompts::StartPromptContext {
    prompts::StartPromptContext {
        read_files: "- .handoff/current/SESSION.md\n- .handoff/current/STATE.md\n- .handoff/current/FEATURE.md\n- .handoff/current/SPEC.md (if present)\n- .handoff/current/DESIGN.md (if present)".to_owned(),
        artifact_status: format!(
            "- FEATURE.md: present\n- SPEC.md: {}\n- DESIGN.md: {}\n- STATE.md: contains a valid execution plan\n- SESSION.md: present",
            if spec_exists { "present" } else { "missing" },
            if design_exists { "present" } else { "missing" }
        ),
        planning_mode: "Execution-only mode. The planning artifacts are ready. Do not regenerate the plan unless you are logically blocked by a contradiction in the existing markdown artifacts.".to_owned(),
        workflow_instructions: "1. Read SESSION.md, STATE.md, and FEATURE.md first.\n2. If SPEC.md and/or DESIGN.md exist, use them as supporting planning context.\n3. Continue from the existing execution plan in STATE.md.\n4. Implement only the current micro-step, then update STATE.md and SESSION.md after the step transition.\n5. Validate with build/tests before proceeding to the next micro-step.".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_start_prompt_context, ensure_start_ready};

    #[test]
    fn start_prompt_uses_execution_only_mode() {
        let context = build_start_prompt_context(true, true);

        assert!(context.planning_mode.contains("Execution-only mode"));
        assert!(
            context
                .workflow_instructions
                .contains("Continue from the existing execution plan")
        );
    }

    #[test]
    fn start_prompt_lists_missing_optional_design_artifact() {
        let context = build_start_prompt_context(true, false);

        assert!(context.artifact_status.contains("SPEC.md: present"));
        assert!(context.artifact_status.contains("DESIGN.md: missing"));
    }

    #[test]
    fn start_requires_generate_when_execution_plan_is_missing() {
        let error = ensure_start_ready(
            "# State\n\n# Current Step\nNot started\n\n# Execution Plan\nNot yet generated.\n",
        )
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "Execution plan not ready. Run `handoff generate` first."
        );
    }

    #[test]
    fn start_preserves_deterministic_invalid_plan_errors() {
        let error = ensure_start_ready(
            "# State\n\n# Current Step\nImplement\n\n# Execution Plan\n- [>] one\n- [>] two\n",
        )
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "Invalid execution plan: multiple current steps ([>]) found."
        );
    }
}
