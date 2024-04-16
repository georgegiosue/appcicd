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

#[cfg(test)]
mod test {

    use crate::{build::runtime::AndroidBuildRuntime, utils::replicate_android_project_to_temp};

    use super::*;

    #[test]
    fn test_exists_github_dotfiles_dir() {
        let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

        assert_eq!(exists_github_dotfiles_dir(&kotlin_project_path), false);

        let _ = std::fs::remove_dir_all(kotlin_project_path);
    }

    #[test]
    fn test_create_github_dotfiles_dir() {
        let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

        fs::create_dir(&kotlin_project_path.join(".github"))
            .expect("Error creating .github dir in kotlin dsl project");

        assert_eq!(kotlin_project_path.join(".github").exists(), true);

        let _ = fs::remove_dir_all(kotlin_project_path);
    }
}
