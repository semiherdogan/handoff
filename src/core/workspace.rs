use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::templates::manager::TemplateManager;
use anyhow::{anyhow, Context, Result};
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink;

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

pub fn init_feature(paths: &AiPaths, feature_name: &str, force: bool) -> Result<()> {
    ensure_workspace(paths)?;

    if paths.current_link.symlink_metadata().is_ok() && !force {
        eprintln!(
            "Warning: {} already exists. Use --force to replace it.",
            paths.current_link.display()
        );
        return Ok(());
    }

    let feature_dir = paths.feature_dir(feature_name);
    let template_manager = TemplateManager::new(paths);
    feature::ensure_feature_files(&feature_dir, feature_name, &template_manager)?;

    if paths.current_link.symlink_metadata().is_ok() {
        eprintln!(
            "Warning: {} already exists and will be replaced.",
            paths.current_link.display()
        );
    }

    set_current_feature(paths, feature_name)?;
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
