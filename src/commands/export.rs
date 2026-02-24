use crate::core::paths::AiPaths;
use crate::templates::manager::TemplateManager;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::{self, Write};

pub fn run(paths: &AiPaths, force: bool) -> Result<()> {
    let template_manager = TemplateManager::new(paths);
    let override_dir = template_manager.override_dir();

    let dir_exists = override_dir.is_dir();

    if dir_exists && !force {
        let has_files = fs::read_dir(override_dir)
            .context("Failed to read templates directory")?
            .next()
            .is_some();

        if has_files {
            print!(
                "Warning: {} already contains files. Overwrite? [y/N]: ",
                override_dir.display()
            );
            io::stdout()
                .flush()
                .context("Failed to flush confirmation prompt")?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .context("Failed to read confirmation response")?;

            if !matches!(input.trim(), "y" | "Y" | "yes" | "YES" | "Yes") {
                println!("Export cancelled.");
                return Ok(());
            }
        }
    }

    if !dir_exists {
        fs::create_dir_all(override_dir).with_context(|| {
            format!(
                "Failed to create templates directory: {}",
                override_dir.display()
            )
        })?;
    }

    let templates = TemplateManager::default_templates();
    for (name, content) in &templates {
        let dest = override_dir.join(name);
        fs::write(&dest, content)
            .with_context(|| format!("Failed to write template: {}", dest.display()))?;
    }

    println!(
        "{} Exported {} templates to {}",
        "✓".green(),
        templates.len(),
        override_dir.display()
    );

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
        let dir =
            std::env::temp_dir().join(format!("handoff-{label}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).expect("failed to create temp test dir");
        dir
    }

    #[test]
    fn export_creates_templates_directory_and_files() {
        let base = make_temp_base("export-creates");
        let paths = AiPaths::discover(&base);

        run(&paths, false).expect("export should succeed");

        let template_dir = paths.ai_dir.join("templates");
        assert!(template_dir.is_dir());

        let expected = TemplateManager::default_templates();
        for (name, content) in &expected {
            let file = template_dir.join(name);
            assert!(file.is_file(), "template file should exist: {name}");
            let written = fs::read_to_string(&file).expect("should read template file");
            assert_eq!(&written.as_str(), content);
        }

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }

    #[test]
    fn export_with_force_overwrites_existing() {
        let base = make_temp_base("export-force");
        let paths = AiPaths::discover(&base);

        let template_dir = paths.ai_dir.join("templates");
        fs::create_dir_all(&template_dir).expect("create template dir");
        fs::write(template_dir.join("default_feature.md"), "custom content")
            .expect("write custom");

        run(&paths, true).expect("export --force should succeed");

        let written = fs::read_to_string(template_dir.join("default_feature.md"))
            .expect("should read template");
        assert_ne!(written, "custom content", "should have been overwritten");

        fs::remove_dir_all(base).expect("failed to cleanup temp test dir");
    }
}
