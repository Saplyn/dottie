use std::fmt::Display;

use owo_colors::OwoColorize;

pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

macro_rules! color {
    ($s:expr) => {
        $s.bold().bright_green()
    };
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

pub fn main() {
    println!("{}: {}", color!("Build Target"), build::TARGET);
    println!("{}: {}", color!("Build Host"), build_host());
    println!("{}: {}", color!("Build Time"), build_time());
    println!("{}: {}", color!("Build Profile"), build::PROFILE);
    println!("{}: {}", color!("Rust Version"), build::RUSTC_VERSION);
    println!("{}: {}", color!("Version"), build_version());
}
