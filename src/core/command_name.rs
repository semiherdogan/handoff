use std::ffi::OsString;
use std::path::Path;

const FALLBACK_COMMAND_NAME: &str = "handoff";

pub fn current() -> String {
    from_arg0(std::env::args_os().next())
}

fn from_arg0(arg0: Option<OsString>) -> String {
    arg0.as_deref()
        .map(Path::new)
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(FALLBACK_COMMAND_NAME)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::from_arg0;
    use std::ffi::OsString;

    #[test]
    fn uses_binary_file_name() {
        assert_eq!(from_arg0(Some(OsString::from("/usr/local/bin/ho"))), "ho");
    }

    #[test]
    fn falls_back_when_arg0_is_missing() {
        assert_eq!(from_arg0(None), "handoff");
    }
}
