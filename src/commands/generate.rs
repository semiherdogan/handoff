use crate::commands::prompt_output;
use crate::core::config;
use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::workspace;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::Result;

pub fn run(paths: &AiPaths, copy: bool, raw: bool) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_file(&active_feature_path, feature::FEATURE_FILE)?;
    let feature_name = workspace::resolve_current_feature_name(paths)?;

    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let prompt = prompts::generate_prompt(
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
            title: "Planning Prompt".to_owned(),
            what_happened: "Prepared a planning prompt from the active feature brief.".to_owned(),
            what_changed: format!(
                "No repository files changed. The prompt is scoped to the active feature workspace '{feature_name}'."
            ),
            next: "Paste this prompt into your AI assistant to refresh SPEC.md, optional DESIGN.md, STATE.md, and SESSION.md."
                .to_owned(),
        }),
    )
}
