use crate::core::paths::AiPaths;
use anyhow::{Context, Result};
use std::fs;

pub const DEFAULT_LANGUAGE: &str = "English";
pub const DEFAULT_CONFIG_CONTENT: &str = "# handoff configuration\nlanguage = \"English\"\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub language: String,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            language: DEFAULT_LANGUAGE.to_owned(),
        }
    }
}

impl WorkspaceConfig {
    pub fn language_instruction(&self) -> String {
        format!(
            "- Write all newly created or rewritten prose in {}.\n- Keep file names, paths, markdown structure, and code/config syntax unchanged unless the task requires a real structural change.\n- In `.handoff/current/STATE.md`, keep these section headers in English: `# Current Step`, `# Execution Plan`, `# Completed Steps`, `# Remaining Steps`, `# Known Issues`, `# Risks`, `# Architectural Notes`, and `# Drift Warnings`.\n- Do not translate or alter execution markers `[ ]`, `[>]`, or `[x]`.",
            self.language
        )
    }
}

pub fn load(paths: &AiPaths) -> Result<WorkspaceConfig> {
    if !paths.config_toml.is_file() {
        return Ok(WorkspaceConfig::default());
    }

    let content = fs::read_to_string(&paths.config_toml)
        .with_context(|| format!("Failed to read file: {}", paths.config_toml.display()))?;

    Ok(WorkspaceConfig {
        language: parse_language(&content).unwrap_or_else(|| DEFAULT_LANGUAGE.to_owned()),
    })
}

fn parse_language(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };

        if key.trim() != "language" {
            continue;
        }

        let raw_value = value.split('#').next().unwrap_or("").trim();
        let normalized = normalize_language_value(raw_value);

        if normalized.is_empty() {
            return None;
        }

        return Some(normalized.to_owned());
    }

    None
}

fn normalize_language_value(value: &str) -> &str {
    let stripped = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|rest| rest.strip_suffix('\''))
        })
        .unwrap_or(value);

    stripped.trim()
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_LANGUAGE, WorkspaceConfig, parse_language};

    #[test]
    fn missing_language_uses_default() {
        assert_eq!(parse_language("# comment only\n"), None);
    }

    #[test]
    fn quoted_language_is_parsed() {
        assert_eq!(
            parse_language("language = \"Turkish\"\n"),
            Some("Turkish".to_owned())
        );
    }

    #[test]
    fn bare_language_is_parsed_leniently() {
        assert_eq!(
            parse_language("language = Turkish\n"),
            Some("Turkish".to_owned())
        );
    }

    #[test]
    fn empty_language_falls_back_to_default() {
        let config = WorkspaceConfig {
            language: parse_language("language = \"\"\n")
                .unwrap_or_else(|| DEFAULT_LANGUAGE.to_owned()),
        };

        assert_eq!(config.language, DEFAULT_LANGUAGE);
    }

    #[test]
    fn language_instruction_mentions_state_contract() {
        let instruction = WorkspaceConfig {
            language: "Turkish".to_owned(),
        }
        .language_instruction();

        assert!(instruction.contains("Turkish"));
        assert!(instruction.contains("keep these section headers in English"));
        assert!(instruction.contains("[>]"));
    }
}
