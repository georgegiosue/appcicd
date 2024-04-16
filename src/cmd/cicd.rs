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
        out::{panic_out, print_out},
        unicode_messages::UMessage,
    },
};

pub fn run(project_path: &Path, build_runtime: &AndroidBuildRuntime) {

    if *build_runtime == AndroidBuildRuntime::GROOVY {
        panic_out(UMessage::WARNING("Unsupported Groovy DSL"))
    }

    if !exists_build_src_dir(&project_path) {
        create_build_src_module(&project_path);
    } else {
        copy_kotlin_files(&project_path);
        print_out(UMessage::SUCCESS(
            "buildSrc module is present and prepared for versioning",
        ));
    }

    if !exists_github_dotfiles_dir(&project_path) {
        create_github_dotfiles_dir(&project_path)
    } else {
        print_out(UMessage::SUCCESS(".github directory is present"));
    }

    copy_workflows(&project_path);

    print_out(UMessage::SUCCESS("Workflow's has been added"));

    if !exists_secrets_dir(&project_path) {
        create_secrets_dir(&project_path);
    } else {
        print_out(UMessage::SUCCESS("secrets directory is present"));
    }

    let secrets_dir = project_path.join("secrets");

    let debug_keystore: KeyStore = create_debug_keystore(&secrets_dir);

    let release_keystore: KeyStore = create_release_keystore(&secrets_dir);

    match constants::OS {
        "windows" => {
            if !chocolatey_installed() {
                panic_out(UMessage::WARNING("Chocolatey is not installed"));
            }

            if !openssl_installed() {
                panic_out(UMessage::WARNING("OpenSSL is not installed"));
            }

            encrypt_keystore(&debug_keystore);
            encrypt_keystore(&release_keystore);

            print_out(UMessage::SUCCESS("Keystore has been encrypted"));
        }
        "linux" | "mascos" => {
            encrypt_keystore(&debug_keystore);
            encrypt_keystore(&release_keystore);

            print_out(UMessage::SUCCESS("Keystore has been encrypted"));
        }
        _ => {
            panic_out(UMessage::ERROR("Unsupported OS"));
        }
    }

    print_out(UMessage::DEPLOY("Ready. Confirm and push to GitHub."))
}
