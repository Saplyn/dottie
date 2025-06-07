use std::{
    ffi::OsString,
    fs, io,
    os::unix,
    path::{Path, PathBuf},
};

use clap::Args;
use log::{error, info, trace, warn};
use owo_colors::OwoColorize;
use thiserror::Error;

use crate::dir::{Dir, HOME_DIR, exists, get, not_package};

#[derive(Debug, Args)]
pub struct LinkArg {
    /// The package(s) whose files will be linked
    #[arg(
        value_name = "packages",
        group = "specify",
        required = true,
        help = "The package(s) whose files will be linked"
    )]
    pub pkgs: Vec<String>,

    /// Link files for all packages
    #[arg(short, long, group = "specify", help = "Link files for all packages")]
    pub all: bool,

    /// Force link and override if possible, even if some files may not be linked
    #[arg(
        short,
        long,
        help = "Force link and override if possible, even if some files may not be linked"
    )]
    pub force: bool,

    /// Dry run mode, only prints what files would be linked
    #[arg(short, long, help = "Dry run mode, only print what files would linked")]
    pub dry: bool,
}

// LYN: Main

pub fn main(arg: &LinkArg) -> eyre::Result<()> {
    let summary = if arg.all {
        link_all(arg)?
    } else {
        link_specified(arg)?
    };

    summary.display();

    Ok(())
}

// LYN: Link Summary

#[derive(Debug, Default)]
struct LinkSummary {
    /// Packages whose files were linked and the linking detail
    details: Vec<LinkDetail>,
    /// package names that do not exist
    non_exist: Vec<String>,
    /// package names that do not have files
    no_files: Vec<String>,
}

impl LinkSummary {
    fn display(&self) {
        println!("{}", "Link Summary:".bold().bright_green());

        for detail in &self.details {
            println!(
                "- Package {} processed {} file(s)",
                format!("`{}`", detail.pkg_name).yellow(),
                detail.detail_pack.len()
            );
            for pack in &detail.detail_pack {
                match pack {
                    LinkDetailPack::Linkable {
                        src_path,
                        dest_path,
                        linked,
                    } => {
                        if let Some(linked) = linked {
                            if let Err(e) = linked {
                                println!(
                                    "  - {} {} due to {e}",
                                    "Failed".bold().on_red(),
                                    format!("`{}`", dest_path.display()).cyan(),
                                );
                            } else {
                                println!(
                                    "  - {} {} {}",
                                    "Linked".bright_green(),
                                    format!("`{}`", dest_path.display()).cyan(),
                                    format!("-> `{}`", src_path.display()).bright_black(),
                                );
                            }
                        } else {
                            println!(
                                "  - {} {} {}",
                                "Planed".bright_green(),
                                format!("`{}`", dest_path.display()).cyan(),
                                format!("-> `{}`", src_path.display()).bright_black(),
                            );
                        }
                    }
                    LinkDetailPack::AlreadyLinked {
                        src_path,
                        dest_path,
                    } => {
                        println!(
                            "  - {} {} {}",
                            "Exists".bright_green(),
                            format!("`{}`", dest_path.display()).cyan(),
                            format!("-> `{}`", src_path.display()).bright_black(),
                        );
                    }
                    LinkDetailPack::DestOccupied {
                        src_path,
                        dest_path,
                        force_linked,
                    } => {
                        if let Some(force_linked) = force_linked {
                            if let Err(e) = force_linked {
                                println!(
                                    "  - {} {} due to {e}",
                                    "Failed".bold().on_red(),
                                    format!("`{}`", dest_path.display()).cyan(),
                                );
                            } else {
                                println!(
                                    "  - {} {} {}",
                                    "Forced".bright_green(),
                                    format!("`{}`", dest_path.display()).cyan(),
                                    format!("-> `{}`", src_path.display()).bright_black(),
                                );
                            }
                        } else {
                            println!(
                                "  - {} {} exists and is not a valid symlink",
                                "Failed".bold().on_red(),
                                format!("`{}`", dest_path.display()).cyan(),
                            );
                        }
                    }
                }
            }
        }
        for pkg_name in &self.non_exist {
            println!(
                "- Package {} doesn't exist",
                format!("`{}`", pkg_name).yellow()
            );
        }
        for pkg_name in &self.no_files {
            println!(
                "- Package {} has no files to link",
                format!("`{}`", pkg_name).yellow()
            );
        }
    }
}

#[derive(Debug)]
struct LinkDetail {
    /// The name of the package whose files were link
    pkg_name: String,
    /// The detail of the files linked for the package
    detail_pack: Vec<LinkDetailPack>,
}

#[derive(Debug)]
enum LinkDetailPack {
    Linkable {
        src_path: PathBuf,
        dest_path: PathBuf,
        linked: Option<io::Result<()>>,
    },
    AlreadyLinked {
        src_path: PathBuf,
        dest_path: PathBuf,
    },
    DestOccupied {
        src_path: PathBuf,
        dest_path: PathBuf,
        force_linked: Option<io::Result<()>>,
    },
}

// LYN: Linke Files

#[derive(Debug, Error)]
enum LinkError {
    #[error("Failed to parse UTF8 OsString: {0:?}")]
    InvalidUtf8OsString(OsString),
    #[error("Impossible nameless path generated by program: {0}")]
    ImpossibleNamelessPath(PathBuf),
    #[error("Cannot confirm existence of {0}: {1}")]
    CannotConfirmFileExistence(PathBuf, io::Error),
}

/// Link files for all packages
fn link_all(arg: &LinkArg) -> eyre::Result<LinkSummary> {
    let mut summary = LinkSummary::default();
    for pkg_entry in get(Dir::App).read_dir()? {
        let pkg_entry = pkg_entry?;
        if not_package(&pkg_entry.path()) {
            info!("Skipping non backage entry: {:?}", pkg_entry);
            continue;
        }
        let pkg_name = pkg_entry
            .file_name()
            .into_string()
            .map_err(LinkError::InvalidUtf8OsString)?;
        if !exists(Dir::Scripts {
            pkg_name: pkg_name.to_owned(),
        })? {
            summary.no_files.push(pkg_name.to_owned());
            warn!("Package `{}` does not have a scripts folder", pkg_name);
            continue;
        }

        let mut detail_pack = prob_link(
            &get(Dir::Files {
                pkg_name: pkg_name.clone(),
            }),
            HOME_DIR.as_path(),
        )?;
        if !arg.dry {
            make_link(&mut detail_pack, arg)?;
        }
        summary.details.push(LinkDetail {
            pkg_name,
            detail_pack,
        });
    }

    Ok(summary)
}

/// Link files for specified packages
fn link_specified(arg: &LinkArg) -> eyre::Result<LinkSummary> {
    let mut summary = LinkSummary::default();
    for pkg_name in &arg.pkgs {
        if !exists(Dir::Pkg {
            pkg_name: pkg_name.to_owned(),
        })? {
            summary.non_exist.push(pkg_name.to_owned());
            warn!("Package `{}` does not exist", pkg_name);
            continue;
        }
        if !exists(Dir::Scripts {
            pkg_name: pkg_name.to_owned(),
        })? {
            summary.no_files.push(pkg_name.to_owned());
            warn!("Package `{}` does not have a scripts folder", pkg_name);
            continue;
        }

        let mut detail_pack = prob_link(
            &get(Dir::Files {
                pkg_name: pkg_name.to_owned(),
            }),
            HOME_DIR.as_path(),
        )?;
        if !arg.dry {
            make_link(&mut detail_pack, arg)?;
        }
        summary.details.push(LinkDetail {
            pkg_name: pkg_name.to_owned(),
            detail_pack,
        });
    }
    Ok(summary)
}

/// Test if the files in the given path can be linked to the target path
fn prob_link(path: &Path, target: &Path) -> eyre::Result<Vec<LinkDetailPack>> {
    let mut detail_pack = Vec::new();
    for file_entry in path.read_dir()? {
        let file_entry = file_entry?;
        let src = file_entry.path();
        let dest = target.join(
            src.file_name()
                .ok_or_else(|| LinkError::ImpossibleNamelessPath(src.clone()))?,
        );

        if !dest
            .try_exists()
            .map_err(|e| LinkError::CannotConfirmFileExistence(dest.to_owned(), e))?
        {
            // Destination doesn't exist, just link
            detail_pack.push(LinkDetailPack::Linkable {
                src_path: src,
                dest_path: dest,
                linked: None,
            });
        } else if dest.is_symlink() && dest.read_link().is_ok_and(|path| path == src) {
            // Destination is symlink and points to src, already linked
            detail_pack.push(LinkDetailPack::AlreadyLinked {
                src_path: src,
                dest_path: dest,
            });
        } else if dest.is_dir() && src.is_dir() {
            // Destination and Source both dir, recursive link
            detail_pack.append(&mut prob_link(&src, &dest)?);
        } else {
            // Destination exists but not dir, fail
            detail_pack.push(LinkDetailPack::DestOccupied {
                src_path: src,
                dest_path: dest,
                force_linked: None,
            });
        }
    }

    Ok(detail_pack)
}

/// Make a link for the files in the given path
fn make_link(packs: &mut Vec<LinkDetailPack>, arg: &LinkArg) -> eyre::Result<()> {
    for pack in packs {
        match pack {
            LinkDetailPack::Linkable {
                src_path,
                dest_path,
                linked,
            } => {
                trace!("Linking {:?}", src_path);
                *linked = Some(soft_link(src_path, dest_path));
            }
            LinkDetailPack::AlreadyLinked {
                src_path,
                dest_path,
            } => {
                trace!("Ignoring already linked {:?} -> {:?}", dest_path, src_path);
                continue;
            }
            LinkDetailPack::DestOccupied {
                src_path,
                dest_path,
                force_linked,
            } => {
                if arg.force {
                    if fs::exists(&dest_path).map_err(|e| {
                        LinkError::CannotConfirmFileExistence(dest_path.to_owned(), e)
                    })? {
                        trace!("Removing occupied destination {:?}", dest_path);
                        if dest_path.is_file() && fs::remove_file(&dest_path).is_err() {
                            warn!(
                                "Failed to remove file {:?}, skip linking {:?}",
                                dest_path, src_path
                            );
                            continue;
                        }
                        if dest_path.is_dir() && fs::remove_dir(&dest_path).is_err() {
                            warn!(
                                "Failed to remove directory {:?}, skip linking {:?}",
                                dest_path, src_path
                            );
                            continue;
                        }
                        trace!("Removed occupied destination {:?}", dest_path);
                    }
                    *force_linked = Some(soft_link(src_path, dest_path));
                } else {
                    warn!("Skipping occupied {:?} -> {:?}", dest_path, src_path);
                }
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
fn soft_link(src_path: &mut PathBuf, dest_path: &mut PathBuf) -> io::Result<()> {
    unix::fs::symlink(src_path, dest_path)
}

#[cfg(not(unix))]
fn soft_link(src_path: &mut PathBuf, dest_path: &mut PathBuf) -> io::Result<()> {
    todo!() // TODO:
}
