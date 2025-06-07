use std::convert::Infallible;

use clap::Parser;
use env_logger::Target;
use log::trace;

use crate::{
    cli::{CliArg, info, link, run},
    dir::{Dir, ensure_exists},
};

mod cli;
mod dir;

// LYN: Main

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    init_logger()?;
    ensure_exists(Dir::App)?;

    let arg = CliArg::parse();
    trace!("Parsed argument: {:?}", arg);

    match arg {
        CliArg::Run(arg) => run::main(&arg)?,
        CliArg::Link(arg) => link::main(&arg)?,
        CliArg::Info(arg) => info::main(&arg)?,
    }

    Ok(())
}

// LYN: Helpers

fn init_logger() -> Result<(), Infallible> {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .init();
    Ok(())
}
