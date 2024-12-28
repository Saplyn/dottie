use std::{
    io,
    os::unix,
    path::{Path, PathBuf},
};

use anyhow::Result;
use home::home_dir;
use log::{error, info, warn};
use thiserror::Error;

use crate::{cli::PackageArg, DOTTIE_DIR, FILES_DIR_NAME};

pub fn main(arg: PackageArg) -> anyhow::Result<()> {
    for package in arg.packages.iter() {
        link(package);
    }

    Ok(())
}

// LYN:

#[derive(Debug, Error)]
enum PackageError {
    #[error("Given package is inaccessible")]
    PackageInaccessible(Option<io::Error>),

    #[error("Files directory of given package is inaccessible")]
    FilesDirInaccessible(Option<io::Error>),

    #[error("Failed to read the files directory (or its subdirectory): {0:?}")]
    ErrorReadingFilesDir(io::Error),

    #[error("Failed to read the home directory (or its subdirectory): {0:?}")]
    ErrorReadingHomeDir(io::Error),

    #[error("Resolving failed: conflict directory found at destination {0}")]
    ConflictDirAtDest(PathBuf),
}

struct FilesSummary {
    file: PathBuf,
    res: io::Result<PathBuf>,
}

fn link(package: &str) -> Result<Vec<FilesSummary>, PackageError> {
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
    todo!("{:#?}", solution);

    let mut summary = Vec::new();
    for item in solution {
        let Err(err) = unix::fs::symlink(&item.orig, &item.link) else {
            summary.push(FilesSummary {
                file: item.orig,
                res: Ok(item.link),
            });
            continue;
        };

        summary.push(FilesSummary {
            file: item.orig,
            res: Err(err),
        });
    }

    Ok(summary)
}

type Solution = Vec<LinkPair>;
#[derive(Debug)]
struct LinkPair {
    orig: PathBuf,
    link: PathBuf,
}

fn resolve(src: &Path, dest_parent: &Path) -> Result<Solution, PackageError> {
    let mut solution = Vec::new();
    for entry in src.read_dir().map_err(PackageError::ErrorReadingFilesDir)? {
        let Ok(entry) = entry else {
            warn!("Error reading entry \"{:?}\", skipping", entry);
            continue;
        };

        // Try finding an entry under `dest_parent` with the same name
        let Some(dest_entry) = dest_parent
            .read_dir()
            .map_err(PackageError::ErrorReadingHomeDir)?
            .flatten()
            .find(|dest_entry| dest_entry.file_name() == entry.file_name())
        else {
            // No shared parent, link them directly, and continue to the next one
            solution.push(LinkPair {
                orig: entry.path(),
                link: dest_parent.join(entry.path().file_name().unwrap()),
            });
            continue;
        };

        // Found an entry with the same name under `dest_parent`
        info!(
            "Found an entry with the same name: {:?} & {:?}",
            entry, dest_entry
        );
        match (
            entry
                .file_type()
                .map_err(PackageError::ErrorReadingFilesDir)?,
            dest_entry
                .file_type()
                .map_err(PackageError::ErrorReadingHomeDir)?,
        ) {
            // If both entries are directory, continue operation (recursively)
            (src_ty, dest_ty) if src_ty.is_dir() && dest_ty.is_dir() => (),
            // If target is symlink
            (_, dest_ty)
                if dest_ty.is_symlink()
                    && dest_entry
                        .path()
                        .read_link()
                        .map_err(PackageError::ErrorReadingHomeDir)?
                        == entry.path() =>
            {
                warn!("Skipping already linked entry: {:?}", entry);
                continue;
            }
            // If either of them is a file, then abort resolving
            (_, _) => {
                // TODO: Error message
                error!("Conflict {:?}, {:?}", entry, dest_entry);
                return Err(PackageError::ConflictDirAtDest(dest_entry.path()));
            }
        }

        //  Resolve linking solution for every entry under it
        for sub_entry in entry
            .path()
            .read_dir()
            .map_err(PackageError::ErrorReadingFilesDir)?
        {
            let Ok(sub_entry) = sub_entry else {
                warn!("Error reading entry \"{:?}\", skipping", entry);
                continue;
            };

            match sub_entry
                .file_type()
                .map_err(PackageError::ErrorReadingFilesDir)?
            {
                ty if ty.is_dir() => {
                    // If it's a dir, then recursively develop solution
                    error!("resolve({:?}, {:?})", sub_entry.path(), dest_entry.path());
                    let mut sub_sol = resolve(&sub_entry.path(), &dest_entry.path())?;
                    solution.append(&mut sub_sol);
                }
                ty if ty.is_symlink() => {
                    // If it's a symlink, check if it's linked by dottie
                    warn!("UWU: {:?}, {:?}", sub_entry.path(), dest_entry.path());
                }
                _ => {
                    // Otherwise, check if destination is unoccupied
                    let dest = dest_entry
                        .path()
                        .join(sub_entry.path().file_name().unwrap());

                    // If occupied, check if it's a symlink created by dottie
                    if dest
                        .try_exists()
                        .map_err(PackageError::ErrorReadingHomeDir)?
                    {
                        if dest.is_symlink()
                            && dest
                                .read_link()
                                .map_err(PackageError::ErrorReadingHomeDir)?
                                == sub_entry.path()
                        {
                            warn!("Skipping already linked entry: {:?}", entry);
                            continue;
                        }
                        error!("Conflict {:?}, {:?}", sub_entry.path(), dest);
                        return Err(PackageError::ConflictDirAtDest(dest));
                    }

                    solution.push(LinkPair {
                        orig: sub_entry.path(),
                        link: dest_entry
                            .path()
                            .join(sub_entry.path().file_name().unwrap()),
                    });
                }
            }
        }
    }

    Ok(solution)
}
