// 声明数据库配置模块
pub mod database;

// 导入反序列化支持
use serde::Deserialize;

// 应用配置结构体定义
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String, // 数据库连接URL
    pub session_timeout: i64, // 会话超时时间（秒）
}

// 为应用配置实现默认值
impl Default for AppConfig {
    fn default() -> Self {
        // 设置SQLite数据库文件路径和模式（读写创建）
        let db_path = "file:movie.db?mode=rwc";

        Self {
            // 构建完整的数据库URL
            database_url: format!("sqlite:{}", db_path),
            // 默认会话超时时间为1小时
            session_timeout: 3600,
        }
    }
}
