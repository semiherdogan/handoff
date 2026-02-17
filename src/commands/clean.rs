use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths) -> Result<()> {
    let removed = workspace::clean_features(paths)?;
    println!("Removed {removed} feature(s).");
    Ok(())
}
