pub mod database;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub session_timeout: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite:movie.db".to_string(),
            session_timeout: 3600, // 1小时
        }
    }
}