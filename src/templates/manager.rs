use crate::core::paths::AiPaths;
use std::fs;
use std::path::PathBuf;

pub const DEFAULT_FEATURE_TEMPLATE_NAME: &str = "default_feature.md";
pub const DEFAULT_SPEC_TEMPLATE_NAME: &str = "default_spec.md";
pub const DEFAULT_DESIGN_TEMPLATE_NAME: &str = "default_design.md";
pub const DEFAULT_STATE_TEMPLATE_NAME: &str = "default_state.md";
pub const DEFAULT_SESSION_TEMPLATE_NAME: &str = "default_session.md";
pub const DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME: &str = "default_generate_prompt.md";
pub const DEFAULT_START_PROMPT_TEMPLATE_NAME: &str = "default_start_prompt.md";
pub const DEFAULT_SPEC_PROMPT_TEMPLATE_NAME: &str = "default_spec_prompt.md";
pub const DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME: &str = "default_design_prompt.md";
pub const DEFAULT_TASKS_PROMPT_TEMPLATE_NAME: &str = "default_tasks_prompt.md";
pub const DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME: &str = "default_continue_prompt.md";

const DEFAULT_FEATURE_TEMPLATE: &str = include_str!("../../templates/default/default_feature.md");
const DEFAULT_SPEC_TEMPLATE: &str = include_str!("../../templates/default/default_spec.md");
const DEFAULT_DESIGN_TEMPLATE: &str = include_str!("../../templates/default/default_design.md");
const DEFAULT_STATE_TEMPLATE: &str = include_str!("../../templates/default/default_state.md");
const DEFAULT_SESSION_TEMPLATE: &str = include_str!("../../templates/default/default_session.md");
const DEFAULT_GENERATE_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_generate_prompt.md");
const DEFAULT_START_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_start_prompt.md");
const DEFAULT_SPEC_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_spec_prompt.md");
const DEFAULT_DESIGN_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_design_prompt.md");
const DEFAULT_TASKS_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_tasks_prompt.md");
const DEFAULT_CONTINUE_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_continue_prompt.md");

pub struct TemplateManager {
    override_dir: PathBuf,
}

impl TemplateManager {
    pub fn new(paths: &AiPaths) -> Self {
        Self {
            override_dir: paths.ai_dir.join("templates"),
        }
    }

    pub fn override_dir(&self) -> &PathBuf {
        &self.override_dir
    }

    pub fn default_templates() -> Vec<(&'static str, &'static str)> {
        vec![
            (DEFAULT_FEATURE_TEMPLATE_NAME, DEFAULT_FEATURE_TEMPLATE),
            (DEFAULT_SPEC_TEMPLATE_NAME, DEFAULT_SPEC_TEMPLATE),
            (DEFAULT_DESIGN_TEMPLATE_NAME, DEFAULT_DESIGN_TEMPLATE),
            (DEFAULT_STATE_TEMPLATE_NAME, DEFAULT_STATE_TEMPLATE),
            (DEFAULT_SESSION_TEMPLATE_NAME, DEFAULT_SESSION_TEMPLATE),
            (
                DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME,
                DEFAULT_GENERATE_PROMPT_TEMPLATE,
            ),
            (
                DEFAULT_START_PROMPT_TEMPLATE_NAME,
                DEFAULT_START_PROMPT_TEMPLATE,
            ),
            (
                DEFAULT_SPEC_PROMPT_TEMPLATE_NAME,
                DEFAULT_SPEC_PROMPT_TEMPLATE,
            ),
            (
                DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME,
                DEFAULT_DESIGN_PROMPT_TEMPLATE,
            ),
            (
                DEFAULT_TASKS_PROMPT_TEMPLATE_NAME,
                DEFAULT_TASKS_PROMPT_TEMPLATE,
            ),
            (
                DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME,
                DEFAULT_CONTINUE_PROMPT_TEMPLATE,
            ),
        ]
    }

    pub fn get_template(&self, name: &str) -> String {
        let override_path = self.override_dir.join(name);
        if override_path.is_file() {
            match fs::read_to_string(&override_path) {
                Ok(content) => return content,
                Err(error) => {
                    eprintln!(
                        "Warning: failed to read override template {}: {error}",
                        override_path.display()
                    );
                }
            }
        }

        match name {
            DEFAULT_FEATURE_TEMPLATE_NAME => DEFAULT_FEATURE_TEMPLATE.to_owned(),
            DEFAULT_SPEC_TEMPLATE_NAME => DEFAULT_SPEC_TEMPLATE.to_owned(),
            DEFAULT_DESIGN_TEMPLATE_NAME => DEFAULT_DESIGN_TEMPLATE.to_owned(),
            DEFAULT_STATE_TEMPLATE_NAME => DEFAULT_STATE_TEMPLATE.to_owned(),
            DEFAULT_SESSION_TEMPLATE_NAME => DEFAULT_SESSION_TEMPLATE.to_owned(),
            DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME => DEFAULT_GENERATE_PROMPT_TEMPLATE.to_owned(),
            DEFAULT_START_PROMPT_TEMPLATE_NAME => DEFAULT_START_PROMPT_TEMPLATE.to_owned(),
            DEFAULT_SPEC_PROMPT_TEMPLATE_NAME => DEFAULT_SPEC_PROMPT_TEMPLATE.to_owned(),
            DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME => DEFAULT_DESIGN_PROMPT_TEMPLATE.to_owned(),
            DEFAULT_TASKS_PROMPT_TEMPLATE_NAME => DEFAULT_TASKS_PROMPT_TEMPLATE.to_owned(),
            DEFAULT_CONTINUE_PROMPT_TEMPLATE_NAME => DEFAULT_CONTINUE_PROMPT_TEMPLATE.to_owned(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME, DEFAULT_DESIGN_TEMPLATE_NAME,
        DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME, DEFAULT_SPEC_PROMPT_TEMPLATE_NAME,
        DEFAULT_SPEC_TEMPLATE_NAME, DEFAULT_TASKS_PROMPT_TEMPLATE_NAME, TemplateManager,
    };

    #[test]
    fn default_templates_include_planning_artifacts_and_prompts() {
        let templates = TemplateManager::default_templates();
        let names = templates.iter().map(|(name, _)| *name).collect::<Vec<_>>();

        assert!(names.contains(&DEFAULT_SPEC_TEMPLATE_NAME));
        assert!(names.contains(&DEFAULT_DESIGN_TEMPLATE_NAME));
        assert!(names.contains(&DEFAULT_GENERATE_PROMPT_TEMPLATE_NAME));
        assert!(names.contains(&DEFAULT_SPEC_PROMPT_TEMPLATE_NAME));
        assert!(names.contains(&DEFAULT_DESIGN_PROMPT_TEMPLATE_NAME));
        assert!(names.contains(&DEFAULT_TASKS_PROMPT_TEMPLATE_NAME));
    }
}
