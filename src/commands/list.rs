use crate::core::command_name;
use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths) -> Result<()> {
    let features = workspace::list_features(paths)?;
    let current = workspace::resolve_current_feature_name(paths).ok();

    if features.is_empty() {
        println!("No features found. Run: {} init", command_name::current());
        return Ok(());
    }

    for feature in features {
        if current.as_deref() == Some(feature.as_str()) {
            println!("* {feature}");
        } else {
            println!("  {feature}");
        }
    }

    Ok(())
}
