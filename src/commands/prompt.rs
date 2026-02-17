use crate::cli::PromptKind;
use crate::core::paths::AiPaths;
use crate::commands::prompt_output;
use crate::templates::manager::TemplateManager;
use crate::templates::prompts;
use anyhow::Result;

pub fn run(paths: &AiPaths, kind: Option<PromptKind>, copy: bool, raw: bool) -> Result<()> {
    let template_manager = TemplateManager::new(paths);
    let selected = kind.unwrap_or(PromptKind::Continue);

    let prompt = match selected {
        PromptKind::Start => prompts::start_prompt(&template_manager),
        PromptKind::Continue => prompts::continuation_prompt(&template_manager),
    };

    prompt_output::output_prompt(&prompt, copy, raw)
}
