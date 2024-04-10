use std::{
    fs::{self},
    path::Path,
};

use crate::utils::{out::print_out, unicode_messages::UMessage};

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
            panic!("Copy error {} workflow in .github | {}", workflow.1, error)
        }
    }

    print_out(UMessage::SUCCESS("Workflow's has been added"));
}
