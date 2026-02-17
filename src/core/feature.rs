use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub const FEATURE_FILE: &str = "FEATURE.md";
pub const STATE_FILE: &str = "STATE.md";
pub const SESSION_FILE: &str = "SESSION.md";

pub fn ensure_feature_files(feature_dir: &Path, feature_name: &str) -> Result<()> {
    fs::create_dir_all(feature_dir)
        .with_context(|| format!("Failed to create feature directory: {}", feature_dir.display()))?;

    write_if_missing(
        &feature_dir.join(FEATURE_FILE),
        &format!("# Feature: {feature_name}\n\nDescribe scope and goals.\n"),
    )?;
    write_if_missing(
        &feature_dir.join(STATE_FILE),
        "# State\n\nTrack latest progress and blockers.\n",
    )?;
    write_if_missing(
        &feature_dir.join(SESSION_FILE),
        "# Session\n\nCapture continuation-ready context.\n",
    )?;

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
