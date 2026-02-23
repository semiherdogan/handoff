use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::{Context, Result};
use std::io::{self, Write};

pub fn run(paths: &AiPaths, feature: Option<&str>, force: bool) -> Result<()> {
    let feature_name = feature.unwrap_or("current");

    // Root cause: init always replaced `.handoff/current` for new features without giving users
    // an explicit opt-in when another active feature already existed.
    let set_as_current = if should_prompt_for_set_current(paths, feature_name, force) {
        confirm_set_as_current(feature_name)?
    } else {
        true
    };

    workspace::init_feature_with_switch_option(paths, feature_name, force, set_as_current)
}

fn should_prompt_for_set_current(paths: &AiPaths, feature_name: &str, force: bool) -> bool {
    !force && feature_name != "current" && paths.current_link.symlink_metadata().is_ok()
}

fn confirm_set_as_current(feature_name: &str) -> Result<bool> {
    print!("Warning: .handoff/current already exists. Set '{feature_name}' as current feature? [y/N]: ");
    io::stdout()
        .flush()
        .context("Failed to flush confirmation prompt")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read confirmation response")?;

    Ok(parse_confirmation_response(&input))
}

fn parse_confirmation_response(input: &str) -> bool {
    matches!(input.trim(), "y" | "Y" | "yes" | "YES" | "Yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
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
    fn parse_confirmation_response_accepts_yes_variants() {
        assert!(parse_confirmation_response("y"));
        assert!(parse_confirmation_response("Y"));
        assert!(parse_confirmation_response("yes"));
        assert!(parse_confirmation_response("Yes\n"));
    }

    #[test]
    fn parse_confirmation_response_defaults_to_no() {
        assert!(!parse_confirmation_response(""));
        assert!(!parse_confirmation_response("n"));
        assert!(!parse_confirmation_response("no"));
        assert!(!parse_confirmation_response("anything-else"));
    }

    #[test]
    fn should_prompt_for_set_current_only_when_current_exists_without_force() {
        let base = make_temp_base("init-prompt-set-current");
        let paths = AiPaths::discover(&base);

        fs::create_dir_all(&paths.ai_dir).expect("failed to create .handoff dir");
        symlink("features/first", &paths.current_link).expect("failed to create current symlink");

        assert!(should_prompt_for_set_current(&paths, "new-feature", false));
        assert!(!should_prompt_for_set_current(&paths, "new-feature", true));
        assert!(!should_prompt_for_set_current(&paths, "current", false));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
