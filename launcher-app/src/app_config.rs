use std::env;

#[derive(Debug)]
pub(crate) struct PreLaunchSettings {
    pub extra_packages: String,
    pub mods_run_from_arma_dir: bool,
}

#[derive(Debug)]
pub(crate) struct ArmaSettings {
    pub binary: String,
    pub config: String,
    pub extra_params: String,
    pub profile: String,
    pub world: String,
    pub limitfps: String,
    pub headless_clients: String,
    pub load_mods: bool,
}

#[derive(Debug)]
pub(crate) struct SteamSettings {
    pub branch: String,
    pub branch_password: String,
    pub user: String,
    pub password: String,
    pub guard_code: String,
    pub skip_install: bool,
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
            binary: "".to_string(),
            config: "".to_string(),
            extra_params: "".to_string(),
            headless_clients: "".to_string(),
            limitfps: "".to_string(),
            profile: "".to_string(),
            world: "".to_string(),
            load_mods: true,
        },
        steam: SteamSettings {
            branch: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            guard_code: "".to_string(),
            skip_install: false,
            branch_password: "".to_string(),
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
                settings.arma.binary = value
            }
            "ARMA_CONFIG" => settings.arma.config = value,
            "ARMA_EXTRA_PARAMS" => settings.arma.extra_params = value,
            "ARMA_HEADLESS_CLIENTS" => settings.arma.headless_clients = value,
            "ARMA_LIMITFPS" => settings.arma.limitfps = value,
            "ARMA_PROFILE" => settings.arma.profile = value,
            "ARMA_WORLD" => settings.arma.world = value,
            "ARMA_LOAD_MODS" => settings.arma.load_mods = value == "true",
            "STEAM_BRANCH" => settings.steam.branch = value,
            "STEAM_BRANCH_PASSWORD" => settings.steam.branch_password = value,
            "STEAM_USER" => settings.steam.user = value,
            "STEAM_PASSWORD" => settings.steam.password = value,
            "STEAM_GUARD_CODE" => settings.steam.guard_code = value,
            "STEAM_SKIP_INSTALL" => settings.steam.skip_install = value == "true",
            _ => continue,
        }
    }

    return settings;
}
