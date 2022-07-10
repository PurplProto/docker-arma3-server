use std::env;

#[derive(Debug)]
pub(crate) struct PreLaunchSettings {
    pub extra_packages: String,
    pub mods_run_from_arma_dir: bool,
}

#[derive(Debug)]
pub(crate) struct ArmaSettings {
    pub arma_binary: String,
    pub arma_config: String,
    pub arma_extra_params: String,
    pub arma_profile: String,
    pub arma_world: String,
    pub arma_limitfps: String,
    pub arma_headless_clients: String,
    pub arma_load_mods: bool,
}

#[derive(Debug)]
pub(crate) struct SteamSettings {
    pub steam_branch: String,
    pub steam_branch_password: String,
    pub steam_user: String,
    pub steam_password: String,
    pub steam_guard_code: String,
    pub steam_skip_install: bool,
}

#[derive(Debug)]
pub(crate) struct AppConfig {
    pub pre_launch: PreLaunchSettings,
    pub arma: ArmaSettings,
    pub steam: SteamSettings,
}

pub(crate) fn parse_env_vars() -> AppConfig {
    info!("Parsing environment");

    let mut settings = AppConfig {
        pre_launch: PreLaunchSettings {
            extra_packages: "".to_string(),
            mods_run_from_arma_dir: true,
        },
        arma: ArmaSettings {
            arma_binary: "".to_string(),
            arma_config: "".to_string(),
            arma_extra_params: "".to_string(),
            arma_headless_clients: "".to_string(),
            arma_limitfps: "".to_string(),
            arma_profile: "".to_string(),
            arma_world: "".to_string(),
            arma_load_mods: true,
        },
        steam: SteamSettings {
            steam_branch: "".to_string(),
            steam_user: "".to_string(),
            steam_password: "".to_string(),
            steam_guard_code: "".to_string(),
            steam_skip_install: false,
            steam_branch_password: "".to_string(),
        },
    };

    for (key, value) in env::vars() {
        match key.as_str() {
            "EXTRA_PACKAGES" => settings.pre_launch.extra_packages = value,
            "MODS_RUN_FROM_ARMA_DIR" => {
                settings.pre_launch.mods_run_from_arma_dir = value == "true"
            }
            "ARMA_BINARY" => {
                let is_standard_bin_name = value.contains("arma3server");
                if !is_standard_bin_name {
                    warn!("unusual binary name given: {}", value);
                }
                settings.arma.arma_binary = value
            }
            "ARMA_CONFIG" => settings.arma.arma_config = value,
            "ARMA_EXTRA_PARAMS" => settings.arma.arma_extra_params = value,
            "ARMA_HEADLESS_CLIENTS" => settings.arma.arma_headless_clients = value,
            "ARMA_LIMITFPS" => settings.arma.arma_limitfps = value,
            "ARMA_PROFILE" => settings.arma.arma_profile = value,
            "ARMA_WORLD" => settings.arma.arma_world = value,
            "ARMA_LOAD_MODS" => settings.arma.arma_load_mods = value == "true",
            "STEAM_BRANCH" => settings.steam.steam_branch = value,
            "STEAM_BRANCH_PASSWORD" => settings.steam.steam_branch_password = value,
            "STEAM_USER" => settings.steam.steam_user = value,
            "STEAM_PASSWORD" => settings.steam.steam_password = value,
            "STEAM_GUARD_CODE" => settings.steam.steam_guard_code = value,
            "STEAM_SKIP_INSTALL" => settings.steam.steam_skip_install = value == "true",
            _ => continue,
        }
    }

    return settings;
}
