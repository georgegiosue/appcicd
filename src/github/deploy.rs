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
        build::runtime::AndroidBuildRuntime, github::create_github_dotfiles_dir,
        utils::{clean_temp, replicate_android_project_to_temp},
    };

    #[test]
    fn test_copy_workflows() {
        let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

        let workflows: [&str; 5] = [
            "bump-version.yml",
            "gradle-wrapper-validation.yml",
            "publish-release.yml",
            "publish-snapshot.yml",
            "test.yml",
        ];

        create_github_dotfiles_dir(&kotlin_project_path);

        let kotlindsl_github_dotfiles_path = kotlin_project_path.join(".github");

        copy_workflows(&kotlin_project_path);

        for workflow in workflows {
            let path = kotlindsl_github_dotfiles_path.join(workflow);

            assert_eq!(path.exists(), true);
        }

        clean_temp(kotlin_project_path);
    }
}
