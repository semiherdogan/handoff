use crate::templates::manager::{
    DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME, DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME,
    DEFAULT_SPEC_PROMPT_TEMPLATE_NAME, DEFAULT_START_PROMPT_TEMPLATE_NAME,
    DEFAULT_TASKS_PROMPT_TEMPLATE_NAME, TemplateManager,
};

pub struct StartPromptContext {
    pub read_files: String,
    pub artifact_status: String,
    pub planning_mode: String,
    pub workflow_instructions: String,
}

pub fn start_prompt(template_manager: &TemplateManager, context: &StartPromptContext) -> String {
    template_manager
        .get_template(DEFAULT_START_PROMPT_TEMPLATE_NAME)
        .replace("{{read_files}}", &context.read_files)
        .replace("{{artifact_status}}", &context.artifact_status)
        .replace("{{planning_mode}}", &context.planning_mode)
        .replace("{{workflow_instructions}}", &context.workflow_instructions)
}

pub fn spec_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_SPEC_PROMPT_TEMPLATE_NAME)
}

pub fn design_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME)
}

pub fn tasks_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_TASKS_PROMPT_TEMPLATE_NAME)
}

pub fn continuation_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME)
}
