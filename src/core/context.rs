use crate::core::paths::AiPaths;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextGap {
    pub name: &'static str,
    pub message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextScan {
    pub found_sources: Vec<String>,
    pub high_value_missing: Vec<ContextGap>,
    pub optional_missing: Vec<ContextGap>,
}

impl ContextScan {
    pub fn found_sources_bullets(&self) -> String {
        render_bullets(
            &self.found_sources,
            "- No obvious repository context sources detected yet.",
        )
    }

    pub fn high_value_missing_bullets(&self) -> String {
        if self.high_value_missing.is_empty() {
            return "- None.".to_owned();
        }

        self.high_value_missing
            .iter()
            .map(|gap| format!("- {}: {}", gap.name, gap.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn optional_missing_bullets(&self) -> String {
        if self.optional_missing.is_empty() {
            return "- None.".to_owned();
        }

        self.optional_missing
            .iter()
            .map(|gap| format!("- {}: {}", gap.name, gap.message))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn context_prompt_missing_bullets(&self) -> String {
        let mut lines = self
            .high_value_missing
            .iter()
            .map(|gap| format!("- High value: {} ({})", gap.name, gap.message))
            .collect::<Vec<_>>();

        lines.extend(
            self.optional_missing
                .iter()
                .map(|gap| format!("- Optional: {} ({})", gap.name, gap.message)),
        );

        render_bullets(&lines, "- No obvious context gaps detected.")
    }
}

pub fn scan(paths: &AiPaths) -> ContextScan {
    let root = repo_root(paths);
    let mut found_sources = Vec::new();

    for entry in [
        "README.md",
        "AGENTS.md",
        "CLAUDE.md",
        "docs/",
        "Cargo.toml",
        "package.json",
        "pyproject.toml",
        "go.mod",
        "Makefile",
        ".github/workflows/",
        "src/",
        "tests/",
    ] {
        if source_exists(root, entry) {
            found_sources.push(entry.to_owned());
        }
    }

    let mut high_value_missing = Vec::new();
    if !root.join("README.md").is_file() {
        high_value_missing.push(ContextGap {
            name: "README.md",
            message: "Creating one will improve AI onboarding and project understanding.",
        });
    }
    if !root.join("AGENTS.md").is_file() {
        high_value_missing.push(ContextGap {
            name: "AGENTS.md",
            message: "Creating one will improve execution constraints, repository rules, and continuity.",
        });
    }

    let mut optional_missing = Vec::new();
    if !root.join("docs").is_dir() {
        optional_missing.push(ContextGap {
            name: "docs/",
            message: "Optional. Only useful if README.md and AGENTS.md are not enough for architecture or workflow context.",
        });
    }

    ContextScan {
        found_sources,
        high_value_missing,
        optional_missing,
    }
}

fn repo_root(paths: &AiPaths) -> &Path {
    paths.ai_dir.parent().unwrap_or(paths.ai_dir.as_path())
}

fn source_exists(root: &Path, entry: &str) -> bool {
    if let Some(dir) = entry.strip_suffix('/') {
        root.join(dir).is_dir()
    } else {
        root.join(entry).is_file()
    }
}

fn render_bullets(lines: &[String], empty: &str) -> String {
    if lines.is_empty() {
        return empty.to_owned();
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::scan;
    use crate::core::paths::AiPaths;
    use crate::core::test_utils::make_temp_base;
    use std::fs;

    #[test]
    fn scan_reports_missing_readme_and_agents_as_high_value() {
        let base = make_temp_base("context-scan-missing");
        let paths = AiPaths::discover(&base);
        fs::create_dir_all(&paths.ai_dir).expect("should create .handoff");

        let scan = scan(&paths);

        assert!(
            scan.high_value_missing
                .iter()
                .any(|gap| gap.name == "README.md")
        );
        assert!(
            scan.high_value_missing
                .iter()
                .any(|gap| gap.name == "AGENTS.md")
        );

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn scan_detects_existing_context_sources() {
        let base = make_temp_base("context-scan-found");
        let paths = AiPaths::discover(&base);
        fs::create_dir_all(base.join("src")).expect("should create src");
        fs::create_dir_all(base.join("docs")).expect("should create docs");
        fs::create_dir_all(&paths.ai_dir).expect("should create .handoff");
        fs::write(base.join("README.md"), "# Demo\n").expect("should write readme");
        fs::write(base.join("AGENTS.md"), "# Rules\n").expect("should write agents");
        fs::write(base.join("CLAUDE.md"), "# Claude\n").expect("should write claude");
        fs::write(base.join("Cargo.toml"), "[package]\nname=\"demo\"\n").expect("write cargo");

        let scan = scan(&paths);

        assert!(scan.found_sources.contains(&"README.md".to_owned()));
        assert!(scan.found_sources.contains(&"AGENTS.md".to_owned()));
        assert!(scan.found_sources.contains(&"CLAUDE.md".to_owned()));
        assert!(scan.found_sources.contains(&"docs/".to_owned()));
        assert!(scan.found_sources.contains(&"src/".to_owned()));
        assert!(scan.high_value_missing.is_empty());

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
