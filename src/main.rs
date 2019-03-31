#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod backend;
mod config;
mod database;
mod error;
mod settings;

use crate::backend::start_backend;
use crate::error::ApplicationError;
use crate::error::ApplicationResult;
use crate::settings::Settings;

fn main() -> ApplicationResult {
    env_logger::init();

    let settings = Settings::from_args();
    let config_path = settings.config_path();
    let config = config::load(config_path).map_err(ApplicationError::load_config_error)?;

    start_backend(config, settings.bind_address(), settings.bind_port());

    Ok(())
}
