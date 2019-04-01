use postgres::Error as PgError;
use std::error::Error;
use std::fmt::Arguments;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io::Error as IoError;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug)]
pub enum DatabaseError {
    PostgresError { message: String },
    IoError { message: String },
    NoData,
    UnsupportedFormat { message: String },
}

impl DatabaseError {
    pub fn no_data() -> DatabaseError {
        DatabaseError::NoData
    }

    pub fn unsupported_format(args: Arguments) -> DatabaseError {
        DatabaseError::UnsupportedFormat {
            message: format!("{}", args),
        }
    }
}

impl From<PgError> for DatabaseError {
    fn from(error: PgError) -> DatabaseError {
        if let Some(db_error) = error.as_db() {
            let mut message = String::default();
            let _ = write!(message, "{} ", db_error.severity);
            let _ = write!(message, "{} - ", db_error.code.code());
            let _ = write!(message, "{}", db_error.message);

            DatabaseError::PostgresError { message }
        } else if let Some(error) = error.as_io() {
            DatabaseError::PostgresError {
                message: error.description().into(),
            }
        } else {
            DatabaseError::PostgresError {
                message: format!("{}", error),
            }
        }
    }
}

impl From<IoError> for DatabaseError {
    fn from(error: IoError) -> DatabaseError {
        DatabaseError::IoError {
            message: format!("{}", error),
        }
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            DatabaseError::PostgresError { ref message } => {
                write!(f, "PostgreSQL error: {}", message)
            }
            DatabaseError::IoError { ref message } => write!(f, "IO error: {}", message),
            DatabaseError::NoData => write!(f, "No data"),
            DatabaseError::UnsupportedFormat { ref message } => {
                write!(f, "Unsupported format: {}", message)
            }
        }
    }
}

impl Error for DatabaseError {}
