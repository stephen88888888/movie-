use crate::auth::SessionManager;
use crate::database::models::UserRepository;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SESSION_MANAGER: Mutex<SessionManager> =
        Mutex::new(SessionManager::new("session.json").unwrap());
}

pub struct AuthService;

impl AuthService {
    pub async fn login(  // 改为 async
        username: &str,
        password: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // 移除 rt.block_on，直接使用异步调用
        if let Some(user) = UserRepository::verify_user(username, password).await? {
            let mut session_manager = SESSION_MANAGER.lock().unwrap();
            let session_id = session_manager.create_session(
                user.username,
                user.role.to_string(),
                3600,
            )?;
            Ok(Some(session_id))
        } else {
            Ok(None)
        }
    }

    pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {  // 改为 async
        let mut session_manager = SESSION_MANAGER.lock().unwrap();
        if let Some(session_id) = session_manager.get_current_session_id() {
            session_manager.delete_session(&session_id)?;
        }
        Ok(())
    }

    pub fn get_current_user() -> Option<String> {
        let session_manager = SESSION_MANAGER.lock().unwrap();
        session_manager.get_current_user()
    }
}