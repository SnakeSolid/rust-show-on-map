use crate::config::ConfigError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ApplicationResult = Result<(), ApplicationError>;

#[derive(Debug)]
pub enum ApplicationError {
    ConfigError { message: String },
}

impl ApplicationError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn load_config_error(error: ConfigError) -> ApplicationError {
        error!("Configuration error - {}", error);

        ApplicationError::ConfigError {
            message: format!("{}", error),
        }
    }
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ApplicationError::ConfigError { message } => write!(f, "{}", message),
        }
    }
}
