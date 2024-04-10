pub mod keystore;

use std::{fs, path::Path, process::Command};

use crate::utils::{git_ignore, out::print_out, unicode_messages::UMessage};

pub fn create_secrets_dir(project_path: &Path) {
    let secrets_dir = project_path.join("secrets");

    match fs::create_dir(secrets_dir.as_path()) {
        Ok(_) => print_out(UMessage::SUCCESS("Secrets dir has been created")),
        Err(error) => panic!("Error creating Secrets dir | {}", error),
    }

    let files_to_ignore = vec!["*.jks", "*.json"];

    git_ignore(&secrets_dir, files_to_ignore);
}

pub fn exists_secrets_dir(project_path: &Path) -> bool {
    let secrets_dir = project_path.join("secrets");

    secrets_dir.exists()
}

pub fn encrypt(file_path: &Path, key: &str) {
    let file_dir = file_path.parent().unwrap();
    let file_in = file_path.file_name().unwrap().to_str().unwrap();
    let file_out = file_in.to_owned() + ".aes";
    let file_out = file_out.as_str();

    Command::new("openssl")
        .args(&[
            "enc",
            "-aes-256-cbc",
            "-md",
            "sha512",
            "-pbkdf2",
            "-iter",
            "100000",
            "-salt",
            "-in",
            file_in,
            "-out",
            file_out,
            "-k",
            key,
        ])
        .current_dir(file_dir)
        .output()
        .expect("Failed to encrypt file");
}

pub fn decrypt(file_path: &Path, key: &str) {
    let file_dir = file_path.parent().unwrap();
    let file_in = file_path.file_name().unwrap().to_str().unwrap();
    let file_out = &file_in[..file_in.len() - 4];

    Command::new("openssl")
        .args(&[
            "enc",
            "-d",
            "-aes-256-cbc",
            "-md",
            "sha512",
            "-pbkdf2",
            "-iter",
            "100000",
            "-salt",
            "-in",
            file_in,
            "-out",
            file_out,
            "-k",
            key,
        ])
        .current_dir(file_dir)
        .output()
        .expect("Failed to decrypt file");
}
