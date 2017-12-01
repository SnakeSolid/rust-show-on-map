use std::sync::Arc;

use super::DatabaseConfig;

pub struct DatabaseClient {
    config: Arc<DatabaseConfig>,
}

impl DatabaseClient {
    pub fn new(config: Arc<DatabaseConfig>) -> DatabaseClient {
        DatabaseClient { config }
    }
}
