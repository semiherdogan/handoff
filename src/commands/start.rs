use crate::commands::prompt_output;
use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::workspace;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::{Context, Result};
use std::fs;

pub fn run(paths: &AiPaths, copy: bool, raw: bool) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    let spec_exists = feature::file_exists(&active_feature_path, feature::SPEC_FILE);
    let design_exists = feature::file_exists(&active_feature_path, feature::DESIGN_FILE);
    let plan_ready = state::ensure_execution_plan_initialized(&state_content).is_ok();

    let template_manager = TemplateManager::new(paths);
    let context = build_start_prompt_context(spec_exists, design_exists, plan_ready);
    let prompt = prompts::start_prompt(&template_manager, &context);

    prompt_output::output_prompt(&prompt, copy, raw)
}

fn build_start_prompt_context(
    spec_exists: bool,
    design_exists: bool,
    plan_ready: bool,
) -> prompts::StartPromptContext {
    if plan_ready {
        return prompts::StartPromptContext {
            read_files: "- .handoff/current/SESSION.md\n- .handoff/current/STATE.md\n- .handoff/current/FEATURE.md\n- .handoff/current/SPEC.md (if present)\n- .handoff/current/DESIGN.md (if present)".to_owned(),
            artifact_status: format!(
                "- FEATURE.md: present\n- SPEC.md: {}\n- DESIGN.md: {}\n- STATE.md: contains a valid execution plan\n- SESSION.md: present",
                if spec_exists { "present" } else { "missing" },
                if design_exists { "present" } else { "missing" }
            ),
            planning_mode: "Execution plan already exists. Do not restart planning. Reuse the current plan and begin with the active micro-step.".to_owned(),
            workflow_instructions: "1. Read SESSION.md, STATE.md, and FEATURE.md first.\n2. If SPEC.md and/or DESIGN.md exist, use them as supporting planning context.\n3. Continue from the existing execution plan in STATE.md.\n4. Do not regenerate the plan unless you are logically blocked by a contradiction in the planning artifacts.\n5. Implement the current step, update STATE.md and SESSION.md after each step transition, and validate via build/tests.".to_owned(),
        };
    }

    if spec_exists {
        return prompts::StartPromptContext {
            read_files: "- .handoff/current/FEATURE.md\n- .handoff/current/SPEC.md\n- .handoff/current/DESIGN.md (if present)\n- .handoff/current/STATE.md\n- .handoff/current/SESSION.md".to_owned(),
            artifact_status: if design_exists {
                "- FEATURE.md: present\n- SPEC.md: present\n- DESIGN.md: present\n- STATE.md: no valid execution plan yet\n- SESSION.md: present"
            } else {
                "- FEATURE.md: present\n- SPEC.md: present\n- DESIGN.md: missing\n- STATE.md: no valid execution plan yet\n- SESSION.md: present"
            }
            .to_owned(),
            planning_mode: "Spec-first planning mode. Reuse SPEC.md as the behavioral source of truth, create DESIGN.md only if complexity justifies it, then generate STATE.md and begin execution.".to_owned(),
            workflow_instructions: "1. Read FEATURE.md and SPEC.md.\n2. If DESIGN.md is missing, decide whether the feature is complex enough to require one. Create it only when it improves implementation quality.\n3. Generate or rewrite the execution plan in STATE.md from SPEC.md and optional DESIGN.md.\n4. Keep the plan to practical micro-steps with exactly one [>] if work remains.\n5. After the plan is ready, begin implementing the current step and keep STATE.md and SESSION.md updated.".to_owned(),
        };
    }

    prompts::StartPromptContext {
        read_files: "- .handoff/current/FEATURE.md\n- .handoff/current/STATE.md\n- .handoff/current/SESSION.md".to_owned(),
        artifact_status: "- FEATURE.md: present\n- SPEC.md: missing\n- DESIGN.md: missing\n- STATE.md: no valid execution plan yet\n- SESSION.md: present".to_owned(),
        planning_mode: "Full orchestration mode. Create the missing planning artifacts in order before implementation: SPEC.md, optional DESIGN.md, then STATE.md.".to_owned(),
        workflow_instructions: "1. Read FEATURE.md and normalize it into a concise SPEC.md with scope, requirements, edge cases, acceptance criteria, assumptions, and non-goals.\n2. Create DESIGN.md only if the task is complex enough to benefit from explicit technical planning.\n3. Generate the execution plan in STATE.md from SPEC.md and optional DESIGN.md.\n4. Keep the plan to practical micro-steps with exactly one [>] if work remains.\n5. Begin implementing the current step, validating with build/tests, and rewrite SESSION.md after each step transition.".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::build_start_prompt_context;

    #[test]
    fn start_prompt_prefers_existing_execution_plan() {
        let context = build_start_prompt_context(true, true, true);

        assert!(
            context
                .planning_mode
                .contains("Execution plan already exists")
        );
        assert!(
            context
                .workflow_instructions
                .contains("Continue from the existing execution plan")
        );
    }

    #[test]
    fn start_prompt_uses_spec_first_mode_when_spec_exists_without_plan() {
        let context = build_start_prompt_context(true, false, false);

        assert!(context.planning_mode.contains("Spec-first planning mode"));
        assert!(
            context
                .workflow_instructions
                .contains("Read FEATURE.md and SPEC.md")
        );
    }

    #[test]
    fn start_prompt_falls_back_to_full_orchestration_when_spec_is_missing() {
        let context = build_start_prompt_context(false, false, false);

        assert!(context.planning_mode.contains("Full orchestration mode"));
        assert!(
            context
                .workflow_instructions
                .contains("normalize it into a concise SPEC.md")
        );
    }
}
