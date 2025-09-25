// 导入应用配置结构体
use crate::config::AppConfig;

// 获取数据库URL的函数
pub fn get_database_url() -> String {
    // 从环境变量获取数据库URL，如果不存在则使用默认配置
    std::env::var("DATABASE_URL").unwrap_or_else(|_| AppConfig::default().database_url)
}
