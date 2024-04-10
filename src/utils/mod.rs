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
