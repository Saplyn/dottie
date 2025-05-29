use std::{env, fs, path::PathBuf, sync::LazyLock};

use log::warn;

pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    env::home_dir()
        .expect("Home directory is not available")
        .join(".dottie")
});
pub static FILES_POSTFIX: &str = "files";
pub static SCRIPTS_POSTFIX: &str = "scripts";

pub enum Dir<'s> {
    App,
    Pkg { pkg_name: &'s str },
    Files { pkg_name: &'s str },
    Scripts { pkg_name: &'s str },
}

pub fn get(dir: Dir) -> PathBuf {
    match dir {
        Dir::App => APP_DIR.clone(),
        Dir::Pkg { pkg_name } => APP_DIR.join(pkg_name),
        Dir::Files { pkg_name } => APP_DIR.join(pkg_name).join(FILES_POSTFIX),
        Dir::Scripts { pkg_name } => APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX),
    }
}

pub fn exists(dir: Dir) -> eyre::Result<bool> {
    Ok(match dir {
        Dir::App => fs::exists(APP_DIR.as_path())?,
        Dir::Pkg { pkg_name } => fs::exists(APP_DIR.join(pkg_name))?,
        Dir::Files { pkg_name } => fs::exists(APP_DIR.join(pkg_name).join(FILES_POSTFIX))?,
        Dir::Scripts { pkg_name } => fs::exists(APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX))?,
    })
}

pub fn ensure_exists(dir: Dir) -> eyre::Result<()> {
    match dir {
        Dir::App => {
            if !fs::exists(APP_DIR.as_path())? {
                warn!("App directory doesn't exist, creating...");
                fs::create_dir(APP_DIR.as_path())?;
            }
        }
        Dir::Pkg { pkg_name } => {
            let pkg_dir = APP_DIR.join(pkg_name);
            if !fs::exists(&pkg_dir)? {
                warn!("Package `{}` doesn't exist, creating...", pkg_name);
                fs::create_dir(&pkg_dir)?;
            }
        }
        Dir::Files { pkg_name } => {
            let files_dir = APP_DIR.join(pkg_name).join(FILES_POSTFIX);
            if !fs::exists(&files_dir)? {
                warn!(
                    "Files directory for package `{}` doesn't exist, creating...",
                    pkg_name
                );
                fs::create_dir(&files_dir)?;
            }
        }
        Dir::Scripts { pkg_name } => {
            let scripts_dir = APP_DIR.join(pkg_name).join(SCRIPTS_POSTFIX);
            if !fs::exists(&scripts_dir)? {
                warn!(
                    "Scripts directory for package `{}` doesn't exist, creating...",
                    pkg_name
                );
                fs::create_dir(&scripts_dir)?;
            }
        }
    }

    Ok(())
}
