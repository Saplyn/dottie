use std::{
    fs::DirEntry,
    io,
    os::unix,
    path::{Path, PathBuf},
};

use home::home_dir;
use log::{error, info, trace, warn};
use thiserror::Error;

use crate::{cli::PackageArg, DOTTIE_DIR, FILES_DIR_NAME};

pub fn main(arg: PackageArg) -> anyhow::Result<()> {
    let mut summary = Vec::new();
    for package in arg.packages.iter() {
        summary.push((package, link(package)));
    }

    println!("=== Execution Summary ===");
    for (package, res) in summary {
        match res {
            Ok(summaries) => {
                println!("Package \"{}\" tried linking the following files:", package);
                for res in summaries {
                    match res {
                        FileSummary::Linked { file, link } => {
                            println!("- Successfully linked: {:?} <- {:?}", file, link);
                        }
                        FileSummary::Failed { file, reason } => {
                            println!("- Failed to link {:?} because: {:?}", file, reason);
                        }
                        FileSummary::Ignored { file } => {
                            println!("- Ignored {:?} as already linked", file);
                        }
                        FileSummary::Skipped { file } => {
                            println!("- Skipped {:?} as requested by user (force link)", file);
                        }
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}

// LYN:

#[derive(Debug, Error)]
enum PackageError {
    #[error("Given package is inaccessible: {0:?}")]
    PackageInaccessible(Option<io::Error>),

    #[error("Files directory of given package is inaccessible: {0:?}")]
    FilesDirInaccessible(Option<io::Error>),

    #[error("Failed to read the files directory (or its subdirectory): {0:?}")]
    ErrorReadingFilesDir(io::Error),

    #[error("Failed to read the home directory (or its subdirectory): {0:?}")]
    ErrorReadingHomeDir(io::Error),

    #[error("Conflict files found at destination (don't force link): {0:?}")]
    ConflictFilesNoForce(Vec<PathBuf>),
}

enum FileSummary {
    Linked { file: PathBuf, link: PathBuf },
    Failed { file: PathBuf, reason: io::Error },
    Ignored { file: PathBuf },
    Skipped { file: PathBuf },
}

fn link(package: &str) -> Result<Vec<FileSummary>, PackageError> {
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

    let dir = dir.join(FILES_DIR_NAME);
    match dir.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            warn!("Package {:?} has no files directory", package);
            return Err(PackageError::FilesDirInaccessible(None));
        }
        Err(err) => {
            warn!(
                "Existence of package {:?}'s files directory connot be comfirmed: {:?}",
                package, err
            );
            return Err(PackageError::PackageInaccessible(Some(err)));
        }
    };

    let solution = resolve(&dir, &home_dir().unwrap())?;
    if !solution.all_good {
        // TODO: message
        warn!(
            "Not all files of package \"{}\" can be safely linked",
            package
        );
        return Err(PackageError::ConflictFilesNoForce(
            solution
                .link_opt
                .into_iter()
                .filter_map(|opt| match opt {
                    LinkOpt::Skip { file } => Some(file),
                    _ => None,
                })
                .collect(),
        ));
    }

    let mut summary = Vec::new();
    for opt in solution.link_opt {
        match opt {
            LinkOpt::Link { file, link } => {
                info!("Symlinking {:?} -> {:?}", link, file);
                match unix::fs::symlink(&file, &link) {
                    Ok(()) => summary.push(FileSummary::Linked { file, link }),
                    Err(e) => summary.push(FileSummary::Failed { file, reason: e }),
                }
            }
            LinkOpt::Ignore { file } => {
                info!("Ignoring {:?} as already linked", file);
                summary.push(FileSummary::Ignored { file });
            }
            LinkOpt::Skip { file } => {
                warn!("Skipping {:?} because destination is occupied", file);
                summary.push(FileSummary::Skipped { file });
            }
        }
    }

    Ok(summary)
}

#[derive(Debug)]
struct Solution {
    all_good: bool,
    link_opt: Vec<LinkOpt>,
}
impl Solution {
    fn new() -> Self {
        Self {
            all_good: true,
            link_opt: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum LinkOpt {
    Link { file: PathBuf, link: PathBuf },
    Ignore { file: PathBuf },
    Skip { file: PathBuf },
}

fn resolve(src_parent: &Path, dest_parent: &Path) -> Result<Solution, PackageError> {
    assert!(
        src_parent.is_dir(),
        "`resolve()` called with `src_parent` not being a dir"
    );

    let dest_entries: Vec<DirEntry> = dest_parent
        .read_dir()
        .map_err(PackageError::ErrorReadingHomeDir)?
        .flatten()
        .collect();
    let mut solution = Solution::new();
    for src_entry in src_parent
        .read_dir()
        .map_err(PackageError::ErrorReadingFilesDir)?
    {
        let Ok(src_entry) = src_entry else {
            warn!("Skipping broken dir entry");
            continue;
        };
        let src_name = src_entry.file_name();
        let src_path = src_entry.path();

        match (
            src_entry
                .file_type()
                .map_err(PackageError::ErrorReadingFilesDir)?,
            dest_entries
                .iter()
                .find(|entry| entry.file_name() == src_name),
        ) {
            // If the destination is unoccupied, just link it
            (_, None) => {
                trace!("{:?} ~~> {:?}", dest_parent.join(&src_name), src_path);
                solution.link_opt.push(LinkOpt::Link {
                    file: src_path,
                    link: dest_parent.join(src_name),
                });
            }
            // If there's a symlink there, check if it's created by dottie
            (_, Some(dest_entry))
                if dest_entry
                    .file_type()
                    .map_err(PackageError::ErrorReadingHomeDir)?
                    .is_symlink()
                    && dest_entry
                        .path()
                        .read_link()
                        .is_ok_and(|path| path == src_path) =>
            {
                trace!("# ~~> {:?}", src_path);
                solution.link_opt.push(LinkOpt::Ignore { file: src_path });
            }
            // If both are directories, resolve recursively
            (ty, Some(dest_entry))
                if ty.is_dir() && dest_entry.file_type().is_ok_and(|ty| ty.is_dir()) =>
            {
                let mut sub_solution = resolve(&src_entry.path(), &dest_entry.path())?;
                solution.all_good &= sub_solution.all_good;
                solution.link_opt.append(&mut sub_solution.link_opt);
            }
            // Otherwise, failed
            (_, Some(_)) => {
                trace!("[!] {:?}", src_path);
                solution.all_good = false;
                solution.link_opt.push(LinkOpt::Skip { file: src_path });
            }
        }
    }

    Ok(solution)
}
