use bales::cli::Arguments;
use clap::Parser;
fn main() -> anyhow::Result<()> {
    Arguments::parse().handle()?;
    Ok(())
}
