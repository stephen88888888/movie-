// 导入HashMap集合
use std::collections::HashMap;
// 导入日期时间处理
use chrono::{DateTime, Utc};

// 会话结构体定义
#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,          // 用户名
    pub role: String,              // 用户角色
    pub created_at: DateTime<Utc>, // 会话创建时间
    pub expires_at: DateTime<Utc>, // 会话过期时间
}

// 为会话实现方法
impl Session {
    // 创建新会话
    pub fn new(username: String, role: String, timeout_seconds: i64) -> Self {
        let now = Utc::now(); // 获取当前时间
        Self {
            username,
            role,
            created_at: now, // 设置创建时间为当前时间
            expires_at: now + chrono::Duration::seconds(timeout_seconds), // 计算过期时间
        }
    }

    // 检查会话是否有效（未过期）
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at // 当前时间小于过期时间即为有效
    }
}

// 会话管理器结构体
pub struct SessionManager {
    sessions: HashMap<String, Session>, // 存储会话ID到会话的映射
}

// 为会话管理器实现方法
impl SessionManager {
    // 创建新的会话管理器
    pub fn new(_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            sessions: HashMap::new(), // 初始化空的会话映射
        })
    }

    // 创建新会话
    pub fn create_session(
        &mut self,
        username: String,
        role: String,
        timeout: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 生成唯一的会话ID（用户名+时间戳）
        let session_id = format!("{}_{}", username, Utc::now().timestamp());
        // 创建新会话
        let session = Session::new(username, role, timeout);

        // 将会话存入映射
        self.sessions.insert(session_id.clone(), session);
        // 清理过期会话
        self.clean_expired_sessions();

        // 返回会话ID
        Ok(session_id)
    }

    // 根据会话ID获取会话
    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        // 查找会话并检查是否有效
        self.sessions.get(session_id).filter(|s| s.is_valid())
    }

    // 删除指定会话
    pub fn delete_session(&mut self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.sessions.remove(session_id);
        Ok(())
    }

    // 获取当前用户
    pub fn get_current_user(&self) -> Option<String> {
        self.sessions
            .values()
            .find(|s| s.is_valid()) // 查找第一个有效会话
            .map(|s| s.username.clone()) // 返回用户名
    }

    // 获取当前会话ID
    pub fn get_current_session_id(&self) -> Option<String> {
        self.sessions
            .iter()
            .find(|(_, session)| session.is_valid()) // 查找第一个有效会话
            .map(|(id, _)| id.clone()) // 返回会话ID
    }

    // 清理过期会话
    fn clean_expired_sessions(&mut self) {
        // 只保留有效的会话
        self.sessions.retain(|_, session| session.is_valid());
    }
}
