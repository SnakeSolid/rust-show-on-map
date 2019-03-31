use std::sync::Arc;

use super::DatabaseClient;
use super::DatabaseConfig;

#[derive(Clone)]
pub struct DatabaseFactory {
    config: Arc<DatabaseConfig>,
}

impl DatabaseFactory {
    pub fn new(config: DatabaseConfig) -> DatabaseFactory {
        DatabaseFactory {
            config: Arc::new(config),
        }
    }

    pub fn client(
        &self,
        host: &str,
        port: i16,
        database: &str,
        role: &str,
        password: &str,
    ) -> DatabaseClient {
        DatabaseClient::new(self.config.clone(), host, port, database, role, password)
    }
}
