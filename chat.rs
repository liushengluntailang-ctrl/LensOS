use crate::ai::AIModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role of a participant in a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "User"),
            MessageRole::Assistant => write!(f, "Assistant"),
            MessageRole::System => write!(f, "System"),
        }
    }
}

/// Individual message entry in a chat session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tokens: usize,
    pub code_snippets: Vec<String>,
    pub is_pinned: bool,
}

impl ChatMessage {
    pub fn new(id: impl Into<String>, role: MessageRole, content: impl Into<String>) -> Self {
        let content_str = content.into();
        let code_snippets = Self::extract_code_blocks(&content_str);
        let tokens = (content_str.len() as f64 / 4.0).ceil() as usize;

        Self {
            id: id.into(),
            role,
            content: content_str,
            timestamp: Utc::now(),
            tokens,
            code_snippets,
            is_pinned: false,
        }
    }

    fn extract_code_blocks(text: &str) -> Vec<String> {
        let mut snippets = Vec::new();
        let mut in_block = false;
        let mut current_snippet = String::new();

        for line in text.lines() {
            if line.starts_with("```") {
                if in_block {
                    snippets.push(current_snippet.trim().to_string());
                    current_snippet.clear();
                    in_block = false;
                } else {
                    in_block = true;
                }
            } else if in_block {
                current_snippet.push_str(line);
                current_snippet.push('\n');
            }
        }
        snippets
    }
}

/// A complete chat conversation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub model: AIModel,
    pub active_system_prompt: Option<String>,
}

impl ChatSession {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            title: title.into(),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            model: AIModel::default(),
            active_system_prompt: None,
        }
    }

    pub fn with_model(mut self, model: AIModel) -> Self {
        self.model = model;
        self
    }

    pub fn add_message(&mut self, role: MessageRole, content: impl Into<String>) -> &ChatMessage {
        let id = format!("msg_{}_{}", self.messages.len() + 1, Utc::now().timestamp_millis());
        let msg = ChatMessage::new(id, role, content);
        self.messages.push(msg);
        self.updated_at = Utc::now();
        self.messages.last().unwrap()
    }

    pub fn add_system_message(&mut self, content: impl Into<String>) -> &ChatMessage {
        self.add_message(MessageRole::System, content)
    }

    pub fn clear_history(&mut self) {
        self.messages.clear();
        self.updated_at = Utc::now();
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn total_tokens(&self) -> usize {
        self.messages.iter().map(|m| m.tokens).sum()
    }

    pub fn get_context_window(&self, max_tokens: usize) -> Vec<ChatMessage> {
        let mut accumulated_tokens = 0;
        let mut window = Vec::new();

        for msg in self.messages.iter().rev() {
            if accumulated_tokens + msg.tokens > max_tokens {
                break;
            }
            accumulated_tokens += msg.tokens;
            window.push(msg.clone());
        }

        window.reverse();
        window
    }

    pub fn search_messages(&self, query: &str) -> Vec<&ChatMessage> {
        let q = query.to_lowercase();
        self.messages
            .iter()
            .filter(|m| m.content.to_lowercase().contains(&q))
            .collect()
    }

    pub fn pin_message(&mut self, message_id: &str) -> bool {
        if let Some(msg) = self.messages.iter_mut().find(|m| m.id == message_id) {
            msg.is_pinned = !msg.is_pinned;
            true
        } else {
            false
        }
    }

    pub fn export_markdown(&self) -> String {
        let mut out = format!("# {}\n\n*Created: {} | Model: {}*\n\n---\n\n", self.title, self.created_at.to_rfc3339(), self.model.name());
        for msg in &self.messages {
            out.push_str(&format!("### **{}** ({})\n{}\n\n", msg.role, msg.timestamp.format("%H:%M:%S"), msg.content));
        }
        out
    }
}

impl Default for ChatSession {
    fn default() -> Self {
        Self::new("session_default", "New LensAI Chat")
    }
}
