pub mod build_src {
    use std::{
        fs::{self},
        path::Path,
    };

    use crate::{
        build::runtime::AndroidBuildRuntime, cmd::rollback, config::constants, utils::{git_ignore, out::print_out, unicode_messages::UMessage}
    };

    pub fn exists_build_src_dir(path: &Path) -> bool {
        Path::new(&path)
            .join(constants::BUILD_SRC_DIRNAME)
            .as_path()
            .exists()
    }

    pub fn create_build_src_module(project_path: &Path, build_runtime: &AndroidBuildRuntime) {
        let build_src_path = Path::new(&project_path).join(constants::BUILD_SRC_DIRNAME);

        let build_src_kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        create_hierarchy(&build_src_kotlin_path);

        let files_to_ignore = vec!["/build"];

        git_ignore(&build_src_path, files_to_ignore);

        copy_build_script(&build_runtime, &build_src_path);

        copy_kotlin_files(&project_path);

        print_out(UMessage::SUCCESS("buildSrc module created successfully"))
    }

    fn copy_build_script(build_runtime: &AndroidBuildRuntime, buid_src_path: &Path) {
        match build_runtime {
            AndroidBuildRuntime::GROOVY => {
                let script = include_bytes!("./../../assets/scripts/build.gradle");
                let script_destination_path = buid_src_path.join("build.gradle");

                if let Err(error) = std::fs::write(script_destination_path, script) {
                    panic!("Error copying build.gradle | {}", error)
                }
            }
            AndroidBuildRuntime::KTS => {
                let script = include_bytes!("./../../assets/scripts/build.gradle.kts");
                let script_destination_path = buid_src_path.join("build.gradle.kts");

                if let Err(error) = std::fs::write(script_destination_path, script) {
                    panic!("Error copying build.gradle.kts | {}", error)
                }
            }
        }
    }

    pub fn copy_kotlin_files(project_path: &Path) {
        let build_src_path = Path::new(&project_path).join(constants::BUILD_SRC_DIRNAME);

        let build_src_kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        let versioning_kt = include_bytes!("./../../assets/kotlin/Versioning.kt");
        let versioning_kt_destination_path = build_src_kotlin_path.join("Versioning.kt");

        if let Err(error) = fs::write(versioning_kt_destination_path, versioning_kt) {
            rollback::run(project_path);
            panic!("Could not create Versioning.kt file | {error}")
        }

        let utils_kt = include_bytes!("./../../assets/kotlin/VersioningUtils.kt");
        let utils_kt_destination_path = build_src_kotlin_path.join("VersioningUtils.kt");

        if let Err(error) = fs::write(utils_kt_destination_path, utils_kt) {
            rollback::run(project_path);
            panic!("Could not create VersioningUtils.kt file | {}", error);
        }
    }

    fn create_hierarchy(dir_path: &Path) {
        if let Err(error) = fs::create_dir_all(&dir_path) {
            rollback::run(
                dir_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap(),
            );
            panic!("Could not create buildSrc directory | {error}")
        }
    }
}

pub mod app {}
