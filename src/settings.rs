use argparse::ArgumentParser;
use argparse::StoreOption;

#[derive(Debug, Clone)]
pub struct Settings {
    bind_address: String,
    bind_port: u16,
    config_path: String,
}

impl Settings {
    pub fn from_args() -> Settings {
        info!("Parsing setting from command line arguments");

        let mut bind_address = None;
        let mut bind_port = None;
        let mut config_path = None;

        {
            let mut ap = ArgumentParser::new();

            ap.set_description("Tool to show places and roads on a map.");
            ap.refer(&mut bind_address).add_option(
                &["-b", "--bind"],
                StoreOption,
                "Address to bind on (default: localhost)",
            );
            ap.refer(&mut bind_port).add_option(
                &["-p", "--port"],
                StoreOption,
                "Port to listen on (default: 8080)",
            );
            ap.refer(&mut config_path).add_option(
                &["-c", "--config"],
                StoreOption,
                "Path to configuration file (default: config.yaml)",
            );
            ap.parse_args_or_exit();
        }

        let mut config = Self::default();

        if let Some(bind_address) = bind_address {
            config.bind_address = bind_address;
        }

        if let Some(bind_port) = bind_port {
            config.bind_port = bind_port;
        }

        if let Some(config_path) = config_path {
            config.config_path = config_path;
        }

        config
    }

    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }

    pub fn bind_port(&self) -> u16 {
        self.bind_port
    }

    pub fn config_path(&self) -> &str {
        &self.config_path
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            bind_address: "localhost".into(),
            bind_port: 8080,
            config_path: "config.yaml".into(),
        }
    }
}
