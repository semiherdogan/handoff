use crate::core::paths::WORKSPACE_DIR;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn run() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.is_dir() {
        anyhow::bail!("Not a git repository (no .git directory found).");
    }

    let info_dir = git_dir.join("info");
    let exclude_path = info_dir.join("exclude");

    fs::create_dir_all(&info_dir)
        .with_context(|| format!("Failed to create directory: {}", info_dir.display()))?;

    let content = if exclude_path.exists() {
        fs::read_to_string(&exclude_path)
            .with_context(|| format!("Failed to read: {}", exclude_path.display()))?
    } else {
        String::new()
    };

    let entry = format!("{WORKSPACE_DIR}/");

    if has_exclude_entry(&content, &entry) {
        let new_content = remove_exclude_entry(&content, &entry);
        fs::write(&exclude_path, new_content)
            .with_context(|| format!("Failed to write: {}", exclude_path.display()))?;
        println!("Removed '{}' from {}", entry, exclude_path.display());
    } else {
        let new_content = add_exclude_entry(&content, &entry);
        fs::write(&exclude_path, new_content)
            .with_context(|| format!("Failed to write: {}", exclude_path.display()))?;
        println!("Added '{}' to {}", entry, exclude_path.display());
    }

    Ok(())
}

fn has_exclude_entry(content: &str, entry: &str) -> bool {
    content.lines().any(|line| line.trim() == entry)
}

fn remove_exclude_entry(content: &str, entry: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    lines.retain(|line| line.trim() != entry);
    let mut result = lines.join("\n");
    if content.ends_with('\n') && !result.is_empty() {
        result.push('\n');
    }
    result
}

fn add_exclude_entry(content: &str, entry: &str) -> String {
    let mut result = content.to_string();
    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(entry);
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTRY: &str = ".handoff/";

    #[test]
    fn has_exclude_entry_detects_entry() {
        assert!(has_exclude_entry(".handoff/\n", ENTRY));
        assert!(has_exclude_entry("foo\n.handoff/\nbar\n", ENTRY));
        assert!(!has_exclude_entry("", ENTRY));
        assert!(!has_exclude_entry("something\n", ENTRY));
    }

    #[test]
    fn add_exclude_entry_to_empty() {
        let result = add_exclude_entry("", ENTRY);
        assert_eq!(result, ".handoff/\n");
    }

    #[test]
    fn add_exclude_entry_to_existing_content() {
        let result = add_exclude_entry("foo\nbar\n", ENTRY);
        assert_eq!(result, "foo\nbar\n.handoff/\n");
    }

    #[test]
    fn add_exclude_entry_to_content_without_trailing_newline() {
        let result = add_exclude_entry("foo\nbar", ENTRY);
        assert_eq!(result, "foo\nbar\n.handoff/\n");
    }

    #[test]
    fn remove_exclude_entry_removes_line() {
        let result = remove_exclude_entry("foo\n.handoff/\nbar\n", ENTRY);
        assert_eq!(result, "foo\nbar\n");
    }

    #[test]
    fn remove_exclude_entry_only_entry() {
        let result = remove_exclude_entry(".handoff/\n", ENTRY);
        assert_eq!(result, "");
    }

    #[test]
    fn remove_exclude_entry_preserves_other_lines() {
        let result = remove_exclude_entry("alpha\n.handoff/\nbeta\n", ENTRY);
        assert_eq!(result, "alpha\nbeta\n");
    }
}
