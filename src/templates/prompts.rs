use crate::templates::manager::{
    DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME, DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME,
    DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME, DEFAULT_SPEC_PROMPT_TEMPLATE_NAME,
    DEFAULT_START_PROMPT_TEMPLATE_NAME, DEFAULT_TASKS_PROMPT_TEMPLATE_NAME, TemplateManager,
};

pub struct PromptOptions {
    pub language_instruction: String,
}

pub struct StartPromptContext {
    pub read_files: String,
    pub artifact_status: String,
    pub planning_mode: String,
    pub workflow_instructions: String,
}

pub fn generate_prompt(template_manager: &TemplateManager, options: &PromptOptions) -> String {
    apply_shared_prompt_options(
        template_manager.get_template(DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME),
        options,
    )
}

pub fn start_prompt(
    template_manager: &TemplateManager,
    context: &StartPromptContext,
    options: &PromptOptions,
) -> String {
    apply_shared_prompt_options(
        template_manager
            .get_template(DEFAULT_START_PROMPT_TEMPLATE_NAME)
            .replace("{{read_files}}", &context.read_files)
            .replace("{{artifact_status}}", &context.artifact_status)
            .replace("{{planning_mode}}", &context.planning_mode)
            .replace("{{workflow_instructions}}", &context.workflow_instructions),
        options,
    )
}

pub fn spec_prompt(template_manager: &TemplateManager, options: &PromptOptions) -> String {
    apply_shared_prompt_options(
        template_manager.get_template(DEFAULT_SPEC_PROMPT_TEMPLATE_NAME),
        options,
    )
}

pub fn design_prompt(template_manager: &TemplateManager, options: &PromptOptions) -> String {
    apply_shared_prompt_options(
        template_manager.get_template(DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME),
        options,
    )
}

pub fn tasks_prompt(template_manager: &TemplateManager, options: &PromptOptions) -> String {
    apply_shared_prompt_options(
        template_manager.get_template(DEFAULT_TASKS_PROMPT_TEMPLATE_NAME),
        options,
    )
}

pub fn continuation_prompt(template_manager: &TemplateManager, options: &PromptOptions) -> String {
    apply_shared_prompt_options(
        template_manager.get_template(DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME),
        options,
    )
}

fn apply_shared_prompt_options(template: String, options: &PromptOptions) -> String {
    template.replace("{{language_instruction}}", &options.language_instruction)
}

#[cfg(test)]
mod tests {
    use super::{PromptOptions, generate_prompt, spec_prompt};
    use crate::core::paths::AiPaths;
    use crate::core::test_utils::make_temp_base;
    use crate::templates::manager::TemplateManager;
    use std::fs;

    #[test]
    fn prompt_templates_render_language_instruction() {
        let base = make_temp_base("prompt-language");
        let paths = AiPaths::discover(&base);
        let manager = TemplateManager::new(&paths);
        let prompt = spec_prompt(
            &manager,
            &PromptOptions {
                language_instruction: "Write prose in Turkish.".to_owned(),
            },
        );

        assert!(prompt.contains("Write prose in Turkish."));
        assert!(!prompt.contains("{{language_instruction}}"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn generate_prompt_renders_language_instruction() {
        let base = make_temp_base("generate-prompt-language");
        let paths = AiPaths::discover(&base);
        let manager = TemplateManager::new(&paths);
        let prompt = generate_prompt(
            &manager,
            &PromptOptions {
                language_instruction: "Write prose in Spanish.".to_owned(),
            },
        );

        assert!(prompt.contains("Write prose in Spanish."));
        assert!(!prompt.contains("{{language_instruction}}"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
