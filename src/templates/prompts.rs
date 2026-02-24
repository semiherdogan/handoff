use crate::templates::manager::{
    TemplateManager, DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME, DEFAULT_START_PROMPT_TEMPLATE_NAME,
};

pub use crate::templates::manager::{
    DEFAULT_FEATURE_TEMPLATE_NAME, DEFAULT_SESSION_TEMPLATE_NAME, DEFAULT_STATE_TEMPLATE_NAME,
};

pub fn start_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_START_PROMPT_TEMPLATE_NAME)
}

pub fn continuation_prompt(template_manager: &TemplateManager) -> String {
    template_manager.get_template(DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME)
}
