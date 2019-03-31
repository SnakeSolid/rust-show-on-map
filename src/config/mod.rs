mod error;

pub use self::error::ConfigError;
pub use self::error::ConfigResult;

use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub type ConfigRef = Arc<Config>;

#[derive(Debug, Deserialize)]
pub struct Config {
    formats: BTreeMap<String, FormatConfig>,
}

impl Config {
    pub fn formats(&self) -> &BTreeMap<String, FormatConfig> {
        &self.formats
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormatConfig {
    format_type: FormatType,
    names_query: String,
    geometry_query: String,
}

impl FormatConfig {
    pub fn format_type(&self) -> FormatType {
        self.format_type
    }

    pub fn names_query(&self) -> &str {
        &self.names_query
    }

    pub fn geometry_query(&self) -> &str {
        &self.geometry_query
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Deserialize)]
pub enum FormatType {
    PlainLines,
    PlainPolygons,
    Wkt,
}

pub fn load<P>(path: P) -> ConfigResult<ConfigRef>
where
    P: AsRef<Path>,
{
    let reader = File::open(path).map_err(ConfigError::io_error)?;
    let config = serde_yaml::from_reader(reader).map_err(ConfigError::yaml_error)?;

    Ok(Arc::new(config))
}
