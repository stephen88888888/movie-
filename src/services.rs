// 导入会话管理模块
use crate::auth::SessionManager;
// 导入用户数据仓库
use crate::database::models::UserRepository;
// 导入线程安全的互斥锁
use std::sync::Mutex;

// 使用lazy_static创建全局的会话管理器实例
lazy_static::lazy_static! {
    static ref SESSION_MANAGER: Mutex<SessionManager> =
        Mutex::new(SessionManager::new("session.json").unwrap());
}

// 定义认证服务结构体
pub struct AuthService;

// 为认证服务实现方法
impl AuthService {
    // 登录方法，改为异步函数
    pub async fn login(
        username: &str, // 用户名
        password: &str, // 密码
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // 验证用户凭证，异步调用用户仓库的验证方法
        if let Some(user) = UserRepository::verify_user(username, password).await? {
            // 获取会话管理器的锁
            let mut session_manager = SESSION_MANAGER.lock().unwrap();
            // 创建新的会话
            let session_id = session_manager.create_session(
                user.username,         // 用户名
                user.role.to_string(), // 用户角色
                3600,                  // 会话超时时间（秒）
            )?;
            // 返回会话ID
            Ok(Some(session_id))
        } else {
            // 验证失败返回None
            Ok(None)
        }
    }

    // 退出方法，改为异步函数
    pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {
        // 获取会话管理器的锁
        let mut session_manager = SESSION_MANAGER.lock().unwrap();
        // 如果存在当前会话ID，则删除该会话
        if let Some(session_id) = session_manager.get_current_session_id() {
            session_manager.delete_session(&session_id)?;
        }
        Ok(())
    }

    // 获取当前用户的方法
    pub fn get_current_user() -> Option<String> {
        // 获取会话管理器的锁（只读）
        let session_manager = SESSION_MANAGER.lock().unwrap();
        // 返回当前用户信息
        session_manager.get_current_user()
    }
}
