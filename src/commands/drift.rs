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
    feature::validate_feature_files(&active_feature_path)?;
    let feature_name = workspace::resolve_current_feature_name(paths)?;

    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let prompt = prompts::drift_prompt(
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
            title: "Drift Audit Prompt".to_owned(),
            what_happened: "Prepared a drift audit prompt for the active feature.".to_owned(),
            what_changed: format!(
                "No repository files changed. The prompt audits saved intent against implementation for '{feature_name}'."
            ),
            next: "Paste this prompt into your AI assistant to inspect implementation drift without changing code."
                .to_owned(),
        }),
    )
}
