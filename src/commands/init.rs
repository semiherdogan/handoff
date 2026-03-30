use crate::commands::confirm;
use crate::core::context;
use crate::core::feature;
use crate::core::feature::FeatureTemplateSeed;
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

    let context_scan = context::scan(paths);
    let template_seed = FeatureTemplateSeed {
        context_sources: context_scan.found_sources_bullets(),
        context_gaps: context_gaps_for_feature(&context_scan),
    };

    workspace::init_feature_with_switch_option(
        paths,
        feature_name,
        force,
        set_as_current,
        &template_seed,
    )?;

    println!(
        "{}",
        init_summary(paths, feature_name, set_as_current, &context_scan)
    );

    Ok(())
}

fn should_prompt_for_set_current(paths: &AiPaths, feature_name: &str, force: bool) -> bool {
    !force && feature_name != "current" && paths.current_link.symlink_metadata().is_ok()
}

fn init_summary(
    paths: &AiPaths,
    feature_name: &str,
    set_as_current: bool,
    context_scan: &context::ContextScan,
) -> String {
    let feature_dir = paths.feature_dir(feature_name);
    let feature_file = format!(".handoff/current/{}", feature::FEATURE_FILE);
    let spec_file = format!(".handoff/current/{}", feature::SPEC_FILE);
    let design_file = format!(".handoff/current/{}", feature::DESIGN_FILE);
    let state_file = format!(".handoff/current/{}", feature::STATE_FILE);
    let session_file = format!(".handoff/current/{}", feature::SESSION_FILE);
    let context_summary = format!(
        "Context readiness:\n- Found sources:\n{}\n- High-value gaps:\n{}\n- Optional gaps:\n{}\n- Improve context with: handoff prompt context --copy",
        indent_block(&context_scan.found_sources_bullets(), 2),
        indent_block(&context_scan.high_value_missing_bullets(), 2),
        indent_block(&context_scan.optional_missing_bullets(), 2),
    );

    if set_as_current {
        return format!(
            "Initialized feature: {feature_name}\n\n{context_summary}\n\nNext:\n1. Edit: {}\n2. If the repo needs better onboarding/context docs, run: handoff prompt context --copy\n3. Then run: handoff run --copy\n4. Use handoff next to inspect the current task without generating a prompt\n\nPlanning files available:\n- {} (AI-managed; usually do not edit unless you want to refine requirements)\n- {} (AI-managed; usually do not edit unless the feature needs explicit design changes)\n- {} (AI-managed during planning and execution)\n- {} (AI-managed during planning and execution)",
            feature_file, spec_file, design_file, state_file, session_file,
        );
    }

    format!(
        "Initialized feature: {feature_name}\n\n{context_summary}\n\nThis feature was not set as current.\nEdit: {}\nOr switch first with: handoff switch {feature_name}",
        feature_dir.join(feature::FEATURE_FILE).display()
    )
}

fn indent_block(block: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    block
        .lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn context_gaps_for_feature(context_scan: &context::ContextScan) -> String {
    let mut lines = Vec::new();

    lines.extend(
        context_scan
            .high_value_missing
            .iter()
            .map(|gap| format!("- {}: {}", gap.name, gap.message)),
    );
    lines.extend(
        context_scan
            .optional_missing
            .iter()
            .map(|gap| format!("- {}: {}", gap.name, gap.message)),
    );

    if lines.is_empty() {
        "- None.".to_owned()
    } else {
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        context_gaps_for_feature, indent_block, init_summary, should_prompt_for_set_current,
    };
    use crate::core::context::{ContextGap, ContextScan};
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
        let summary = init_summary(
            &paths,
            "my-feature",
            true,
            &ContextScan {
                found_sources: vec!["README.md".to_owned()],
                high_value_missing: Vec::new(),
                optional_missing: Vec::new(),
            },
        );

        assert!(summary.contains("Initialized feature: my-feature"));
        assert!(summary.contains("Context readiness"));
        assert!(summary.contains(".handoff/current/FEATURE.md"));
        assert!(summary.contains("handoff prompt context --copy"));
        assert!(summary.contains("Then run: handoff run --copy"));
        assert!(summary.contains("Use handoff next"));
        assert!(summary.contains("AI-managed"));

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn indent_block_prefixes_each_line() {
        assert_eq!(indent_block("- one\n- two", 2), "  - one\n  - two");
    }

    #[test]
    fn context_gaps_for_feature_combines_high_value_and_optional_gaps() {
        let rendered = context_gaps_for_feature(&ContextScan {
            found_sources: Vec::new(),
            high_value_missing: vec![ContextGap {
                name: "README.md",
                message: "Create one.",
            }],
            optional_missing: vec![ContextGap {
                name: "docs/",
                message: "Optional.",
            }],
        });

        assert_eq!(rendered, "- README.md: Create one.\n- docs/: Optional.");
    }
}
