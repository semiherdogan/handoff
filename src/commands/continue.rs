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
    state::ensure_execution_plan_initialized(&state_content)?;

    let template_manager = TemplateManager::new(paths);
    let prompt = prompts::continuation_prompt(&template_manager);

    prompt_output::output_prompt(&prompt, copy, raw)
}
