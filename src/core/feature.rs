use crate::templates::manager::{
    TemplateManager, DEFAULT_DESIGN_TEMPLATE_NAME, DEFAULT_FEATURE_TEMPLATE_NAME,
    DEFAULT_SESSION_TEMPLATE_NAME, DEFAULT_SPEC_TEMPLATE_NAME, DEFAULT_STATE_TEMPLATE_NAME,
};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub const FEATURE_FILE: &str = "FEATURE.md";
pub const SPEC_FILE: &str = "SPEC.md";
pub const DESIGN_FILE: &str = "DESIGN.md";
pub const STATE_FILE: &str = "STATE.md";
pub const SESSION_FILE: &str = "SESSION.md";

pub fn ensure_feature_files(
    feature_dir: &Path,
    feature_name: &str,
    template_manager: &TemplateManager,
) -> Result<()> {
    fs::create_dir_all(feature_dir)
        .with_context(|| format!("Failed to create feature directory: {}", feature_dir.display()))?;

    let feature_template = template_manager
        .get_template(DEFAULT_FEATURE_TEMPLATE_NAME)
        .replace("{{feature_name}}", feature_name);
    let spec_template = template_manager.get_template(DEFAULT_SPEC_TEMPLATE_NAME);
    let design_template = template_manager.get_template(DEFAULT_DESIGN_TEMPLATE_NAME);
    let state_template = template_manager.get_template(DEFAULT_STATE_TEMPLATE_NAME);
    let session_template = template_manager.get_template(DEFAULT_SESSION_TEMPLATE_NAME);

    write_if_missing(&feature_dir.join(FEATURE_FILE), &feature_template)?;
    write_if_missing(&feature_dir.join(SPEC_FILE), &spec_template)?;
    write_if_missing(&feature_dir.join(DESIGN_FILE), &design_template)?;
    write_if_missing(&feature_dir.join(STATE_FILE), &state_template)?;
    write_if_missing(&feature_dir.join(SESSION_FILE), &session_template)?;

    Ok(())
}

pub fn validate_feature_files(feature_dir: &Path) -> Result<()> {
    validate_file_exists(&feature_dir.join(FEATURE_FILE), FEATURE_FILE)?;
    validate_file_exists(&feature_dir.join(STATE_FILE), STATE_FILE)?;
    validate_file_exists(&feature_dir.join(SESSION_FILE), SESSION_FILE)?;
    Ok(())
}

pub fn validate_spec_file(feature_dir: &Path) -> Result<()> {
    validate_file(feature_dir, SPEC_FILE)
}

pub fn validate_design_inputs(feature_dir: &Path) -> Result<()> {
    validate_file(feature_dir, FEATURE_FILE)?;
    validate_file(feature_dir, SPEC_FILE)?;
    Ok(())
}

pub fn file_exists(feature_dir: &Path, file_name: &str) -> bool {
    feature_dir.join(file_name).is_file()
}

pub fn validate_file(feature_dir: &Path, file_name: &str) -> Result<()> {
    validate_file_exists(&feature_dir.join(file_name), file_name)
}

fn write_if_missing(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
    }
    Ok(())
}

fn validate_file_exists(path: &Path, display_name: &str) -> Result<()> {
    if !path.is_file() {
        anyhow::bail!("Missing required file: {display_name}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ensure_feature_files, DESIGN_FILE, FEATURE_FILE, SESSION_FILE, SPEC_FILE, STATE_FILE};
    use crate::core::paths::AiPaths;
    use crate::core::test_utils::make_temp_base;
    use crate::templates::manager::TemplateManager;
    use std::fs;

    #[test]
    fn ensure_feature_files_creates_planning_and_execution_artifacts() {
        let base = make_temp_base("feature-files");
        let paths = AiPaths::discover(&base);
        let template_manager = TemplateManager::new(&paths);
        let feature_dir = paths.feature_dir("new-flow");

        ensure_feature_files(&feature_dir, "new-flow", &template_manager)
            .expect("should create feature files");

        for name in [FEATURE_FILE, SPEC_FILE, DESIGN_FILE, STATE_FILE, SESSION_FILE] {
            assert!(feature_dir.join(name).is_file(), "missing expected file: {name}");
        }

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
