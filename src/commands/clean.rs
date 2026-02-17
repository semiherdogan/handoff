use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths, force: bool) -> Result<()> {
    let removed = workspace::clean_features(paths, force)?;
    println!("Removed {removed} feature(s).");
    Ok(())
}
