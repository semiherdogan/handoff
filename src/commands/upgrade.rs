use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::fs;
use std::io::Write;

const GITHUB_RELEASES_URL: &str =
    "https://api.github.com/repos/semiherdogan/handoff/releases/latest";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

pub fn run() -> Result<()> {
    let current_version = env!("HANDOFF_VERSION");
    println!("Current version: {current_version}");

    println!("Checking for updates...");
    let release = fetch_latest_release()?;

    let latest_version = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);

    if !is_newer(current_version, latest_version) {
        println!("Already up to date.");
        return Ok(());
    }

    println!("New version available: {latest_version}");

    let target = current_target_triple();
    let asset_name = format!("handoff-{target}");

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| {
            anyhow!(
                "No release asset found for target '{target}'. Available assets: {}",
                release
                    .assets
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    println!("Downloading {asset_name}...");
    let binary_data = download_asset(&asset.browser_download_url)?;

    let current_exe = std::env::current_exe()
        .context("Failed to determine current executable path")?;

    replace_binary(&current_exe, &binary_data)?;

    println!("Upgraded to {latest_version} successfully.");
    Ok(())
}

fn fetch_latest_release() -> Result<GitHubRelease> {
    let response = ureq::get(GITHUB_RELEASES_URL)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "handoff-cli")
        .call()
        .map_err(|e| anyhow!("Failed to fetch latest release: {e}"))?;

    let release: GitHubRelease = response
        .into_json()
        .context("Failed to parse release response")?;

    Ok(release)
}

fn download_asset(url: &str) -> Result<Vec<u8>> {
    let response = ureq::get(url)
        .set("User-Agent", "handoff-cli")
        .call()
        .map_err(|e| anyhow!("Failed to download asset: {e}"))?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .context("Failed to read downloaded binary")?;

    Ok(bytes)
}

fn replace_binary(current_exe: &std::path::Path, new_binary: &[u8]) -> Result<()> {
    let backup_path = current_exe.with_extension("old");

    // Rename current binary to .old backup
    fs::rename(current_exe, &backup_path).with_context(|| {
        format!(
            "Failed to back up current binary. You may need to run with elevated permissions.\n  from: {}\n  to: {}",
            current_exe.display(),
            backup_path.display()
        )
    })?;

    // Write new binary
    let write_result = (|| -> Result<()> {
        let mut file = fs::File::create(current_exe)
            .with_context(|| format!("Failed to create new binary at {}", current_exe.display()))?;
        file.write_all(new_binary)
            .context("Failed to write new binary")?;
        file.flush()?;
        set_executable_permissions(current_exe)?;
        Ok(())
    })();

    if let Err(e) = write_result {
        // Attempt to restore backup on failure
        let _ = fs::rename(&backup_path, current_exe);
        return Err(e);
    }

    // Remove backup
    let _ = fs::remove_file(&backup_path);

    Ok(())
}

#[cfg(unix)]
fn set_executable_permissions(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::Permissions::from_mode(0o755);
    fs::set_permissions(path, perms)
        .with_context(|| format!("Failed to set executable permissions on {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_executable_permissions(_path: &std::path::Path) -> Result<()> {
    Ok(())
}

fn current_target_triple() -> &'static str {
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    {
        "x86_64-unknown-linux-gnu"
    }
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        "aarch64-apple-darwin"
    }
    #[cfg(not(any(
        all(target_arch = "x86_64", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "macos"),
    )))]
    {
        "unknown"
    }
}

/// Compare two semver-like version strings. Returns true if `latest` is newer than `current`.
fn is_newer(current: &str, latest: &str) -> bool {
    let parse = |v: &str| -> (u64, u64, u64) {
        let parts: Vec<u64> = v
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };

    let c = parse(current);
    let l = parse(latest);
    l > c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_newer_detects_major_bump() {
        assert!(is_newer("0.1.0", "1.0.0"));
    }

    #[test]
    fn is_newer_detects_minor_bump() {
        assert!(is_newer("0.1.0", "0.2.0"));
    }

    #[test]
    fn is_newer_detects_patch_bump() {
        assert!(is_newer("0.1.0", "0.1.1"));
    }

    #[test]
    fn is_newer_returns_false_for_same_version() {
        assert!(!is_newer("1.2.3", "1.2.3"));
    }

    #[test]
    fn is_newer_returns_false_for_older_version() {
        assert!(!is_newer("1.0.0", "0.9.9"));
    }

    #[test]
    fn current_target_triple_is_not_unknown() {
        let triple = current_target_triple();
        assert_ne!(triple, "unknown", "target triple should be recognized");
    }
}
