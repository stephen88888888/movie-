use crate::models::{Role, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn get_user_by_username(
        username: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let pool = super::get_pool()?;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, role FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(
        username: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let pool = super::get_pool()?;

        let result =
            sqlx::query("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")
                .bind(username)
                .bind(password_hash)
                .bind(role)
                .execute(&pool)
                .await?;

        // 获取刚插入的用户
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, role FROM users WHERE id = ?",
        )
        .bind(result.last_insert_rowid())
        .fetch_one(&pool)
        .await?;

        Ok(user)
    }

    pub async fn verify_user(
        username: &str,
        password: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        if let Some(user) = Self::get_user_by_username(username).await?
            && crate::auth::verify_password(password, &user.password_hash)?
        {
            return Ok(Some(user));
        }
        Ok(None)
    }
}
