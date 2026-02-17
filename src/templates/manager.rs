use crate::core::paths::AiPaths;
use std::fs;
use std::path::PathBuf;

const DEFAULT_FEATURE_TEMPLATE: &str = include_str!("../../templates/default/default_feature.md");
const DEFAULT_STATE_TEMPLATE: &str = include_str!("../../templates/default/default_state.md");
const DEFAULT_SESSION_TEMPLATE: &str = include_str!("../../templates/default/default_session.md");
const DEFAULT_START_PROMPT_TEMPLATE: &str =
    include_str!("../../templates/default/default_start_prompt.md");
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
            "default_feature.md" => DEFAULT_FEATURE_TEMPLATE.to_owned(),
            "default_state.md" => DEFAULT_STATE_TEMPLATE.to_owned(),
            "default_session.md" => DEFAULT_SESSION_TEMPLATE.to_owned(),
            "default_start_prompt.md" => DEFAULT_START_PROMPT_TEMPLATE.to_owned(),
            "default_continue_prompt.md" => DEFAULT_CONTINUE_PROMPT_TEMPLATE.to_owned(),
            _ => String::new(),
        }
    }
}
