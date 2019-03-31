use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::fmt::Write;
use std::io::Error as IoError;

use postgres::Error as PgError;

#[derive(Debug)]
pub enum DatabaseError {
    PostgresError { description: String },
    IoError { description: String },
    NoData,
}

impl DatabaseError {
    pub fn no_data() -> DatabaseError {
        DatabaseError::NoData
    }
}

impl From<PgError> for DatabaseError {
    fn from(error: PgError) -> DatabaseError {
        if let Some(db_error) = error.as_db() {
            let mut description = String::default();
            let _ = write!(description, "{} ", db_error.severity);
            let _ = write!(description, "{} - ", db_error.code.code());
            let _ = write!(description, "{}", db_error.message);

            DatabaseError::PostgresError { description }
        } else if let Some(error) = error.as_io() {
            DatabaseError::PostgresError {
                description: error.description().into(),
            }
        } else {
            DatabaseError::PostgresError {
                description: format!("{}", error),
            }
        }
    }
}

impl From<IoError> for DatabaseError {
    fn from(error: IoError) -> DatabaseError {
        DatabaseError::IoError {
            description: format!("{}", error),
        }
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            DatabaseError::PostgresError { ref description } => {
                write!(f, "PostgreSQL error: {}", description)
            }
            DatabaseError::IoError { ref description } => write!(f, "IO error: {}", description),
            DatabaseError::NoData => write!(f, "No data"),
        }
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        match *self {
            DatabaseError::PostgresError { .. } => "PostgreSQL error",
            DatabaseError::IoError { .. } => "IO error",
            DatabaseError::NoData => "No data",
        }
    }
}
