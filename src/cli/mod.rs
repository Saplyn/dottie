use clap::{Args, Parser};

pub mod link;
pub mod run;

#[derive(Debug, Clone, Parser)]
pub enum CliArg {
    // TODO: help message
    #[clap(about = "TODO")]
    Run(PackageArg),

    // TODO: help message
    #[clap(about = "TODO")]
    Link(PackageArg),
}

#[derive(Debug, Clone, Args)]
pub struct PackageArg {
    // TODO: help message
    #[arg(group = "specify", required = true, help = "TODO")]
    pub packages: Vec<String>,

    // TODO: help message
    #[arg(short, long, group = "specify", help = "TODO")]
    pub all: bool,
}
