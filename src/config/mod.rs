pub mod database;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub session_timeout: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        // 使用相对路径，但确保文件在当前目录创建
        let db_path = "movie.db";

        Self {
            database_url: format!("sqlite:{}", db_path),
            session_timeout: 3600, // 1小时
        }
    }
}
