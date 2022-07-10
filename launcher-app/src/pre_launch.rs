use crate::app_config::PreLaunchSettings;
use std::process::Command;

pub(crate) fn handle_env_setup(config: PreLaunchSettings) {
    if !config.extra_packages.is_empty() {
        install_packages(config.extra_packages);
    }
}

fn install_packages(extra_packages: String) {
    if extra_packages.is_empty() {
        return;
    }

    info!(
        "Attempting to install the given packages: {}",
        extra_packages
    );

    let install_status = Command::new("apt-get")
        .args(["install", "-y", extra_packages.as_str()])
        .status()
        .expect("Unable to install the given packages");

    if !install_status.success() {
        panic!("Failed to install the desired packages. Please check the above output for more information.");
    }
}
