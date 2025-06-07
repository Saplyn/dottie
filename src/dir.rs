use std::{
    env,
    fmt::{self, Display, Formatter},
    fs, io,
    path::PathBuf,
    sync::LazyLock,
};

use log::warn;
use thiserror::Error;

pub static HOME_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| env::home_dir().expect("Home directory is not available"));
pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::home_dir()
        .expect("Home directory is not available")
        .join(".dottie")
});
pub static FILES_POSTFIX: &str = "files";
pub static SCRIPTS_POSTFIX: &str = "scripts";

#[derive(Debug, Clone)]
pub enum Dir {
    App,
    Pkg { pkg_name: String },
    Files { pkg_name: String },
    Scripts { pkg_name: String },
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Dir::App => write!(f, "App directory `~/.dottie`"),
            Dir::Pkg { pkg_name } => write!(f, "Package Directory: {}", pkg_name),
            Dir::Files { pkg_name } => write!(f, "Files Directory for Package: {}", pkg_name),
            Dir::Scripts { pkg_name } => write!(f, "Scripts Directory for Package: {}", pkg_name),
        }
    }
}
pub fn get(dir: Dir) -> PathBuf {
    match dir {
        Dir::App => APP_DIR.clone(),
        Dir::Pkg { pkg_name } => APP_DIR.join(pkg_name),
        Dir::Files { pkg_name } => APP_DIR.join(pkg_name).join(FILES_POSTFIX),
        Dir::Scripts { pkg_name } => APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX),
    }
}

#[derive(Debug, Error)]
pub enum DirError {
    #[error("Cannot confirm existence of {0}: {1}")]
    CannotConfirmDirExistence(Dir, io::Error),
    #[error("Failed to create directory {0}: {1}")]
    FailedToCreateDir(Dir, io::Error),
}

pub fn exists(dir: Dir) -> eyre::Result<bool> {
    Ok(match dir {
        Dir::App => fs::exists(APP_DIR.as_path())
            .map_err(|e| DirError::CannotConfirmDirExistence(dir, e))?,
        Dir::Pkg { ref pkg_name } => fs::exists(APP_DIR.join(pkg_name))
            .map_err(|e| DirError::CannotConfirmDirExistence(dir, e))?,
        Dir::Files { ref pkg_name } => fs::exists(APP_DIR.join(pkg_name).join(FILES_POSTFIX))
            .map_err(|e| DirError::CannotConfirmDirExistence(dir, e))?,
        Dir::Scripts { ref pkg_name } => {
            fs::exists(APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX))
                .map_err(|e| DirError::CannotConfirmDirExistence(dir, e))?
        }
    })
}

pub fn ensure_exists(dir: Dir) -> eyre::Result<()> {
    match dir {
        Dir::App => {
            if !fs::exists(APP_DIR.as_path())
                .map_err(|e| DirError::CannotConfirmDirExistence(dir.clone(), e))?
            {
                warn!("App directory doesn't exist, creating...");
                fs::create_dir(APP_DIR.as_path())
                    .map_err(|e| DirError::FailedToCreateDir(dir.clone(), e))?;
            }
        }
        Dir::Pkg { ref pkg_name } => {
            let pkg_dir = APP_DIR.join(pkg_name);
            if !fs::exists(&pkg_dir)
                .map_err(|e| DirError::CannotConfirmDirExistence(dir.clone(), e))?
            {
                warn!("Package `{}` doesn't exist, creating...", pkg_name);
                fs::create_dir(&pkg_dir)
                    .map_err(|e| DirError::FailedToCreateDir(dir.clone(), e))?;
            }
        }
        Dir::Files { ref pkg_name } => {
            let files_dir = APP_DIR.join(pkg_name).join(FILES_POSTFIX);
            if !fs::exists(&files_dir)
                .map_err(|e| DirError::CannotConfirmDirExistence(dir.clone(), e))?
            {
                warn!(
                    "Files directory for package `{}` doesn't exist, creating...",
                    pkg_name
                );
                fs::create_dir(&files_dir)
                    .map_err(|e| DirError::FailedToCreateDir(dir.clone(), e))?;
            }
        }
        Dir::Scripts { ref pkg_name } => {
            let scripts_dir = APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX);
            if !fs::exists(&scripts_dir)
                .map_err(|e| DirError::CannotConfirmDirExistence(dir.clone(), e))?
            {
                warn!(
                    "Scripts directory for package `{}` doesn't exist, creating...",
                    pkg_name
                );
                fs::create_dir(&scripts_dir)
                    .map_err(|e| DirError::FailedToCreateDir(dir.clone(), e))?;
            }
        }
    }

    Ok(())
}
