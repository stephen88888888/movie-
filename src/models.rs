// 导入序列化和反序列化支持
use serde::{Deserialize, Serialize};
// 导入数据库行映射支持
use sqlx::FromRow;
// 导入显示格式化支持
use std::fmt::Display;

// 定义用户结构体，并自动实现各种trait
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,               // 用户ID
    pub username: String,      // 用户名
    pub password_hash: String, // 密码哈希值
    pub role: Role,            // 用户角色
}

// 定义角色枚举，映射到数据库的文本类型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum Role {
    Admin, // 管理员角色
    User,  // 普通用户角色
}

// 为Role实现Display trait，用于友好显示
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Administrator"), // 显示为"Administrator"
            Role::User => write!(f, "User"),           // 显示为"User"
        }
    }
}

// 为Role实现从字符串的转换
impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,         // "admin"转换为管理员角色
            "administrator" => Role::Admin, // "administrator"也转换为管理员角色
            _ => Role::User,                // 其他情况都转换为普通用户角色
        }
    }
}
