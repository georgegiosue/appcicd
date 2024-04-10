use std::path::Path;

use crate::{
    crypto::exists_secrets_dir,
    github::exists_github_dotfiles_dir,
    source::module::build_src::exists_build_src_dir,
    utils::{out::print_out, unicode_messages::UMessage},
};

pub fn run(project_path: &Path) {
    if exists_github_dotfiles_dir(project_path) {
        let github_dotfiles_path = project_path.join(".github/");

        let workflows: [&str; 5] = [
            "bump-version.yml",
            "gradle-wrapper-validation.yml",
            "publish-release.yml",
            "publish-snapshot.yml",
            "test.yml",
        ];

        for workflow in workflows {
            let path = github_dotfiles_path.join(workflow);

            if path.exists() {
                std::fs::remove_file(path).expect("Error removing file");
            }
        }
    }

    if exists_build_src_dir(project_path) {
        let build_src_path = project_path.join("buildSrc");
        let kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        let kotlin_files = ["Versioning.kt", "VersioningUtils.kt"];

        if kotlin_path.exists() {
            for file in kotlin_files {
                let path = kotlin_path.join(file);

                if path.exists() {
                    std::fs::remove_file(path).expect("Error removing file");
                }
            }
        }
    }

    if exists_secrets_dir(project_path) {
        let secrets_path = project_path.join("secrets");

        if secrets_path.exists() {
            std::fs::remove_dir_all(secrets_path).expect("Error removing directory");
        }
    }

    print_out(UMessage::ROLLBACK("Rollback completed!"));
}
