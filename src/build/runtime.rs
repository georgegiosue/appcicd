use std::{fmt, path::Path};

use crate::utils::{out::panic_out, unicode_messages::UMessage};

#[derive(PartialEq, Debug)]
pub enum AndroidBuildRuntime {
    GROOVY,
    KTS,
}

pub enum BuildType {
    DEBUG,
    RELEASE,
}

impl fmt::Display for AndroidBuildRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AndroidBuildRuntime::GROOVY => write!(f, "Groovy DSL"),
            AndroidBuildRuntime::KTS => write!(f, "Kotlin DSL"),
        }
    }
}

pub fn get_build_runtime(path: &Path) -> AndroidBuildRuntime {
    let android_files = ["gradlew", "gradle.properties"];

    let all_files_present = android_files
        .iter()
        .map(|file_name| Path::new(&path).join(file_name).exists())
        .all(|file_present| file_present);

    if !all_files_present {
        panic_out(UMessage::ERROR("The folder no contains a Android Project"));
    }

    let kts_files = ["build.gradle.kts", "settings.gradle.kts"];

    let build_runtime = if kts_files
        .iter()
        .map(|kts_file| Path::new(&path).join(kts_file).exists())
        .all(|kts_file_present| kts_file_present)
    {
        AndroidBuildRuntime::KTS
    } else {
        AndroidBuildRuntime::GROOVY
    };

    build_runtime
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs, io,
        path::{Path, PathBuf},
    };

    use super::*;

    #[test]
    fn test_build_runtime() {
        replicate_android_projects_to_temp();

        let groovy_project_path = env::temp_dir().join("groovydsl");
        let kotlin_project_path = env::temp_dir().join("kotlindsl");

        assert_eq!(
            get_build_runtime(&groovy_project_path),
            AndroidBuildRuntime::GROOVY
        );

        assert_eq!(
            get_build_runtime(&kotlin_project_path),
            AndroidBuildRuntime::KTS
        );
    }

    fn replicate_android_projects_to_temp() {
        let root_project_path = find_project_path().expect("Failed to find root project path");

        let groovydsl_path = root_project_path
            .join("test")
            .join("android")
            .join("groovydsl");

        let kotlindsl_path = root_project_path
            .join("test")
            .join("android")
            .join("kotlindsl");

        let temp_dir = env::temp_dir();

        if groovydsl_path.exists() {
            copy_dir_all(groovydsl_path, temp_dir.join("groovydsl"))
                .expect("Failed to copy groovydsl project to temp");
        }
        if kotlindsl_path.exists() {
            copy_dir_all(kotlindsl_path, temp_dir.join("kotlindsl"))
                .expect("Failed to copy kotlindsl project to temp");
        }
    }

    fn find_project_path() -> Option<PathBuf> {
        let mut current_dir = env::current_dir().ok()?;

        loop {
            if current_dir.join("Cargo.toml").exists() {
                return Some(current_dir);
            }

            if !current_dir.pop() {
                return None;
            }
        }
    }

    // Thanks Simon Buchan and zacoons | related to https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
