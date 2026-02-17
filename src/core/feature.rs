use crate::templates::prompts;
use crate::templates::manager::TemplateManager;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub const FEATURE_FILE: &str = "FEATURE.md";
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
        .get_template(prompts::DEFAULT_FEATURE_TEMPLATE_NAME)
        .replace("{{feature_name}}", feature_name);
    let state_template = template_manager.get_template(prompts::DEFAULT_STATE_TEMPLATE_NAME);
    let session_template = template_manager.get_template(prompts::DEFAULT_SESSION_TEMPLATE_NAME);

    write_if_missing(
        &feature_dir.join(FEATURE_FILE),
        &feature_template,
    )?;
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
