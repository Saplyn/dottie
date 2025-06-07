use clap::Parser;

use crate::{link::LinkArg, run::RunArg};

pub mod info;
pub mod link;
pub mod run;

#[derive(Debug, Parser)]
pub enum CliArg {
    #[clap(about = "Execute scripts of packages")]
    Run(RunArg),

    #[clap(about = "Link files of packages")]
    Link(LinkArg),

    #[clap(about = "Display binary built info")]
    Info,
}
