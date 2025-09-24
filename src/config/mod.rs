pub mod database;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub session_timeout: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        // 修改这一行 - 添加 file: 前缀和模式参数
        let db_path = "file:movie.db?mode=rwc";

        Self {
            database_url: format!("sqlite:{}", db_path),
            session_timeout: 3600,
        }
    }
}
