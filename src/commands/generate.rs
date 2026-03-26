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

    let template_manager = TemplateManager::new(paths);
    let config = config::load(paths)?;
    let prompt = prompts::generate_prompt(
        &template_manager,
        &prompts::PromptOptions {
            language_instruction: config.language_instruction(),
        },
    );

    prompt_output::output_prompt(&prompt, copy, raw)
}
