mod cicd;
pub mod rollback;
pub mod schema;

use std::path::PathBuf;
use std::{env, path::Path, process::exit};

use clap::Parser;

use crate::build::runtime::{get_build_runtime, AndroidBuildRuntime};
use crate::cmd::schema::Command;
use crate::utils::out::set_verbose;

use self::schema::{AndroidCICD, AuthSubcommand, GlobalOpts};

pub fn make() {
    let cmd = AndroidCICD::parse();

    let gb_options: GlobalOpts = cmd.global_opts;

    let command = cmd.command;

    let current_path_buf: PathBuf = env::current_dir().unwrap();

    let project_path: &Path = match &gb_options.path {
        Some(target_path) => Path::new(target_path),
        None => current_path_buf.as_path(),
    };

    let verbose: bool = gb_options.verbose;

    set_verbose(verbose);

    match command {
        Command::SetUp => {
            let build_runtime: AndroidBuildRuntime = get_build_runtime(&project_path);

            cicd::run(&project_path, &build_runtime);
        }
        Command::Auth(subcommand) => match subcommand {
            AuthSubcommand::Login => (),
            AuthSubcommand::Logout => (),
        },
        Command::Rollback => {
            rollback::run(&project_path);
            exit(0);
        }
    }
}
