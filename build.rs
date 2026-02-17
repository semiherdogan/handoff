use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=version.txt");

    let fallback = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let version = fs::read_to_string("version.txt")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback);

    println!("cargo:rustc-env=HANDOFF_VERSION={version}");
}
