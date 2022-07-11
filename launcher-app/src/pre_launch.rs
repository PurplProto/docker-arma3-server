use crate::app_config::PreLaunchSettings;
use std::{fs, path::PathBuf, process::Command};

#[derive(Debug)]
pub struct PreLaunchMeta {
    pub has_mods: bool,
    pub mods: Vec<PathBuf>,
    pub has_server_mods: bool,
    pub server_mods: Vec<PathBuf>,
}

pub(crate) fn handle_env_setup(config: PreLaunchSettings) -> PreLaunchMeta {
    let mods = get_mod_names();
    let server_mods = get_server_mod_names();
    let meta = PreLaunchMeta {
        has_mods: mods.len() > 0,
        mods,
        has_server_mods: server_mods.len() > 0,
        server_mods,
    };

    debug!("pre-launch metadata: {:?}", meta);

    if !config.extra_packages.is_empty() {
        install_packages(config.extra_packages);
    }

    return meta;
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

fn get_mod_names() -> Vec<PathBuf> {
    get_relative_directories_of("./mods")
}

fn get_server_mod_names() -> Vec<PathBuf> {
    get_relative_directories_of("./server_mods")
}

fn get_relative_directories_of(path: &str) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(path) {
        entries
            .into_iter()
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().path())
            .filter(|r| r.is_dir())
            .collect()
    } else {
        return Vec::new();
    }
}
