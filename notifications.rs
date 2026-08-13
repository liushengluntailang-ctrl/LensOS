//! LensOS v0.1 - Notification System
//!
//! Provides system-wide notification dispatching, prioritization, filtering,
//! user action handling, and desktop panel notification queue management.

use std::collections::VecDeque;

/// Importance / Severity level of a notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationLevel {
    /// Informational background event
    Info,
    /// Task completion / operation success
    Success,
    /// Non-blocking warning requiring user attention
    Warning,
    /// Application or service operational error
    Error,
    /// Critical system alert (e.g. power loss, kernel fault)
    Critical,
}

impl Default for NotificationLevel {
    fn default() -> Self {
        NotificationLevel::Info
    }
}

/// System notification object
#[derive(Debug, Clone)]
pub struct Notification {
    /// Unique message identifier
    pub id: String,
    /// Headline title
    pub title: String,
    /// Detailed text body
    pub body: String,
    /// Severity level
    pub level: NotificationLevel,
    /// Module or application emitting the notification
    pub source_module: String,
    /// Dispatch timestamp
    pub timestamp: u64,
    /// Read/acknowledged flag
    pub is_read: bool,
    /// Optional action button labels (e.g., ["Restart", "Cancel"])
    pub actions: Vec<String>,
}

impl Notification {
    /// Creates a new notification object
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        level: NotificationLevel,
        source_module: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            level,
            source_module: source_module.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            is_read: false,
            actions: Vec::new(),
        }
    }

    /// Attaches interactive action buttons to notification
    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.actions = actions;
        self
    }
}

/// Center managing system notifications and UI toasts
#[derive(Debug, Default)]
pub struct NotificationCenter {
    queue: VecDeque<Notification>,
    max_history_capacity: usize,
    do_not_disturb: bool,
}

impl NotificationCenter {
    /// Creates a new `NotificationCenter`
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_history_capacity: 100,
            do_not_disturb: false,
        }
    }

    /// Enables or disables Do Not Disturb mode
    pub fn set_do_not_disturb(&mut self, dnd: bool) {
        self.do_not_disturb = dnd;
    }

    /// Returns whether Do Not Disturb mode is active
    pub fn is_dnd_active(&self) -> bool {
        self.do_not_disturb
    }

    /// Posts a notification to the system queue
    pub fn post(&mut self, notification: Notification) {
        // Critical alerts bypass Do Not Disturb
        if self.do_not_disturb && notification.level != NotificationLevel::Critical {
            return;
        }

        if self.queue.len() >= self.max_history_capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(notification);
    }

    /// Posts a quick notification with helper arguments
    pub fn post_quick(&mut self, title: &str, body: &str, level: NotificationLevel, source: &str) -> String {
        let id = format!("notif_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0));

        let notif = Notification::new(id.clone(), title, body, level, source);
        self.post(notif);
        id
    }

    /// Marks a notification as read
    pub fn mark_read(&mut self, id: &str) -> bool {
        if let Some(n) = self.queue.iter_mut().find(|n| n.id == id) {
            n.is_read = true;
            true
        } else {
            false
        }
    }

    /// Returns unread notifications
    pub fn unread_notifications(&self) -> Vec<&Notification> {
        self.queue.iter().filter(|n| !n.is_read).collect()
    }

    /// Returns all notifications
    pub fn all_notifications(&self) -> &VecDeque<Notification> {
        &self.queue
    }

    /// Clears all non-critical notifications
    pub fn clear_all(&mut self) {
        self.queue.retain(|n| n.level == NotificationLevel::Critical && !n.is_read);
    }
}
