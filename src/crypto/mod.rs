pub mod keystore;

use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

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

pub fn encrypt(file_path: &Path, key: &str) -> Result<PathBuf, Error> {
    let file_dir = file_path.parent().unwrap();
    let file_in = file_path.file_name().unwrap().to_str().unwrap();
    let file_out = file_in.to_owned() + ".aes";
    let file_out = file_out.as_str();

    let output = Command::new("openssl")
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
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(file_dir.join(file_out))
            } else {
                Err(Error::new(ErrorKind::Other, "Failed to encrypt file"))
            }
        }
        Err(error) => Err(error),
    }
}

pub fn decrypt(file_path: &Path, key: &str) -> Result<PathBuf, Error> {
    let file_dir = file_path.parent().unwrap();
    let file_in = file_path.file_name().unwrap().to_str().unwrap();
    let file_out = &file_in[..file_in.len() - 4];

    let output = Command::new("openssl")
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
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(file_dir.join(file_out))
            } else {
                Err(Error::new(ErrorKind::Other, "Failed to decrypt file"))
            }
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const FILE_NAME: &str = "text_file.txt";
    const FILE_CONTENT: &str = "Hello, World!";
    const KEY: &str = "mysecretkey";

    #[test]
    fn test_encrypt_file() {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join(FILE_NAME);

        std::fs::write(&file_path, FILE_CONTENT).expect("Failed to write file");

        let encrypted_file_path = encrypt(&file_path, KEY).expect("Failed to encrypt file");

        assert_eq!(encrypted_file_path.exists(), true)
    }

    #[test]
    fn test_decrypt_file() {
        let temp_dir = env::temp_dir();

        let file_path = temp_dir.join(FILE_NAME);

        // Encrypt test file again

        std::fs::write(&file_path, FILE_CONTENT).expect("Failed to write file");

        let file_encrypt_path = encrypt(&file_path, KEY).expect("Failed to encrypt file");

        let file_decrypt_path = decrypt(&file_encrypt_path, KEY).expect("Failed to decrypt file");

        let file_content =
            std::fs::read_to_string(&file_decrypt_path).expect("Failed to read file");

        assert_eq!(file_content, FILE_CONTENT)
    }
}