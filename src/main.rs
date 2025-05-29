use clap::Parser;
use log::{LevelFilter, trace};
use simplelog::TermLogger;

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
        CliArg::Info => info::main()?,
    }

    Ok(())
}

// LYN: Helpers

fn init_logger() -> eyre::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;

    Ok(())
}
