use crate::chat::ChatSession;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Lightweight summary metadata for conversation history lists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub session_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub model_used: String,
    pub tags: Vec<String>,
}

/// Conversation history storage and session manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryManager {
    sessions: Vec<ChatSession>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    pub fn save_session(&mut self, session: &ChatSession) -> Result<(), String> {
        if let Some(existing) = self.sessions.iter_mut().find(|s| s.id == session.id) {
            *existing = session.clone();
        } else {
            self.sessions.push(session.clone());
        }
        Ok(())
    }

    pub fn get_session(&self, session_id: &str) -> Option<&ChatSession> {
        self.sessions.iter().find(|s| s.id == session_id)
    }

    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut ChatSession> {
        self.sessions.iter_mut().find(|s| s.id == session_id)
    }

    pub fn list_entries(&self) -> Vec<HistoryEntry> {
        self.sessions
            .iter()
            .map(|s| HistoryEntry {
                session_id: s.id.clone(),
                title: s.title.clone(),
                created_at: s.created_at,
                updated_at: s.updated_at,
                message_count: s.messages.len(),
                model_used: s.model.name().to_string(),
                tags: vec!["lensos".to_string(), "ai".to_string()],
            })
            .collect()
    }

    pub fn search_history(&self, query: &str) -> Vec<HistoryEntry> {
        let q = query.to_lowercase();
        self.sessions
            .iter()
            .filter(|s| {
                s.title.to_lowercase().contains(&q)
                    || s.messages.iter().any(|m| m.content.to_lowercase().contains(&q))
            })
            .map(|s| HistoryEntry {
                session_id: s.id.clone(),
                title: s.title.clone(),
                created_at: s.created_at,
                updated_at: s.updated_at,
                message_count: s.messages.len(),
                model_used: s.model.name().to_string(),
                tags: vec!["lensos".to_string()],
            })
            .collect()
    }

    pub fn delete_session(&mut self, session_id: &str) -> bool {
        let original_len = self.sessions.len();
        self.sessions.retain(|s| s.id != session_id);
        self.sessions.len() < original_len
    }

    pub fn clear_all(&mut self) {
        self.sessions.clear();
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn export_all_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.sessions)
            .map_err(|e| format!("Failed to export history: {}", e))
    }
}
