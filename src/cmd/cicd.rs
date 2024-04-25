use std::path::Path;

use crate::{
    build::runtime::AndroidBuildRuntime,
    config::constants,
    crypto::{
        create_secrets_dir, exists_secrets_dir,
        keystore::{create_debug_keystore, create_release_keystore, encrypt_keystore, KeyStore},
    },
    github::{create_github_dotfiles_dir, deploy::copy_workflows, exists_github_dotfiles_dir},
    platform::{chocolatey_installed, openssl_installed},
    source::module::build_src::{copy_kotlin_files, create_build_src_module, exists_build_src_dir},
    utils::{
        input,
        out::{panic_out, print_out},
        unicode_messages::UMessage,
    },
    verbose_println,
};

pub fn run(project_path: &Path, build_runtime: &AndroidBuildRuntime) {
    if *build_runtime == AndroidBuildRuntime::GROOVY {
        panic_out(UMessage::WARNING("Unsupported Groovy DSL"))
    }

    if !exists_build_src_dir(&project_path) {
        create_build_src_module(&project_path);
    } else {
        copy_kotlin_files(&project_path).expect("Error creating kotlin files for versioning");
        print_out(UMessage::SUCCESS(
            "buildSrc module is present and prepared for versioning",
        ));
    }

    if !exists_github_dotfiles_dir(&project_path) {
        create_github_dotfiles_dir(&project_path)
    } else {
        verbose_println!("{}", UMessage::SUCCESS(".github directory is present"));
    }

    copy_workflows(&project_path);

    print_out(UMessage::SUCCESS("Workflow's has been added"));

    if !exists_secrets_dir(&project_path) {
        create_secrets_dir(&project_path);
    } else {
        verbose_println!("{}", UMessage::SUCCESS("secrets directory is present"));
    }

    let secrets_dir = project_path.join("secrets");

    let mut debug_keystore: KeyStore = create_debug_keystore(&secrets_dir);

    let mut release_keystore: KeyStore = create_release_keystore(&secrets_dir);

    match constants::OS {
        "windows" => {
            if !chocolatey_installed() {
                panic_out(UMessage::WARNING("Chocolatey is not installed"));
            }

            if !openssl_installed() {
                panic_out(UMessage::WARNING("OpenSSL is not installed"));
            }

            print_out(UMessage::PWD(
                "Please enter your key for encrypt Debug KeyStore",
            ));

            let debug_key = input();

            encrypt_keystore(&mut debug_keystore, debug_key);

            print_out(UMessage::PWD(
                "Please enter your key for encrypt Release KeyStore",
            ));

            let release_key = input();

            encrypt_keystore(&mut release_keystore, release_key);

            print_out(UMessage::SUCCESS("Keystores have been encrypted"));
        }
        "linux" | "mascos" => {
            print_out(UMessage::PWD(
                "Please enter your key for encrypt Debug KeyStore",
            ));

            let debug_key = input();

            encrypt_keystore(&mut debug_keystore, debug_key);

            print_out(UMessage::PWD(
                "Please enter your key for encrypt Release KeyStore",
            ));

            let release_key = input();

            encrypt_keystore(&mut release_keystore, release_key);

            print_out(UMessage::SUCCESS("Keystores have been encrypted"));
        }
        _ => {
            panic_out(UMessage::ERROR("Unsupported OS"));
        }
    }
    //TODO: Binging config chucks to app/build.gradle.kts
    //TODO: Github Auth
    //TODO: Secrets upload throught Github REST API
    print_out(UMessage::DEPLOY("Ready. Confirm and push to GitHub."))
}
