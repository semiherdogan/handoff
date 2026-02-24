use anyhow::{Context, Result};
use std::io::{self, Write};

pub fn prompt_yes_no(message: &str) -> Result<bool> {
    print!("{message}");
    io::stdout()
        .flush()
        .context("Failed to flush confirmation prompt")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read confirmation response")?;

    Ok(parse_confirmation_response(&input))
}

pub fn parse_confirmation_response(input: &str) -> bool {
    matches!(input.trim(), "y" | "Y" | "yes" | "YES" | "Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_confirmation_response_accepts_yes_variants() {
        assert!(parse_confirmation_response("y"));
        assert!(parse_confirmation_response("Y"));
        assert!(parse_confirmation_response("yes"));
        assert!(parse_confirmation_response("Yes\n"));
    }

    #[test]
    fn parse_confirmation_response_defaults_to_no() {
        assert!(!parse_confirmation_response(""));
        assert!(!parse_confirmation_response("n"));
        assert!(!parse_confirmation_response("no"));
        assert!(!parse_confirmation_response("anything-else"));
    }
}
