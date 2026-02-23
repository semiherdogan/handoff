use std::path::{Path, PathBuf};

/// Workspace directory name. Change this value to use a different folder.
pub const WORKSPACE_DIR: &str = ".handoff";

#[derive(Debug, Clone)]
pub struct AiPaths {
    pub ai_dir: PathBuf,
    pub config_toml: PathBuf,
    pub current_link: PathBuf,
    pub features_dir: PathBuf,
}

impl AiPaths {
    pub fn discover(base_dir: &Path) -> Self {
        let ai_dir = base_dir.join(WORKSPACE_DIR);
        let features_dir = ai_dir.join("features");

        Self {
            config_toml: ai_dir.join("config.toml"),
            current_link: ai_dir.join("current"),
            ai_dir,
            features_dir,
        }
    }

    pub fn feature_dir(&self, feature: &str) -> PathBuf {
        self.features_dir.join(feature)
    }
}
