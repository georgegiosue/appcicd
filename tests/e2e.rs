use std::process::Command;

use utils::{clean_temp, find_project_path};

use crate::utils::{replicate_android_project_to_temp, AndroidBuildRuntime};

#[test]
fn test_e2e_rollback() {
    let kotlin_project_path =
        replicate_android_project_to_temp(AndroidBuildRuntime::KTS { cicd: true });

    let project_path = find_project_path().expect("Project path wasn't found");

    let cargo_manifest = project_path.join("Cargo.toml");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(&cargo_manifest)
        .arg("--")
        .arg("rollback")
        .current_dir(&kotlin_project_path)
        .output()
        .expect("Error executing rollback");

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    assert_eq!(stdout_str.trim(), "↩️ Rollback completed!");

    clean_temp(kotlin_project_path);
}

mod utils {
    use std::{
        env, fmt, fs, io,
        path::{Path, PathBuf},
        time::UNIX_EPOCH,
    };

    use rand::Rng;

    #[derive(PartialEq, Debug)]
    pub enum AndroidBuildRuntime {
        KTS { cicd: bool },
    }

    impl fmt::Display for AndroidBuildRuntime {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AndroidBuildRuntime::KTS { cicd: false } => write!(f, "kotlindsl"),
                AndroidBuildRuntime::KTS { cicd: true } => write!(f, "kotlindsl_cicd"),
            }
        }
    }

    pub fn find_project_path() -> Option<PathBuf> {
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

    pub fn replicate_android_project_to_temp(build_runtime: AndroidBuildRuntime) -> PathBuf {
        let root_project_path = find_project_path().expect("Failed to find root project path");

        let android_project_path = root_project_path
            .join("tests")
            .join("android")
            .join(build_runtime.to_string());

        let dir_name = gen_random_dir_name("test");

        let temp_dir = env::temp_dir().join(dir_name);

        fs::create_dir(&temp_dir).expect("Error creating temp dir");

        let android_project_temp_dir = temp_dir.join(build_runtime.to_string());

        if android_project_path.exists() {
            copy_dir_all(&android_project_path, &android_project_temp_dir)
                .expect("Failed to copy android project to temp");
        }

        android_project_temp_dir
    }

    pub fn clean_temp(android_project_path: PathBuf) {
        fs::remove_dir_all(android_project_path.parent().unwrap())
            .expect("Error clean project dir from temp directory");
    }

    pub fn gen_random_dir_name(prefix: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error to get current timestamp")
            .as_millis()
            .to_string();

        let random: u32 = rand::thread_rng().gen();

        let dir_name = format!("{}_{}_{}", prefix, timestamp, random.to_string());

        dir_name
    }
}
