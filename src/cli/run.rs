use std::{
    convert::Infallible,
    io,
    process::{Command, Output},
};

use log::{error, info, warn};
use thiserror::Error;

use crate::{cli::PackageArg, DOTTIE_DIR, SCRIPTS_DIR_NAME};

pub fn main(arg: PackageArg) -> Result<(), Infallible> {
    let mut summary = Vec::new();
    for package in arg.packages.iter() {
        summary.push((package, run(package)));
    }

    println!("=== Execution Summary ===");
    for (package, res) in summary {
        match res {
            Ok(summaries) => {
                println!("Package \"{}\" executed the following scripts:", package);
                for ScriptSummary { script, result } in summaries {
                    match result {
                        Ok(output) if output.status.success() => {
                            println!("- \"{}\" successfully finished.", script);
                        }
                        Ok(output) if output.status.code().is_some() => {
                            println!(
                                "- \"{}\" finished with status code {}.",
                                script,
                                output.status.code().unwrap()
                            );
                        }
                        Ok(_) => {
                            println!("- \"{}\" finished with no status code.", script);
                        }
                        Err(err) => println!("- \"{:?}\" failed with error: {}.", script, err),
                    }
                }
            }
            Err(PackageError::PackageInaccessible(None)) => {
                println!("Package \"{}\" doesn't exist.", package);
            }
            Err(PackageError::PackageInaccessible(e)) => {
                println!("Failed to read package \"{}\": {:?}.", package, e);
            }
            Err(PackageError::ScriptDirInaccessible(None)) => {
                println!("Package \"{}\" doesn't have scripts directory.", package);
            }
            Err(PackageError::ScriptDirInaccessible(e)) => {
                println!(
                    "Failed to read package \"{}\"'s scripts directory: {:?}.",
                    package, e
                );
            }
            Err(e) => println!("{:?}.", e),
        }
    }
    println!("=========================");

    Ok(())
}

// LYN:

#[derive(Debug, Error)]
enum PackageError {
    #[error("Given package is inaccessible")]
    PackageInaccessible(Option<io::Error>),

    #[error("Script directory of given package is inaccessible")]
    ScriptDirInaccessible(Option<io::Error>),

    #[error("Failed to read the scripts directory: {0:?}")]
    ErrorReadingScriptsDir(io::Error),
}

#[derive(Debug)]
struct ScriptSummary {
    script: String,
    result: io::Result<Output>,
}

fn run(package: &str) -> Result<Vec<ScriptSummary>, PackageError> {
    let dir = DOTTIE_DIR.join(package);

    match dir.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            warn!("Package {:?} doesn't exist", package);
            return Err(PackageError::PackageInaccessible(None));
        }
        Err(err) => {
            warn!(
                "Existence of package {:?} connot be comfirmed: {:?}",
                package, err
            );
            return Err(PackageError::PackageInaccessible(Some(err)));
        }
    };

    let dir = dir.join(SCRIPTS_DIR_NAME);
    match dir.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            warn!("Package {:?} has no script directory", package);
            return Err(PackageError::ScriptDirInaccessible(None));
        }
        Err(err) => {
            warn!(
                "Existence of package {:?}'s script directory connot be comfirmed: {:?}",
                package, err
            );
            return Err(PackageError::PackageInaccessible(Some(err)));
        }
    };

    let mut summaries = Vec::new();
    for entry in dir
        .read_dir()
        .map_err(PackageError::ErrorReadingScriptsDir)?
    {
        let Ok(entry) = entry else {
            warn!("Error reading entry \"{:?}\", skipping", entry);
            continue;
        };

        info!("Executing script {:?}", entry.path());
        summaries.push(ScriptSummary {
            script: entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("*Unkown*")
                .to_owned(),
            result: Command::new("sh").arg(entry.path()).output(),
        });
    }

    Ok(summaries)
}
