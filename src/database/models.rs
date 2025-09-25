// 导入角色和用户模型
use crate::models::{Role, User};

// 用户数据仓库结构体
pub struct UserRepository;

// 为用户仓库实现方法
impl UserRepository {
    // 根据用户名获取用户（异步函数）
    pub async fn get_user_by_username(
        username: &str, // 用户名
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        // 获取数据库连接池
        let pool = super::get_pool()?;

        // 执行SQL查询，将结果映射到User结构体
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, role FROM users WHERE username = ?",
        )
        .bind(username) // 绑定用户名参数
        .fetch_optional(&pool) // 执行查询，可能返回None
        .await?; // 等待查询完成

        Ok(user)
    }

    // 创建新用户（异步函数）
    pub async fn create_user(
        username: &str,      // 用户名
        password_hash: &str, // 密码哈希值
        role: Role,          // 用户角色
    ) -> Result<User, Box<dyn std::error::Error>> {
        // 获取数据库连接池
        let pool = super::get_pool()?;

        // 执行插入用户的SQL语句
        let result =
            sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")
                .bind(username)
                .bind(password_hash)
                .bind(role)
                .execute(&pool) // 执行插入操作
                .await?;

        // 获取刚插入的用户信息
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, role FROM users WHERE id = ?",
        )
        .bind(result.last_insert_rowid()) // 使用最后插入的ID
        .fetch_one(&pool) // 获取单条记录
        .await?;

        Ok(user)
    }

    // 验证用户凭证（异步函数）
    pub async fn verify_user(
        username: &str, // 用户名
        password: &str, // 密码
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        // 尝试获取用户并验证密码
        let user = Self::get_user_by_username(username).await?;
        if let Some(user) = user {
            // 使用密码验证函数检查密码是否匹配
            if crate::auth::verify_password(password, &user.password_hash)? {
                // 验证成功返回用户信息
                return Ok(Some(user));
            }
        }
        // 验证失败返回None
        Ok(None)
    }
}
