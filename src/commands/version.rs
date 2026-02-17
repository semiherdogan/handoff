use anyhow::Result;

pub fn run() -> Result<()> {
    println!("{}", env!("HANDOFF_VERSION"));
    Ok(())
}
