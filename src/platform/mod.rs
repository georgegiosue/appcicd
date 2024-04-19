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

#[cfg(test)]
#[cfg(target_os = "windows")]
mod test {
    use super::*;

    #[test]
    fn test_chocolatey_installed() {
        let chocolatey_is_installed = chocolatey_installed();

        assert_eq!(chocolatey_is_installed, true);
    }

    #[test]
    fn test_openssl_installed() {
        let openssl_is_installed = openssl_installed();

        assert_eq!(openssl_is_installed, true);
    }
}
