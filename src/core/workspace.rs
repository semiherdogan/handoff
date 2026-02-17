use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::templates::manager::TemplateManager;
use anyhow::{anyhow, Context, Result};
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink;

const MAX_FEATURE_NAME_LEN: usize = 64;

pub fn ensure_workspace(paths: &AiPaths) -> Result<()> {
    fs::create_dir_all(&paths.ai_dir)
        .with_context(|| format!("Failed to create directory: {}", paths.ai_dir.display()))?;
    fs::create_dir_all(&paths.features_dir)
        .with_context(|| format!("Failed to create directory: {}", paths.features_dir.display()))?;

    if !paths.config_toml.exists() {
        fs::write(&paths.config_toml, "# ai v0.1 configuration\n")
            .with_context(|| format!("Failed to write file: {}", paths.config_toml.display()))?;
    }

    Ok(())
}

fn validate_feature_name_slug(feature_name: &str) -> Result<()> {
    let bytes = feature_name.as_bytes();

    if bytes.is_empty() || bytes.len() > MAX_FEATURE_NAME_LEN {
        anyhow::bail!(
            "Invalid feature name '{feature_name}'. Use a slug (kebab-case), 1-{MAX_FEATURE_NAME_LEN} chars."
        );
    }

    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        anyhow::bail!(
            "Invalid feature name '{feature_name}'. Slug cannot start or end with '-'"
        );
    }

    let mut prev_dash = false;
    for b in bytes {
        let is_lower = b.is_ascii_lowercase();
        let is_digit = b.is_ascii_digit();
        let is_dash = *b == b'-';

        if !is_lower && !is_digit && !is_dash {
            anyhow::bail!(
                "Invalid feature name '{feature_name}'. Use lowercase letters, digits, and '-' only."
            );
        }

        if is_dash && prev_dash {
            anyhow::bail!(
                "Invalid feature name '{feature_name}'. Use single '-' separators (no consecutive dashes)."
            );
        }

        prev_dash = is_dash;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_temp_base(label: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("handoff-{label}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("failed to create temp test dir");
        dir
    }

    #[test]
    fn validate_feature_name_slug_accepts_valid_slug() {
        assert!(validate_feature_name_slug("feature-123").is_ok());
    }

    #[test]
    fn validate_feature_name_slug_rejects_spaces_and_uppercase() {
        assert!(validate_feature_name_slug("My Feature").is_err());
    }

    #[test]
    fn init_feature_does_not_block_new_feature_when_current_exists() {
        let base = make_temp_base("init-feature-current-exists");
        let paths = AiPaths::discover(&base);

        init_feature_with_switch_option(&paths, "first-feature", false, true)
            .expect("failed to init first feature");
        init_feature_with_switch_option(&paths, "second-feature", false, true)
            .expect("failed to init second feature");

        assert!(paths.feature_dir("second-feature").is_dir());
        let current_target = fs::read_link(&paths.current_link).expect("failed to read current symlink");
        assert_eq!(current_target, std::path::Path::new("features/second-feature"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn clean_features_removes_all_non_current_feature_directories() {
        let base = make_temp_base("clean-features");
        let paths = AiPaths::discover(&base);

        init_feature_with_switch_option(&paths, "active-feature", false, true)
            .expect("failed to init active feature");
        init_feature_with_switch_option(&paths, "stale-feature", false, false)
            .expect("failed to init stale feature");
        init_feature_with_switch_option(&paths, "old-feature", false, false)
            .expect("failed to init old feature");

        let removed = clean_features(&paths).expect("failed to clean features");

        assert_eq!(removed, 2);
        assert!(paths.feature_dir("active-feature").is_dir());
        assert!(!paths.feature_dir("stale-feature").exists());
        assert!(!paths.feature_dir("old-feature").exists());

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}

pub fn init_feature_with_switch_option(
    paths: &AiPaths,
    feature_name: &str,
    force: bool,
    set_as_current: bool,
) -> Result<()> {
    ensure_workspace(paths)?;

    validate_feature_name_slug(feature_name)?;

    // Root cause: this guard blocked all new feature inits once `.ai/current` existed,
    // even when the requested feature directory did not exist yet.
    if feature_name == "current" && paths.current_link.symlink_metadata().is_ok() && !force {
        eprintln!(
            "Warning: {} already exists. Use --force to replace it.",
            paths.current_link.display()
        );
        return Ok(());
    }

    let feature_dir = paths.feature_dir(feature_name);
    let template_manager = TemplateManager::new(paths);
    feature::ensure_feature_files(&feature_dir, feature_name, &template_manager)?;

    if set_as_current {
        set_current_feature(paths, feature_name)?;
    }

    Ok(())
}

pub fn set_current_feature(paths: &AiPaths, feature_name: &str) -> Result<()> {
    let target_feature = paths.feature_dir(feature_name);
    if !target_feature.is_dir() {
        anyhow::bail!("Feature '{feature_name}' not found. Run: ai init {feature_name}");
    }

    if paths.current_link.exists() || paths.current_link.symlink_metadata().is_ok() {
        let metadata = paths
            .current_link
            .symlink_metadata()
            .with_context(|| format!("Failed to inspect: {}", paths.current_link.display()))?;

        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            fs::remove_dir_all(&paths.current_link).with_context(|| {
                format!(
                    "Failed to remove existing directory: {}",
                    paths.current_link.display()
                )
            })?;
        } else {
            fs::remove_file(&paths.current_link).with_context(|| {
                format!("Failed to remove existing link: {}", paths.current_link.display())
            })?;
        }
    }

    let relative_target = std::path::Path::new("features").join(feature_name);
    symlink(&relative_target, &paths.current_link).with_context(|| {
        format!(
            "Failed to create symlink: {} -> {}",
            paths.current_link.display(),
            relative_target.display()
        )
    })?;

    Ok(())
}

pub fn resolve_current_feature_path(paths: &AiPaths) -> Result<std::path::PathBuf> {
    let target = fs::read_link(&paths.current_link)
        .map_err(|_| anyhow!("No active feature found. Run: ai init"))?;

    let absolute = if target.is_absolute() {
        target
    } else {
        paths.ai_dir.join(target)
    };

    if !absolute.is_dir() {
        return Err(anyhow!("No active feature found. Run: ai init"));
    }

    Ok(absolute)
}

pub fn resolve_current_feature_name(paths: &AiPaths) -> Result<String> {
    let current_path = resolve_current_feature_path(paths)?;
    let name = current_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("No active feature found. Run: ai init"))?;
    Ok(name.to_owned())
}

pub fn list_features(paths: &AiPaths) -> Result<Vec<String>> {
    if !paths.features_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(&paths.features_dir)
        .with_context(|| format!("Failed to read directory: {}", paths.features_dir.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_owned());
            }
        }
    }

    names.sort();
    Ok(names)
}

pub fn clean_features(paths: &AiPaths) -> Result<usize> {
    let current = resolve_current_feature_name(paths)?;
    let features = list_features(paths)?;

    let mut removed = 0usize;
    for feature in features {
        if feature == current {
            continue;
        }

        let feature_dir = paths.feature_dir(&feature);
        fs::remove_dir_all(&feature_dir)
            .with_context(|| format!("Failed to remove feature directory: {}", feature_dir.display()))?;
        removed += 1;
    }

    Ok(removed)
}

pub fn archive_feature(paths: &AiPaths, feature_name: &str) -> Result<()> {
    let source = paths.feature_dir(feature_name);
    if !source.is_dir() {
        anyhow::bail!("Feature '{feature_name}' not found.");
    }

    let is_archiving_current = match fs::read_link(&paths.current_link) {
        Ok(target) => {
            let absolute = if target.is_absolute() {
                target
            } else {
                paths.ai_dir.join(target)
            };
            absolute == source
        }
        Err(_) => false,
    };

    let archived_name = format!("{feature_name}.archived");
    let destination = paths.feature_dir(&archived_name);
    fs::rename(&source, &destination).with_context(|| {
        format!(
            "Failed to archive feature: {} -> {}",
            source.display(),
            destination.display()
        )
    })?;

    if is_archiving_current {
        // Root cause: checking current after rename can fail because the symlink target is already moved.
        fs::remove_file(&paths.current_link).with_context(|| {
            format!(
                "Archived active feature but failed to clear symlink: {}",
                paths.current_link.display()
            )
        })?;
    }

    Ok(())
}
