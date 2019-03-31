#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod algorithm;
mod backend;
mod database;
mod error;
mod settings;

use crate::backend::start_backend;
use crate::database::DatabaseConfig;
use crate::database::DatabaseFactory;
use crate::error::ApplicationError;
use crate::error::ApplicationResult;
use crate::settings::Settings;

fn main() -> ApplicationResult {
    env_logger::init();

    let settings = Settings::from_args();
    let config = DatabaseConfig::read(settings.config_path())
        .map_err(ApplicationError::read_config_error)?;
    let factory = DatabaseFactory::new(config);

    start_backend(factory, &settings.bind_address(), settings.bind_port())
        .map_err(ApplicationError::backend_error)
}
