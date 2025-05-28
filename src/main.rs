use std::{env, path::PathBuf, sync::LazyLock};

use clap::Parser;
use cli::{CliArg, info};

mod cli;

struct AppDirs {
    main: PathBuf,
    files: PathBuf,
    scripts: PathBuf,
}

static APP_DIRS: LazyLock<AppDirs> = LazyLock::new(|| {
    let main = env::home_dir()
        .expect("Home directory is not available")
        .join(".dottie");

    AppDirs {
        main: main.clone(),
        files: main.join("scripts"),
        scripts: main.join("files"),
    }
});

fn main() {
    let arg = CliArg::parse();
    match arg {
        CliArg::Run(arg) => todo!(),
        CliArg::Link(arg) => todo!(),
        CliArg::Info => info::main(),
    }
}
