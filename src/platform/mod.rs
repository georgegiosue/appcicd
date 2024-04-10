use std::process::Command;

pub fn chocolatey_installed() -> bool {
    match Command::new("choco").arg("-v").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn openssl_installed() -> bool {
    match Command::new("openssl").arg("version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
