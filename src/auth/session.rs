use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(username: String, role: String, timeout_seconds: i64) -> Self {
        let now = Utc::now();
        Self {
            username,
            role,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(timeout_seconds),
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new(_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            sessions: HashMap::new(),
        })
    }

    pub fn create_session(&mut self, username: String, role: String, timeout: i64) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = format!("{}_{}", username, Utc::now().timestamp());
        let session = Session::new(username, role, timeout);
        
        self.sessions.insert(session_id.clone(), session);
        self.clean_expired_sessions();
        
        Ok(session_id)
    }

    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id).filter(|s| s.is_valid())
    }

    pub fn delete_session(&mut self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.sessions.remove(session_id);
        Ok(())
    }

    pub fn get_current_user(&self) -> Option<String> {
        self.sessions.values()
            .find(|s| s.is_valid())
            .map(|s| s.username.clone())
    }

    pub fn get_current_session_id(&self) -> Option<String> {
        self.sessions.iter()
            .find(|(_, session)| session.is_valid())
            .map(|(id, _)| id.clone())
    }

    fn clean_expired_sessions(&mut self) {
        self.sessions.retain(|_, session| session.is_valid());
    }
}