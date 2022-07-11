#[macro_use]
extern crate log;
extern crate env_logger;
mod app_config;
mod pre_launch;

fn main() {
    env_logger::init_from_env("LOG_LEVEL");
    info!("Launcher app loaded.");

    let config = app_config::parse_env_vars();
    debug!("This is our config: {:?}", config);
    pre_launch::handle_env_setup(config.pre_launch);
}
