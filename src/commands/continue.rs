use crate::commands::prompt_output;
use crate::core::command_name;
use crate::core::config;
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
    let feature_name = workspace::resolve_current_feature_name(paths)?;

    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;
    state::ensure_execution_plan_initialized_with_command(
        &state_content,
        &command_name::current(),
    )?;

    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let prompt = prompts::continuation_prompt(
        &template_manager,
        &prompts::PromptOptions {
            language_instruction: config.workflow_language_instruction(),
        },
    );

    prompt_output::output_prompt_with_summary(
        &prompt,
        copy,
        raw,
        Some(prompt_output::PromptSummary {
            title: "Continuation Prompt".to_owned(),
            what_happened: "Prepared a continuation prompt from the saved project state."
                .to_owned(),
            what_changed: format!(
                "No repository files changed. The prompt loads SESSION.md and STATE.md from the active feature workspace '{feature_name}'."
            ),
            next: "Paste this prompt into your AI assistant to resume the remaining micro-steps from the current state."
                .to_owned(),
        }),
    )
}
