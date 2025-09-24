pub mod models;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Error, SqlitePool};
use std::sync::Mutex;
use std::time::Duration;
use bcrypt::{hash, verify, DEFAULT_COST};

static POOL: Mutex<Option<SqlitePool>> = Mutex::new(None);

pub async fn init() -> Result<(), Error> {
    let database_url = crate::config::database::get_database_url();

    println!("Connecting to database: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10)) // 增加超时时间
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to database: {}", e);
            e
        })?;

    println!("Database connected successfully");

    // 手动创建表
    init_database_tables(&pool).await?;

    // 设置全局连接池
    let mut global_pool = POOL.lock().unwrap();
    *global_pool = Some(pool);

    Ok(())
}

async fn init_database_tables(pool: &SqlitePool) -> Result<(), Error> {
    println!("Initializing database tables...");

    // 创建用户表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;

    println!("Users table created/verified");

    // 检查是否已有用户数据
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

        if count.0 == 0 {
            println!("Inserting sample data...");
            
            // 使用 bcrypt 库动态生成正确的哈希
            let password_hash = bcrypt::hash("password", 12)
                .expect("Failed to hash password");
            
            sqlx::query(
                "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES 
                ('admin', ?, 'admin'),
                ('user1', ?, 'user')",
            )
            .bind(&password_hash)
            .bind(&password_hash)
            .execute(pool)
            .await?;
            
            println!("Sample data initialized");
        }else {
        println!("Database already contains {} users", count.0);
    }

    Ok(())
}

pub fn get_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let pool_guard = POOL.lock().unwrap();
    pool_guard.clone().ok_or("Database not initialized".into())
}
