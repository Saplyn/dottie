use std::process::{Command, Output};

use clap::Args;
use eyre::eyre;
use log::{trace, warn};

use crate::dir::{Dir, exists, get};

// LYN: Arguments

#[derive(Debug, Clone, Args)]
pub struct RunArg {
    /// The package(s) whose scripts will be run
    #[arg(
        value_name = "packages",
        group = "specify",
        required = true,
        help = "The package(s) whose scripts will be run"
    )]
    pub pkgs: Vec<String>,

    /// Run scripts for all packages
    #[arg(short, long, group = "specify", help = "Run scripts for all packages")]
    pub all: bool,

    /// Dry run mode, only prints what scripts would be run
    #[arg(
        short,
        long,
        help = "Dry run mode, only print what scripts would be run"
    )]
    pub dry: bool,
}

// LYN: Main

pub fn main(arg: &RunArg) -> eyre::Result<()> {
    let summary = if arg.all {
        run_all(arg)?
    } else {
        run_specified(arg)?
    };

    todo!("Display summary: {:#?}", summary);

    Ok(())
}

// LYN: Run Scripts

#[derive(Debug, Clone, Default)]
struct RunSummary {
    /// Packages whose scripts were ran and it's outputs
    outputs: Vec<RunOutput>,
    /// package names that do not exist
    non_exist: Vec<String>,
    /// package names that do not have scripts
    no_scripts: Vec<String>,
}

#[derive(Debug, Clone)]
struct RunOutput {
    pkg_name: String,
    output_pack: Vec<RunOutputPack>,
}

#[derive(Debug, Clone)]
struct RunOutputPack {
    script_name: String,
    output: Output,
}

fn run_all(arg: &RunArg) -> eyre::Result<RunSummary> {
    todo!("Handle all packages");
}

fn run_specified(arg: &RunArg) -> eyre::Result<RunSummary> {
    let mut summary = RunSummary::default();

    for pkg_name in &arg.pkgs {
        if !exists(Dir::Pkg { pkg_name })? {
            summary.non_exist.push(pkg_name.clone());
            warn!("Package `{}` does not exist", pkg_name);
            continue;
        }
        if !exists(Dir::Scripts { pkg_name })? {
            summary.no_scripts.push(pkg_name.clone());
            warn!("Package `{}` does not have a scripts folder", pkg_name);
            continue;
        }
        let mut output_pack = Vec::new();
        for script_entry in get(Dir::Scripts { pkg_name }).read_dir()? {
            let script_entry = script_entry?;
            let script_path = script_entry.path();
            let script_name = script_path
                .file_name()
                .map(|os| os.to_string_lossy().into_owned())
                .ok_or_else(|| eyre!("Failed to get script name for `{}`", pkg_name))?;

            trace!(
                "Executing script `{}` for package `{}`",
                script_name, pkg_name
            );

            let output = Command::new(&script_path).output()?;
            trace!("script finished with output: {:?}", output);
            output_pack.push(RunOutputPack {
                script_name,
                output,
            });
        }
        summary.outputs.push(RunOutput {
            pkg_name: pkg_name.clone(),
            output_pack,
        });
    }

    Ok(summary)
}
