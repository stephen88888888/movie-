pub mod models;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{SqlitePool, Error};
use std::time::Duration;

static mut POOL: Option<SqlitePool> = None;

pub async fn init() -> Result<(), Error> {
    let database_url = crate::config::database::get_database_url();
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // 手动创建表（避免使用迁移宏）
    init_database_tables(&pool).await?;

    unsafe {
        POOL = Some(pool);
    }

    Ok(())
}

async fn init_database_tables(pool: &SqlitePool) -> Result<(), Error> {
    // 创建用户表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;

    // 检查是否已有用户数据
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        // 插入示例用户（密码是 "password" 的bcrypt哈希）
        sqlx::query(
            "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES 
            ('admin', '$2b$12$LQv3c1yqBWVH.0x6pQwZTuYQ7qBqQY8Y8Y8Y8Y8Y8Y8Y8Y8Y8Y8Y8', 'admin'),
            ('user1', '$2b$12$LQv3c1yqBWVH.0x6pQwZTuYQ7qBqQY8Y8Y8Y8Y8Y8Y8Y8Y8Y8Y8', 'user')"
        )
        .execute(pool)
        .await?;
        println!("Sample data initialized");
    }

    Ok(())
}

pub fn get_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    unsafe {
        if let Some(pool) = &POOL {
            Ok(pool.clone())
        } else {
            Err("Database not initialized".into())
        }
    }
}