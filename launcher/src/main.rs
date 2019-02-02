mod settings;

use config::{Config, Environment, File};
use failure::Error;

include!(concat!(env!("OUT_DIR"), "/built.rs"));

fn load_settings() -> Result<crate::settings::Settings, Error> {
    let mut config = Config::default();
    config.merge(File::with_name(&format!("settings/{}", PROFILE)).required(false))?;
    config.merge(Environment::with_prefix("TRI"))?;

    Ok(config.try_into()?)
}

fn run() -> Result<(), Error> {
    let settings = load_settings()?;
    application::Application::from_settings(&settings)?.run()?;
    Ok(())
}

fn main() {
    env_logger::Builder::from_env("LOG").init();
    log::info!(
        "Start {} {} mode (version: {})",
        PKG_NAME,
        PROFILE,
        PKG_VERSION
    );

    if let Err(err) = run() {
        log::error!("{:?}", err);
        std::process::exit(1);
    } else {
        log::info!("Quit {}", PKG_NAME);
        std::process::exit(0);
    }
}
