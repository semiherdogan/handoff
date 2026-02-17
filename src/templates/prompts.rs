use crate::templates::manager::TemplateManager;

pub const DEFAULT_FEATURE_TEMPLATE_NAME: &str = "default_feature.md";
pub const DEFAULT_STATE_TEMPLATE_NAME: &str = "default_state.md";
pub const DEFAULT_SESSION_TEMPLATE_NAME: &str = "default_session.md";
pub const DEFAULT_START_PROMPT_TEMPLATE_NAME: &str = "default_start_prompt.md";
pub const DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME: &str = "default_continue_prompt.md";

pub fn start_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_START_PROMPT_TEMPLATE_NAME)
}

pub fn continuation_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME)
}
