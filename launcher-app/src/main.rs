#[macro_use]
extern crate log;
extern crate colored;
extern crate pretty_env_logger;
mod app_config;

fn main() {
    pretty_env_logger::init();
    info!("Launcher app loaded.");

    let config = app_config::parse_env_vars();

    debug!("This is our config: {:?}", config)
}
