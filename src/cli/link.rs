use clap::Args;

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

// LYN: Main

pub fn main(arg: LinkArg) -> eyre::Result<()> {
    Ok(())
}
