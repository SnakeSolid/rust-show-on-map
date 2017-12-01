use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::io::Error as IoError;
use std::path::PathBuf;

#[derive(Debug)]
pub enum DatabaseError {
    IsDirectoryError { path: PathBuf },
    IoError { description: String },
    NoData,
}

impl DatabaseError {
    pub fn is_directory(path: PathBuf) -> DatabaseError {
        DatabaseError::IsDirectoryError { path }
    }

    pub fn no_data() -> DatabaseError {
        DatabaseError::NoData
    }
}

impl From<IoError> for DatabaseError {
    fn from(error: IoError) -> DatabaseError {
        DatabaseError::IoError { description: error.description().into() }
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            DatabaseError::IsDirectoryError { ref path } => {
                write!(f, "Database path is directory: {}", path.display())
            }
            DatabaseError::IoError { ref description } => write!(f, "IO error: {}", description),
            DatabaseError::NoData => write!(f, "No data"),
        }
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        match *self {
            DatabaseError::IsDirectoryError { .. } => "Database path is directory",
            DatabaseError::IoError { .. } => "IO error",
            DatabaseError::NoData => "No data",
        }
    }
}
