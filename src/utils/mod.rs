use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use rand::Rng;

use crate::build::runtime::AndroidBuildRuntime;

pub mod out;
pub mod unicode_messages;

pub fn git_ignore(path: &Path, files: Vec<&str>) {
    let mut ignore = String::new();
    for file in files {
        ignore.push_str(file);
        ignore.push_str("\n");
    }

    let gitignore_path = path.join(".gitignore");

    std::fs::write(gitignore_path, ignore).expect("Error writing .gitignore");
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

pub fn clean_temp(android_project_path: PathBuf) {
    fs::remove_dir_all(android_project_path.parent().unwrap())
        .expect("Error clean project dir from temp directory");
}

pub fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}