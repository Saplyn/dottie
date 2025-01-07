use std::{fs, path::PathBuf, sync::LazyLock};

use clap::{error::Result, Parser};
use cli::CliArg;
use home::home_dir;
use log::{info, trace, LevelFilter, SetLoggerError};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod cli;

static DOTTIE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    home_dir()
        .expect("Failed to get home directory path")
        .join(".dottie")
});
const SCRIPTS_DIR_NAME: &str = "scripts";
const FILES_DIR_NAME: &str = "files";

fn main() -> anyhow::Result<()> {
    init_log()?;
    info!("Logger initialized");

    if !DOTTIE_DIR.try_exists()? {
        // TODO: handle broken symlink
        fs::create_dir(DOTTIE_DIR.as_path())?;
    };

    let arg = CliArg::parse();
    trace!("Parsed argument {:?}", arg);

    match arg {
        CliArg::Run(arg) => cli::run::main(arg)?,
        CliArg::Link(arg) => cli::link::main(arg)?,
    }

    Ok(())
}

fn init_log() -> Result<(), SetLoggerError> {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
}
