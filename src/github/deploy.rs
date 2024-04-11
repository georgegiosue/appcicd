use std::{
    fs::{self},
    path::Path,
};

use crate::cmd::rollback;

pub fn copy_workflows(project_path: &Path) {
    let github_dotfiles_path = project_path.join(".github/");

    let workflows: Vec<(&[u8], &str)> = vec![
        (
            include_bytes!("./../../assets/workflows/bump-version.yml"),
            "bump-version.yml",
        ),
        (
            include_bytes!("./../../assets/workflows/gradle-wrapper-validation.yml"),
            "gradle-wrapper-validation.yml",
        ),
        (
            include_bytes!("./../../assets/workflows/publish-release.yml"),
            "publish-release.yml",
        ),
        (
            include_bytes!("./../../assets/workflows/publish-snapshot.yml"),
            "publish-snapshot.yml",
        ),
        (
            include_bytes!("./../../assets/workflows/test.yml"),
            "test.yml",
        ),
    ];

    for workflow in workflows {
        let path = github_dotfiles_path.join(workflow.1);
        if let Err(error) = fs::write(path, workflow.0) {
            rollback::run(&project_path);
            panic!("Copy error {} workflow in .github | {}", workflow.1, error)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        github::create_github_dotfiles_dir, utils::test::replicate_android_projects_to_temp,
    };

    #[test]
    fn test_copy_workflows() {
        let (groovy_project_path, kotlin_project_path) = replicate_android_projects_to_temp();

        let workflows: [&str; 5] = [
            "bump-version.yml",
            "gradle-wrapper-validation.yml",
            "publish-release.yml",
            "publish-snapshot.yml",
            "test.yml",
        ];

        create_github_dotfiles_dir(&groovy_project_path);
        create_github_dotfiles_dir(&kotlin_project_path);

        let groovydsl_github_dotfiles_path = groovy_project_path.join(".github");
        let kotlindsl_github_dotfiles_path = kotlin_project_path.join(".github");

        copy_workflows(&groovy_project_path);
        copy_workflows(&kotlin_project_path);

        for workflow in workflows {
            let path = groovydsl_github_dotfiles_path.join(workflow);

            assert_eq!(path.exists(), true);
        }

        for workflow in workflows {
            let path = kotlindsl_github_dotfiles_path.join(workflow);

            assert_eq!(path.exists(), true);
        }

        let _ = std::fs::remove_dir_all(groovy_project_path);
        let _ = std::fs::remove_dir_all(kotlin_project_path);
    }
}
