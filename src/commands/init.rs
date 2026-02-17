use crate::core::paths::AiPaths;
use crate::core::workspace;
use anyhow::Result;

pub fn run(paths: &AiPaths, feature: Option<&str>, force: bool) -> Result<()> {
    let feature_name = feature.unwrap_or("current");
    workspace::init_feature(paths, feature_name, force)
}
