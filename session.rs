//! LensOS v0.1 - User Session Management
//!
//! Controls user session lifecycles, active session contexts, session locking/unlocking,
//! auto-logout parameters, and session state tracking across desktop interfaces.

use crate::user::User;
use std::collections::HashMap;

/// Status of an active user session
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session is active and accepting user interaction
    Active,
    /// Desktop workspace is locked requiring re-authentication
    Locked,
    /// User is idle (no input detected)
    Idle,
    /// Session is suspended (e.g. during sleep state)
    Suspended,
    /// Session has been terminated/logged out
    Terminated,
}

/// Represents an active runtime session for a logged-in LensOS user
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier token
    pub session_id: String,
    /// Associated user profile
    pub user: User,
    /// Current lifecycle state
    pub state: SessionState,
    /// Unix timestamp (seconds) when session was established
    pub created_at: u64,
    /// Unix timestamp of last user activity
    pub last_active_at: u64,
    /// Custom session environment variables / attributes
    pub attributes: HashMap<String, String>,
}

impl Session {
    /// Creates a new user session
    pub fn new(session_id: impl Into<String>, user: User) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            session_id: session_id.into(),
            user,
            state: SessionState::Active,
            created_at: now,
            last_active_at: now,
            attributes: HashMap::new(),
        }
    }

    /// Updates session heartbeat / activity timestamp
    pub fn touch(&mut self) {
        self.last_active_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if self.state == SessionState::Idle {
            self.state = SessionState::Active;
        }
    }

    /// Locks the session screen
    pub fn lock(&mut self) {
        if self.state != SessionState::Terminated {
            self.state = SessionState::Locked;
        }
    }

    /// Unlocks the session screen back to active state
    pub fn unlock(&mut self) {
        if self.state == SessionState::Locked {
            self.state = SessionState::Active;
            self.touch();
        }
    }
}

/// Session Manager orchestrating user sessions in LensOS
#[derive(Debug, Default)]
pub struct SessionManager {
    /// Active sessions keyed by session_id
    sessions: HashMap<String, Session>,
    /// Currently active session_id powering desktop UI
    active_session_id: Option<String>,
    /// Session idle timeout in seconds (default: 900s / 15 minutes)
    idle_timeout_secs: u64,
}

impl SessionManager {
    /// Creates a new `SessionManager`
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            active_session_id: None,
            idle_timeout_secs: 900,
        }
    }

    /// Registers and activates a new session for a user
    pub fn create_session(&mut self, user: User) -> String {
        let session_id = format!("sess_{}_{}", user.id, std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0));

        let session = Session::new(&session_id, user);
        self.sessions.insert(session_id.clone(), session);
        self.active_session_id = Some(session_id.clone());
        session_id
    }

    /// Gets reference to the active desktop session
    pub fn get_active_session(&self) -> Option<&Session> {
        self.active_session_id.as_ref().and_then(|id| self.sessions.get(id))
    }

    /// Gets mutable reference to the active desktop session
    pub fn get_active_session_mut(&mut self) -> Option<&mut Session> {
        if let Some(id) = self.active_session_id.clone() {
            self.sessions.get_mut(&id)
        } else {
            None
        }
    }

    /// Switches active session focus to another session ID
    pub fn switch_session(&mut self, session_id: &str) -> bool {
        if self.sessions.contains_key(session_id) {
            self.active_session_id = Some(session_id.to_string());
            if let Some(sess) = self.sessions.get_mut(session_id) {
                sess.touch();
            }
            true
        } else {
            false
        }
    }

    /// Terminates a session by ID
    pub fn terminate_session(&mut self, session_id: &str) -> bool {
        if let Some(mut sess) = self.sessions.remove(session_id) {
            sess.state = SessionState::Terminated;
            if self.active_session_id.as_deref() == Some(session_id) {
                self.active_session_id = self.sessions.keys().next().cloned();
            }
            true
        } else {
            false
        }
    }

    /// Checks sessions for idle timeout and transitions them to Idle/Locked
    pub fn evaluate_idle_timeouts(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        for session in self.sessions.values_mut() {
            if session.state == SessionState::Active && (now - session.last_active_at) > self.idle_timeout_secs {
                session.state = SessionState::Idle;
            }
        }
    }
}
