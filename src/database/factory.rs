use std::sync::Arc;

use super::DatabaseClient;
use super::DatabaseConfig;

#[derive(Clone)]
pub struct DatabaseFactory {
    config: Arc<DatabaseConfig>,
}

impl DatabaseFactory {
    pub fn new(config: DatabaseConfig) -> DatabaseFactory {
        DatabaseFactory { config: Arc::new(config) }
    }

    pub fn client(&self) -> DatabaseClient {
        DatabaseClient::new(self.config.clone())
    }
}
