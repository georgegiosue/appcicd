use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{
    build::runtime::BuildType,
    cmd::rollback,
    utils::{
        input,
        out::{panic_out, print_out},
        unicode_messages::UMessage,
    },
};

use super::encrypt;

pub struct KeyStore {
    pub path: PathBuf,
    pub build_type: BuildType,
    pub pwd: String,
}

impl KeyStore {
    pub fn new(path: PathBuf, build_type: BuildType, pwd: String) -> Self {
        KeyStore {
            path,
            build_type,
            pwd,
        }
    }
}

pub fn create_debug_keystore(secrets_path: &Path) -> KeyStore {
    let debug_keystore_path = secrets_path.join("debug-keystore.jks");

    if debug_keystore_path.exists() {
        print_out(UMessage::INFO("Debug keystore already exists"));

        let mut buffer = String::new();

        print_out(UMessage::PWD(
            "Please enter the password for the debug keystore",
        ));

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading password");

        let pwd = buffer.trim().to_string();

        let debug_keystore = KeyStore::new(debug_keystore_path, BuildType::DEBUG, pwd);

        return debug_keystore;
    }

    let java_home = match std::env::var("JAVA_HOME") {
        Ok(java_home_env) => java_home_env,
        Err(error) => {
            rollback::run(secrets_path.parent().unwrap());
            panic!(
                "JAVA_HOME not found. Make sure you have installed JDK | {}",
                error
            );
        }
    };

    let keytool_path = Path::new(&java_home).join("bin").join("keytool");

    print_out(UMessage::PWD(
        "Please enter the password for the debug keystore",
    ));

    let pwd = input();

    print_out(UMessage::INPUT("Please enter your First and Last name"));

    let first_and_last_name = input();

    print_out(UMessage::INPUT("Please enter your organizational unit"));

    let organizational_unit = input();

    print_out(UMessage::INPUT("Please enter your organization"));

    let organization = input();

    print_out(UMessage::INPUT("Please enter your city or locality"));

    let city = input();

    print_out(UMessage::INPUT("Please enter your state or province"));

    let state = input();

    print_out(UMessage::INPUT("Please enter your two-letter country code"));

    let country = input();

    let args = [
        "-genkey",
        "-keystore",
        "debug-keystore.jks",
        "-storepass",
        &pwd,
        "-alias",
        "debug",
        "-keypass",
        &pwd,
        "-keyalg",
        "RSA",
        "-keysize",
        "2048",
        "-validity",
        "10000",
        "-dname",
        &format!(
            "CN={}, OU={}, O={}, L={}, ST={}, C={}",
            first_and_last_name, organizational_unit, organization, city, state, country
        ),
    ];

    let cmd = Command::new(keytool_path)
        .args(args)
        .current_dir(&secrets_path)
        .stdout(Stdio::null())
        .status()
        .expect("Error running keytool");

    if !cmd.success() {
        rollback::run(secrets_path.parent().unwrap());
        panic_out(UMessage::ERROR("Error creating debug keystore"));
    }

    print_out(UMessage::SUCCESS("Debug keystore created successfully"));

    let debug_keystore = KeyStore::new(debug_keystore_path, BuildType::DEBUG, pwd.to_string());

    debug_keystore
}

pub fn create_release_keystore(secrets_path: &Path) -> KeyStore {
    let release_keystore_path = secrets_path.join("release-keystore.jks");

    if release_keystore_path.exists() {
        print_out(UMessage::INFO("Release keystore already exists"));

        let mut buffer = String::new();

        print_out(UMessage::PWD(
            "Please enter the password for the release keystore",
        ));

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading password");

        let pwd = buffer.trim().to_string();

        let release_keystore = KeyStore::new(release_keystore_path, BuildType::RELEASE, pwd);

        return release_keystore;
    }

    let java_home = match std::env::var("JAVA_HOME") {
        Ok(java_home_env) => java_home_env,
        Err(error) => {
            rollback::run(secrets_path.parent().unwrap());
            panic!("JAVA_HOME not found | {}", error);
        }
    };

    let keytool_path = Path::new(&java_home).join("bin").join("keytool");

    print_out(UMessage::PWD(
        "Please enter the password for the release keystore",
    ));

    let pwd = input();

    print_out(UMessage::INPUT("Please enter your First and Last name"));

    let first_and_last_name = input();

    print_out(UMessage::INPUT("Please enter your organizational unit"));

    let organizational_unit = input();

    print_out(UMessage::INPUT("Please enter your organization"));

    let organization = input();

    print_out(UMessage::INPUT("Please enter your city or locality"));

    let city = input();

    print_out(UMessage::INPUT("Please enter your state or province"));

    let state = input();

    print_out(UMessage::INPUT("Please enter your two-letter country code"));

    let country = input();

    let args = [
        "-genkey",
        "-keystore",
        "release-keystore.jks",
        "-storepass",
        &pwd,
        "-alias",
        "release",
        "-keypass",
        &pwd,
        "-keyalg",
        "RSA",
        "-keysize",
        "2048",
        "-validity",
        "10000",
        "-dname",
        &format!(
            "CN={}, OU={}, O={}, L={}, ST={}, C={}",
            first_and_last_name, organizational_unit, organization, city, state, country
        ),
    ];

    let cmd = Command::new(keytool_path)
        .args(args)
        .current_dir(&secrets_path)
        .stdout(Stdio::null())
        .status()
        .expect("Error running keytool");

    if !cmd.success() {
        rollback::run(secrets_path.parent().unwrap());
        panic_out(UMessage::ERROR("Error creating release keystore"));
    }

    print_out(UMessage::SUCCESS("Release keystore created successfully"));

    let release_keystore = KeyStore::new(release_keystore_path, BuildType::DEBUG, pwd.to_string());

    release_keystore
}

pub fn encrypt_keystore(keystore: &KeyStore) -> PathBuf {
    let keystore_path = keystore.path.as_path();
    let keystore_pwd = keystore.pwd.as_str();

    let keystore_encrypt_path =
        encrypt(keystore_path, keystore_pwd).expect("Error encrypting keystore");

    keystore_encrypt_path
}
