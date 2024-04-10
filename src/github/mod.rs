use std::{fs, path::Path};

use crate::utils::{out::print_out, unicode_messages::UMessage};

pub mod deploy;

pub fn exists_github_dotfiles_dir(project_path: &Path) -> bool {
    project_path.join(".github").exists()
}

pub fn create_github_dotfiles_dir(project_path: &Path) {
    let github_dotfiles_path = project_path.join(".github/");

    match fs::create_dir(github_dotfiles_path.as_path()) {
        Ok(_) => {
            print_out(UMessage::SUCCESS("The .github dir has been created."));
        }
        Err(error) => panic!("Error creating .github directory | {}", error),
    };
}
