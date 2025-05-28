use clap::{Args, Parser};

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
    #[clap(about = "Built info")]
    Info,
}

#[derive(Debug, Clone, Args)]
pub struct RunArg {
    // TODO: help message
    #[arg(group = "specify", required = true, help = "TODO")]
    pub packages: Vec<String>,

    // TODO: help message
    #[arg(short, long, group = "specify", help = "TODO")]
    pub all: bool,
}

#[derive(Debug, Clone, Args)]
pub struct LinkArg {
    // TODO: help message
    #[arg(group = "specify", required = true, help = "TODO")]
    pub packages: Vec<String>,

    // TODO: help message
    #[arg(short, long, group = "specify", help = "TODO")]
    pub all: bool,

    // TODO: help message
    #[arg(short, long, help = "TODO")]
    pub force: bool,
}
