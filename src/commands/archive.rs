use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths, feature: &str) -> Result<()> {
    workspace::archive_feature(paths, feature)
}
