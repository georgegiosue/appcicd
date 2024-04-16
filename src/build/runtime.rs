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
            AndroidBuildRuntime::GROOVY => write!(f, "groovydsl"),
            AndroidBuildRuntime::KTS => write!(f, "kotlindsl"),
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

    use super::*;

    use crate::utils::replicate_android_project_to_temp;

    #[test]
    fn test_build_runtime() {
        let groovy_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::GROOVY);
        let kotlin_project_path = replicate_android_project_to_temp(AndroidBuildRuntime::KTS);

        assert_eq!(
            get_build_runtime(&groovy_project_path),
            AndroidBuildRuntime::GROOVY
        );

        assert_eq!(
            get_build_runtime(&kotlin_project_path),
            AndroidBuildRuntime::KTS
        );

        let _ = std::fs::remove_dir_all(groovy_project_path);
        let _ = std::fs::remove_dir_all(kotlin_project_path);
    }
}
