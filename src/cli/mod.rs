use clap::Parser;

use crate::link::LinkArg;
use crate::run::RunArg;

pub mod info;
pub mod link;
pub mod run;

#[derive(Debug, Clone, Parser)]
pub enum CliArg {
    // TODO: help message
    #[clap(about = "Execute scripts of packages")]
    Run(RunArg),

    // TODO: help message
    #[clap(about = "Link files of packages")]
    Link(LinkArg),

    // TODO: help message
    #[clap(about = "Display binary built info")]
    Info,
}
