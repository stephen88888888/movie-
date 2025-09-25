// 导入认证服务
use crate::services::AuthService;
// 导入错误类型
use std::error::Error;

// 处理登录的函数
pub async fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    // 显示用户名
    println!("Username: {}", username);

    // 提示用户输入密码
    println!("Please enter the password:");
    // 使用tokio的阻塞任务来读取密码（避免阻塞异步运行时）
    let password = tokio::task::spawn_blocking(|| {
        rpassword::read_password().expect("Failed to read password")
    })
    .await // 等待阻塞任务完成
    .expect("Task failed");

    // 调用认证服务的登录方法
    match AuthService::login(username, &password).await {
        Ok(Some(_session_id)) => {
            // 登录成功
            println!("Login successful!");
            // 显示当前用户信息
            if let Some(current_user) = AuthService::get_current_user() {
                println!("Current user: {}", current_user);
            }
        }
        Ok(None) => {
            // 用户名或密码错误
            println!("Invalid username or password.");
        }
        Err(e) => {
            // 登录过程中发生错误
            println!("Login error: {}", e);
        }
    }

    Ok(())
}

// 处理退出的函数
pub async fn handle_logout() -> Result<(), Box<dyn Error>> {
    // 调用认证服务的退出方法
    match AuthService::logout().await {
        Ok(()) => {
            // 退出成功
            println!("Logout successful!");
        }
        Err(e) => {
            // 退出过程中发生错误
            println!("Logout error: {}", e);
        }
    }
    Ok(())
}
