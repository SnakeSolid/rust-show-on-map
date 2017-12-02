#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate argparse;
extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate postgres;
extern crate router;
extern crate serde_json;
extern crate staticfile;
extern crate time;
extern crate toml;

mod backend;
mod database;
mod logger;
mod settings;

use backend::start_backend;
use database::DatabaseFactory;
use database::DatabaseConfig;
use logger::UnwrapLog;
use settings::Settings;

fn main() {
    if let Err(err) = logger::init() {
        panic!("Failed to initialize logger: {}", err);
    }

    let settings = Settings::from_args();
    let config =
        DatabaseConfig::read(settings.config_path()).unwrap_log("Can't read configuration");
    let factory = DatabaseFactory::new(config);

    start_backend(factory, &settings.bind_address(), settings.bind_port())
        .unwrap_log("Can't start back-end server");
}
