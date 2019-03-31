use crate::backend::BackendError;
use crate::database::ReadConfigError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ApplicationResult = Result<(), ApplicationError>;

#[derive(Debug)]
pub enum ApplicationError {
    LoadConfigError { message: String },
    BackendError { message: String },
}

impl ApplicationError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn read_config_error(error: ReadConfigError) -> ApplicationError {
        error!("Failed to read configuration - {}", error);

        ApplicationError::LoadConfigError {
            message: format!("{}", error),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn backend_error(error: BackendError) -> ApplicationError {
        error!("Backend error - {}", error);

        ApplicationError::BackendError {
            message: format!("{}", error),
        }
    }
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ApplicationError::LoadConfigError { message } => write!(f, "{}", message),
            ApplicationError::BackendError { message } => write!(f, "{}", message),
        }
    }
}
