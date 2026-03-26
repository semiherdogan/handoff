use crate::commands::confirm;
use crate::core::feature;
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

    workspace::init_feature_with_switch_option(paths, feature_name, force, set_as_current)?;

    println!("{}", init_summary(paths, feature_name, set_as_current));

    Ok(())
}

fn should_prompt_for_set_current(paths: &AiPaths, feature_name: &str, force: bool) -> bool {
    !force && feature_name != "current" && paths.current_link.symlink_metadata().is_ok()
}

fn init_summary(paths: &AiPaths, feature_name: &str, set_as_current: bool) -> String {
    let feature_dir = paths.feature_dir(feature_name);
    let feature_file = format!(".handoff/current/{}", feature::FEATURE_FILE);
    let spec_file = format!(".handoff/current/{}", feature::SPEC_FILE);
    let design_file = format!(".handoff/current/{}", feature::DESIGN_FILE);
    let state_file = format!(".handoff/current/{}", feature::STATE_FILE);
    let session_file = format!(".handoff/current/{}", feature::SESSION_FILE);

    if set_as_current {
        return format!(
            "Initialized feature: {feature_name}\n\nNext:\n1. Edit: {}\n2. Then run: handoff generate --copy\n3. After planning artifacts are ready, run: handoff start --copy\n\nPlanning files available:\n- {} (AI-managed; usually do not edit unless you want to refine requirements)\n- {} (AI-managed; usually do not edit unless the feature needs explicit design changes)\n- {} (AI-managed during planning and execution)\n- {} (AI-managed during planning and execution)",
            feature_file, spec_file, design_file, state_file, session_file,
        );
    }

    format!(
        "Initialized feature: {feature_name}\n\nThis feature was not set as current.\nEdit: {}\nOr switch first with: handoff switch {feature_name}",
        feature_dir.join(feature::FEATURE_FILE).display()
    )
}

#[cfg(test)]
mod tests {
    use super::{init_summary, should_prompt_for_set_current};
    use crate::core::paths::AiPaths;
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

    #[test]
    fn init_summary_points_user_to_feature_file_first() {
        let base = make_temp_base("init-summary");
        let paths = AiPaths::discover(&base);
        let summary = init_summary(&paths, "my-feature", true);

        assert!(summary.contains("Initialized feature: my-feature"));
        assert!(summary.contains(".handoff/current/FEATURE.md"));
        assert!(summary.contains("Then run: handoff generate --copy"));
        assert!(summary.contains("run: handoff start --copy"));
        assert!(summary.contains("AI-managed"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
