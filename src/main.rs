use anyhow::{Context, Result};
use usage_tracker::load_usage_information_from_default;

fn main() -> Result<()> {
    dbg!(load_usage_information_from_default().context("File read failed")?);

    Ok(())
}
