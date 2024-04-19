pub mod build_src {
    use std::{fs, io::Error, path::Path};

    use crate::{
        cmd::rollback,
        config::constants,
        utils::{git_ignore, out::print_out, unicode_messages::UMessage},
    };

    pub fn exists_build_src_dir(path: &Path) -> bool {
        Path::new(&path)
            .join(constants::BUILD_SRC_DIRNAME)
            .as_path()
            .exists()
    }

    pub fn create_build_src_module(project_path: &Path) {
        let build_src_path = Path::new(&project_path).join(constants::BUILD_SRC_DIRNAME);

        let build_src_kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        create_hierarchy(&build_src_kotlin_path).expect("Could not create buildSrc directory");

        let files_to_ignore = vec!["/build"];

        git_ignore(&build_src_path, files_to_ignore);

        copy_build_script(&build_src_path).expect("Error copying build.gradle.kts");

        copy_kotlin_files(&project_path).expect("Error creating kotlin files for versioning");

        print_out(UMessage::SUCCESS("buildSrc module created successfully"))
    }

    fn copy_build_script(buid_src_path: &Path) -> Result<(), Error> {
        let script = include_bytes!("./../../assets/scripts/build.gradle.kts");
        let script_destination_path = buid_src_path.join("build.gradle.kts");

        match std::fs::write(script_destination_path, script) {
            Ok(_) => Ok(()),
            Err(error) => {
                rollback::run(buid_src_path.parent().unwrap());

                Err(error)
            }
        }
    }

    pub fn copy_kotlin_files(project_path: &Path) -> Result<(), Error> {
        let build_src_path = Path::new(&project_path).join(constants::BUILD_SRC_DIRNAME);

        let build_src_kotlin_path = build_src_path.join("src").join("main").join("kotlin");

        let versioning_kt = include_bytes!("./../../assets/kotlin/Versioning.kt");
        let versioning_kt_destination_path = build_src_kotlin_path.join("Versioning.kt");

        if let Err(error) = fs::write(versioning_kt_destination_path, versioning_kt) {
            rollback::run(project_path);

            let error_message = format!("{}: {}", "Could not create Versioning.kt", error);

            return Err(Error::new(std::io::ErrorKind::Other, error_message));
        }

        let utils_kt = include_bytes!("./../../assets/kotlin/VersioningUtils.kt");
        let utils_kt_destination_path = build_src_kotlin_path.join("VersioningUtils.kt");

        if let Err(error) = fs::write(utils_kt_destination_path, utils_kt) {
            rollback::run(project_path);

            let error_message =
                format!("{}: {}", "Could not create VersioningUtils.kt file", error);

            return Err(Error::new(std::io::ErrorKind::Other, error_message));
        }

        Ok(())
    }

    fn create_hierarchy(dir_path: &Path) -> Result<(), Error> {
        match fs::create_dir_all(&dir_path) {
            Ok(_) => Ok(()),
            Err(error) => {
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

                Err(error)
            }
        }
    }

    #[cfg(test)]
    mod test {

        use crate::{
            build::runtime::AndroidBuildRuntime,
            utils::test::{clean_temp, replicate_android_project_to_temp},
        };

        use super::*;

        #[test]
        fn test_create_hierarchy() {
            let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

            let build_src_path = kotlin_project_path.join("buildSrc");

            let build_src_kotlin_path = build_src_path.join("src").join("main").join("kotlin");

            create_hierarchy(&build_src_kotlin_path).unwrap();

            assert_eq!(build_src_kotlin_path.exists(), true);

            clean_temp(kotlin_project_path);
        }

        #[test]
        fn test_copy_kotlin_files() {
            let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

            let kotlin_files = vec!["Versioning.kt", "VersioningUtils.kt"];

            let build_src_kotlin_path = kotlin_project_path
                .join("buildSrc")
                .join("src")
                .join("main")
                .join("kotlin");

            create_hierarchy(&build_src_kotlin_path).unwrap();

            copy_kotlin_files(&kotlin_project_path).unwrap();

            let read_dir = fs::read_dir(build_src_kotlin_path).unwrap();

            let kotlin_files_present: bool = read_dir
                .map(|asd| asd.unwrap().file_name().to_string_lossy().to_string())
                .all(|file_name| kotlin_files.contains(&file_name.as_str()));

            assert_eq!(kotlin_files_present, true);

            clean_temp(kotlin_project_path);
        }
    }
}

pub mod app {}
