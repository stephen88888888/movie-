use crate::config::AppConfig;

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| AppConfig::default().database_url)
}