use std::{convert::Infallible, fmt::Display};

use clap::Args;
use owo_colors::OwoColorize;

#[derive(Debug, Args)]
pub struct InfoArg {
    #[arg(short = 'T', long, help = "Print build target information")]
    pub target: bool,
    #[arg(short = 'H', long, help = "Print build host information")]
    pub host: bool,
    #[arg(short = 't', long, help = "Print build time")]
    pub time: bool,
    #[arg(short = 'p', long, help = "Print build profile")]
    pub profile: bool,
    #[arg(short = 'r', long, help = "Print version of Rust toolchain used")]
    pub rust: bool,
    #[clap(short = 'v', long, help = "Print version of the application")]
    pub version: bool,
}

// LYN: Helper Macros

macro_rules! color {
    ($s:expr) => {
        $s.bold().bright_green()
    };
}

// LYN: Main

pub fn main(arg: &InfoArg) -> Result<(), Infallible> {
    if !print_flagged(arg) {
        println!("{}: {}", color!("Build Target"), build::TARGET);
        println!("{}: {}", color!("Build Host"), build_host());
        println!("{}: {}", color!("Build Time"), build_time());
        println!("{}: {}", color!("Build Profile"), build::PROFILE);
        println!("{}: {}", color!("Rust Version"), build::RUSTC_VERSION);
        println!("{}: {}", color!("Version"), build_version());
    }

    Ok(())
}

fn print_flagged(arg: &InfoArg) -> bool {
    let mut flagged = false;
    if arg.target {
        flagged = true;
        println!("{}", build::TARGET);
    }
    if arg.host {
        flagged = true;
        println!("{}", build_host());
    }
    if arg.time {
        flagged = true;
        println!("{}", build_time().format("%Y-%m-%d %H:%M:%S %Z"));
    }
    if arg.profile {
        flagged = true;
        println!("{}", build::PROFILE);
    }
    if arg.rust {
        flagged = true;
        println!("{}", build::RUSTC_VERSION);
    }
    if arg.version {
        flagged = true;
        println!("{}", build_version());
    }
    flagged
}

// LYN: Build Info

pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn build_time() -> built::chrono::DateTime<built::chrono::Local> {
    built::util::strptime(build::BUILT_TIME_UTC).with_timezone(&built::chrono::offset::Local)
}

fn build_version() -> impl Display {
    format!("v{}", build::PKG_VERSION)
}

fn build_host() -> impl Display {
    match build::CI_PLATFORM {
        None => build::HOST.to_owned(),
        Some(ci) => format!("{} ({})", build::HOST, ci),
    }
}
