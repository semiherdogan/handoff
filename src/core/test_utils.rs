use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn make_temp_base(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("handoff-{label}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&dir).expect("failed to create temp test dir");
    dir
}
