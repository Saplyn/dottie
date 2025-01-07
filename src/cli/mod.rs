use clap::{Args, Parser};

pub mod link;
pub mod run;

#[derive(Debug, Clone, Parser)]
pub enum CliArg {
    // TODO: help message
    #[clap(about = "TODO")]
    Run(RunArg),

    // TODO: help message
    #[clap(about = "TODO")]
    Link(LinkArg),
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
