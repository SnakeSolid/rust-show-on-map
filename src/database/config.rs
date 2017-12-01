use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;
use std::path::Path;

use toml;
use toml::de::Error as TomlError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    points_for_places: String,
    points_for_roads: String,
    unique_place_ids: String,
    unique_road_ids: String,
    names_for_places: String,
    names_for_roads: String,
}

#[derive(Debug)]
pub enum ReadConfigError {
    IoError { description: String },
    TomlError { description: String },
}

impl From<IoError> for ReadConfigError {
    fn from(error: IoError) -> ReadConfigError {
        ReadConfigError::IoError { description: error.description().into() }
    }
}

impl From<TomlError> for ReadConfigError {
    fn from(error: TomlError) -> ReadConfigError {
        ReadConfigError::TomlError { description: error.description().into() }
    }
}

impl Display for ReadConfigError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            ReadConfigError::IoError { ref description } => write!(f, "IO error: {}", description),
            ReadConfigError::TomlError { ref description } => {
                write!(f, "TOML error: {}", description)
            }
        }
    }
}

impl Error for ReadConfigError {
    fn description(&self) -> &str {
        match *self {
            ReadConfigError::IoError { .. } => "IO error",
            ReadConfigError::TomlError { .. } => "TOML error",
        }
    }
}

impl DatabaseConfig {
    pub fn read<P>(path: P) -> Result<DatabaseConfig, ReadConfigError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        info!("Reading file `{}`.", path.display());

        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        info!("Parsing configuration from file `{}`.", path.display());

        Ok(toml::from_str(&content)?)
    }
}
