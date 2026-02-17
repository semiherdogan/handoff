use crate::core::feature;
use crate::core::paths::AiPaths;
use crate::core::state;
use crate::core::workspace;
use anyhow::{Context, Result};
use std::fs;

pub fn run(paths: &AiPaths) -> Result<()> {
    let active_feature_path = workspace::resolve_current_feature_path(paths)?;
    feature::validate_feature_files(&active_feature_path)?;

    let feature_name = workspace::resolve_current_feature_name(paths)?;
    let state_path = active_feature_path.join(feature::STATE_FILE);
    let state_content = fs::read_to_string(&state_path)
        .with_context(|| format!("Failed to read file: {}", state_path.display()))?;

    let summary = state::parse_state(&state_content);

    println!("Active feature: {feature_name}");
    println!("Current Step: {}", summary.current_step);
    println!("Remaining steps: {}", summary.remaining_steps);
    println!("Completed steps: {}", summary.completed_steps);

    if summary.known_risks.is_empty() {
        println!("Known risks: None");
    } else {
        println!("Known risks:");
        for risk in summary.known_risks {
            println!("- {risk}");
        }
    }

    Ok(())
}
