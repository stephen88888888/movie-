// 声明模型模块
pub mod models;

// 导入SQLite连接池相关
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Error, SqlitePool};
// 导入线程安全的数据结构
use std::sync::Mutex;
use std::time::Duration;
// 允许未使用的导入（用于测试或示例）
#[allow(unused_imports)]
use bcrypt::{DEFAULT_COST, hash, verify};

// 全局数据库连接池
static POOL: Mutex<Option<SqlitePool>> = Mutex::new(None);

// 初始化数据库连接（异步函数）
pub async fn init() -> Result<(), Error> {
    // 获取数据库URL
    let database_url = crate::config::database::get_database_url();

    println!("Connecting to database: {}", database_url);

    // 创建连接池
    let pool = SqlitePoolOptions::new()
        .max_connections(5) // 最大连接数
        .acquire_timeout(Duration::from_secs(10)) // 增加超时时间
        .connect(&database_url) // 连接数据库
        .await // 等待连接完成
        .map_err(|e| {
            eprintln!("Failed to connect to database: {}", e);
            e
        })?;

    println!("Database connected successfully");

    // 手动创建数据库表
    init_database_tables(&pool).await?;

    // 设置全局连接池
    let mut global_pool = POOL.lock().unwrap();
    *global_pool = Some(pool);

    Ok(())
}

// 初始化数据库表结构（异步函数）
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
    .execute(pool) // 执行建表语句
    .await?;

    println!("Users table created/verified");

    // 检查是否已有用户数据
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool) // 获取单条记录
        .await?;

    // 如果没有用户数据，插入示例数据
    if count.0 == 0 {
        println!("Inserting sample data...");

        // 为不同用户设置不同的强密码
        let admin_password_hash = hash("admin123", 12).expect("Failed to hash password");
        let user1_password_hash = hash("user123", 12).expect("Failed to hash password");

        // 插入示例用户数据
        sqlx::query(
            "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES 
                ('admin', ?, 'admin'),
                ('user1', ?, 'user')",
        )
        .bind(&admin_password_hash)
        .bind(&user1_password_hash)
        .execute(pool)
        .await?;

        println!("Sample data initialized");
    } else {
        // 如果已有用户数据，显示数量
        println!("Database already contains {} users", count.0);
    }

    Ok(())
}

// 获取数据库连接池的函数
pub fn get_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // 获取连接池的锁
    let pool_guard = POOL.lock().unwrap();
    // 返回连接池或错误
    pool_guard.clone().ok_or("Database not initialized".into())
}
