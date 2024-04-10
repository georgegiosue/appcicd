mod cicd;
pub mod rollback;
pub mod schema;

use std::{env, path::Path, process::exit};

use clap::Parser;

use crate::build::runtime::{get_build_runtime, AndroidBuildRuntime};

use self::schema::Args;

pub fn make() {
    let args = Args::parse();

    let current_path_buf = env::current_dir().unwrap();

    let project_path: &Path = match &args.path {
        Some(target_path) => Path::new(target_path),
        None => current_path_buf.as_path(),
    };

    let build_runtime: AndroidBuildRuntime = get_build_runtime(&project_path);

    if args.rollback {
        rollback::run(&project_path);
        exit(0);
    }

    cicd::run(&project_path, &build_runtime);
}
