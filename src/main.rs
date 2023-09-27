mod catc;
mod cli;

use anyhow;
use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let rs = Cli::parse();
    catc::handle(rs.name)?;
    Ok(())
}
