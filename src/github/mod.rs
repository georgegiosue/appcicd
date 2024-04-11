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

    use crate::utils::test::replicate_android_projects_to_temp;

    use super::*;

    #[test]
    fn test_exists_github_dotfiles_dir() {
        let (groovydsl_project_path, kotlindsl_project_path) = replicate_android_projects_to_temp();

        assert_eq!(exists_github_dotfiles_dir(&groovydsl_project_path), false);
        assert_eq!(exists_github_dotfiles_dir(&kotlindsl_project_path), false);

        let _ = std::fs::remove_dir_all(groovydsl_project_path);
        let _ = std::fs::remove_dir_all(kotlindsl_project_path);
    }

    #[test]
    fn test_create_github_dotfiles_dir() {
        let (groovydsl_project_path, kotlindsl_project_path) = replicate_android_projects_to_temp();

        fs::create_dir(&groovydsl_project_path.join(".github"))
            .expect("Error creating .github dir in groovy dsl project");

        assert_eq!(groovydsl_project_path.join(".github").exists(), true);

        fs::create_dir(&kotlindsl_project_path.join(".github"))
            .expect("Error creating .github dir in kotlin dsl project");

        assert_eq!(kotlindsl_project_path.join(".github").exists(), true);

        let _ = fs::remove_dir_all(groovydsl_project_path);
        let _ = fs::remove_dir_all(kotlindsl_project_path);
    }
}
