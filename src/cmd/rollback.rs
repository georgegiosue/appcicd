use std::{
    fs::{self},
    path::Path,
};

use crate::{
    crypto::exists_secrets_dir,
    github::exists_github_dotfiles_dir,
    source::module::build_src::exists_build_src_dir,
    utils::{check_android_project, out::print_out, unicode_messages::UMessage},
};

pub fn run(project_path: &Path) {

    check_android_project(project_path);

    // path: .github/

    if exists_github_dotfiles_dir(project_path) {
        let github_dotfiles_path = project_path.join(".github/");

        // Check if .github/ dir is empty (excluding .gitignore) then remove it from project

        let is_empty = match fs::read_dir(&github_dotfiles_path) {
            Ok(dir) => dir
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .filter(|file_name| file_name != ".gitignore")
                .next()
                .is_none(),
            Err(_) => false,
        };

        if is_empty {
            fs::remove_dir_all(&github_dotfiles_path).expect("Error removing buildSrc dir");
        } else {
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
            // Check again if .github/ dir is empty (excluding .gitignore) then remove it from project

            let is_empty = match fs::read_dir(&github_dotfiles_path) {
                Ok(dir) => dir
                    .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                    .filter(|file_name| file_name != ".gitignore")
                    .next()
                    .is_none(),
                Err(_) => false,
            };

            if is_empty {
                fs::remove_dir_all(&github_dotfiles_path).expect("Error removing .github dir");
            }
        }
    }

    // path: buildSrc/

    if exists_build_src_dir(project_path) {
        let build_src_path = project_path.join("buildSrc");
        let kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        // Unexpected behavior if another directory exists in buildSrc/src/main other than kotlin dir

        if kotlin_path.exists() {
            // Check if buildSrc/src/main/kotlin/ dir is empty (excluding .gitignore) then remove it from project

            let is_empty = match fs::read_dir(&kotlin_path) {
                Ok(dir) => dir
                    .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                    .filter(|file_name| file_name != ".gitignore")
                    .next()
                    .is_none(),
                Err(_) => false,
            };

            if is_empty {
                fs::remove_dir_all(&build_src_path).expect("Error removing buildSrc dir");
            } else {
                let kotlin_files = ["Versioning.kt", "VersioningUtils.kt"];

                for file in kotlin_files {
                    let path = kotlin_path.join(file);

                    if path.exists() {
                        std::fs::remove_file(path).expect("Error removing file");
                    }
                }

                // Check again if buildSrc/src/main/kotlin/ dir is empty (excluding .gitignore) then remove it from project

                let is_empty = match fs::read_dir(&kotlin_path) {
                    Ok(dir) => dir
                        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                        .filter(|file_name| file_name != ".gitignore")
                        .next()
                        .is_none(),
                    Err(_) => false,
                };

                if is_empty {
                    fs::remove_dir_all(&build_src_path).expect("Error removing buildSrc dir");
                }
            }
        }
    }

    // path: secrets/

    if exists_secrets_dir(project_path) {
        let secrets_path = project_path.join("secrets");

        // Check if secrets/ dir is empty (excluding .gitignore) then remove secret path dir from project

        let is_empty = match fs::read_dir(&secrets_path) {
            Ok(dir) => dir
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .filter(|file_name| file_name != ".gitignore")
                .next()
                .is_none(),
            Err(_) => false,
        };

        if is_empty {
            std::fs::remove_dir_all(&secrets_path).expect("Error removing directory");
        } else {
            let files: Vec<&str> = vec![
                "release-keystore.jks",
                "debug-keystore.jks",
                "release-keystore.jks.aes",
                "debug-keystore.jks.aes",
            ];

            for file in files {
                let file_path = secrets_path.join(&file);

                if file_path.exists() {
                    std::fs::remove_file(file_path).expect("Error removing file");
                }
            }

            // Check again if secrets/ dir is empty (excluding .gitignore) then remove it from project

            let is_empty = match fs::read_dir(&secrets_path) {
                Ok(dir) => dir
                    .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                    .filter(|file_name| file_name != ".gitignore")
                    .next()
                    .is_none(),
                Err(_) => false,
            };

            if is_empty {
                std::fs::remove_dir_all(&secrets_path).expect("Error removing secrets dir");
            }
        }
    }

    print_out(UMessage::ROLLBACK("Rollback completed!"));
}
