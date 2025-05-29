use clap::Parser;
use log::{LevelFilter, info};
use simplelog::TermLogger;

use crate::cli::{CliArg, info, link, run};
use crate::dir::{Dir, ensure_dir_exists};

mod cli;
mod dir;

fn init_logger() -> eyre::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )?;

    Ok(())
}

// LYN: Main

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    init_logger()?;
    ensure_dir_exists(Dir::App)?;

    let arg = CliArg::parse();
    info!("Parsed argument: {:?}", arg);

    match arg {
        CliArg::Run(arg) => run::main(arg)?,
        CliArg::Link(arg) => link::main(arg)?,
        CliArg::Info => info::main()?,
    }

    Ok(())
}
