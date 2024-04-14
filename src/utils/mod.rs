use std::path::Path;

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

#[cfg(test)]
pub mod test {

    use std::{
        env, fs, io,
        path::{Path, PathBuf},
        time::UNIX_EPOCH,
    };

    use rand::Rng;

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

    pub fn replicate_android_projects_to_temp() -> (PathBuf, PathBuf) {
        let root_project_path = find_project_path().expect("Failed to find root project path");

        let groovydsl_path = root_project_path
            .join("tests")
            .join("android")
            .join("groovydsl");

        let kotlindsl_path = root_project_path
            .join("tests")
            .join("android")
            .join("kotlindsl");

        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error to get current timestamp")
            .as_millis()
            .to_string();

        let random: u32 = rand::thread_rng().gen();

        let timestamp = timestamp + "+" + random.to_string().as_str();

        let temp_dir = env::temp_dir().join(timestamp);

        fs::create_dir(&temp_dir).expect("Error creating temp dir");

        let groovy_dsl_temp_dir = temp_dir.join("groovydsl");
        let kotlin_dsl_temp_dir = temp_dir.join("kotlindsl");

        if groovydsl_path.exists() {
            copy_dir_all(groovydsl_path, &groovy_dsl_temp_dir)
                .expect("Failed to copy groovydsl project to temp");
        }
        if kotlindsl_path.exists() {
            copy_dir_all(kotlindsl_path, &kotlin_dsl_temp_dir)
                .expect("Failed to copy kotlindsl project to temp");
        }

        (groovy_dsl_temp_dir, kotlin_dsl_temp_dir)
    }
}
