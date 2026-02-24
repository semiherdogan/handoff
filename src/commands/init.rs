use crate::commands::confirm;
use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths, feature: Option<&str>, force: bool) -> Result<()> {
    let feature_name = feature.unwrap_or("current");

    // Root cause: init always replaced `.handoff/current` for new features without giving users
    // an explicit opt-in when another active feature already existed.
    let set_as_current = if should_prompt_for_set_current(paths, feature_name, force) {
        confirm::prompt_yes_no(&format!(
            "Warning: .handoff/current already exists. Set '{feature_name}' as current feature? [y/N]: ",
        ))?
    } else {
        true
    };

    workspace::init_feature_with_switch_option(paths, feature_name, force, set_as_current)
}

fn should_prompt_for_set_current(paths: &AiPaths, feature_name: &str, force: bool) -> bool {
    !force && feature_name != "current" && paths.current_link.symlink_metadata().is_ok()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::make_temp_base;
    use std::fs;
    use std::os::unix::fs::symlink;

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
