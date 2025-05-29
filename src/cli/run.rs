use clap::Args;

use crate::dir::{Dir, ensure_dir_exists};

#[derive(Debug, Clone, Args)]
pub struct RunArg {
    // TODO: help message
    #[arg(group = "specify", required = true, help = "TODO")]
    pub packages: Vec<String>,

    // TODO: help message
    #[arg(short, long, group = "specify", help = "TODO")]
    pub all: bool,
}

// LYN: Main

pub fn main(arg: RunArg) -> eyre::Result<()> {
    todo!();

    Ok(())
}
