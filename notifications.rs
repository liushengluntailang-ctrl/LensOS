//! Notification Center system module for LensOS.
//!
//! Handles desktop toast notifications, notification history, urgency levels,
//! action callbacks, auto-dismiss timers, and Do Not Disturb (DND) filtering.

/// Urgency level for desktop notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

impl NotificationUrgency {
    pub fn display_name(&self) -> &'static str {
        match self {
            NotificationUrgency::Low => "Low",
            NotificationUrgency::Normal => "Normal",
            NotificationUrgency::Critical => "Critical",
        }
    }
}

/// Action button payload attached to a notification.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
}

/// Individual desktop notification record.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub app_name: String,
    pub app_icon: String,
    pub urgency: NotificationUrgency,
    pub timestamp_secs: f64,
    pub read: bool,
    pub dismissed: bool,
    pub actions: Vec<NotificationAction>,
    pub timeout_secs: f32,
    pub elapsed_secs: f32,
}

impl Notification {
    pub fn new(id: u64, title: &str, body: &str, app_name: &str, urgency: NotificationUrgency) -> Self {
        let timeout = match urgency {
            NotificationUrgency::Low => 3.5,
            NotificationUrgency::Normal => 5.0,
            NotificationUrgency::Critical => 10.0,
        };

        Self {
            id,
            title: title.to_string(),
            body: body.to_string(),
            app_name: app_name.to_string(),
            app_icon: "bell".to_string(),
            urgency,
            timestamp_secs: 0.0,
            read: false,
            dismissed: false,
            actions: Vec::new(),
            timeout_secs: timeout,
            elapsed_secs: 0.0,
        }
    }

    /// Adds an interactive button action to the notification.
    pub fn with_action(mut self, action_id: &str, label: &str) -> Self {
        self.actions.push(NotificationAction {
            id: action_id.to_string(),
            label: label.to_string(),
        });
        self
    }
}

/// Central Notification Manager and Sidebar Panel engine.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationCenter {
    pub is_open: bool,
    pub notifications: Vec<Notification>,
    pub do_not_disturb: bool,
    next_id: u64,
    pub total_lifetime_count: u64,
}

impl NotificationCenter {
    /// Creates a new NotificationCenter instance.
    pub fn new() -> Self {
        Self {
            is_open: false,
            notifications: Vec::new(),
            do_not_disturb: false,
            next_id: 1,
            total_lifetime_count: 0,
        }
    }

    /// Toggles sidebar notification panel.
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.mark_all_read();
        }
    }

    /// Opens sidebar panel.
    pub fn open(&mut self) {
        self.is_open = true;
        self.mark_all_read();
    }

    /// Closes sidebar panel.
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Toggles Do Not Disturb mode.
    pub fn toggle_dnd(&mut self) -> bool {
        self.do_not_disturb = !self.do_not_disturb;
        self.do_not_disturb
    }

    /// Dispatches a new notification to LensOS desktop.
    pub fn send(
        &mut self,
        title: &str,
        body: &str,
        app_name: &str,
        urgency: NotificationUrgency,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.total_lifetime_count += 1;

        let notif = Notification::new(id, title, body, app_name, urgency);

        // If DND is enabled, suppress low/normal urgency toasts into background log
        if !self.do_not_disturb || urgency == NotificationUrgency::Critical {
            self.notifications.push(notif);
        } else {
            let mut muted_notif = notif;
            muted_notif.dismissed = true; // Stored in panel log but toast skipped
            self.notifications.push(muted_notif);
        }

        id
    }

    /// Dismisses a notification toast by ID.
    pub fn dismiss(&mut self, id: u64) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.dismissed = true;
        }
    }

    /// Clears all historical notifications.
    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }

    /// Marks all notifications as read.
    pub fn mark_all_read(&mut self) {
        for n in self.notifications.iter_mut() {
            n.read = true;
        }
    }

    /// Returns unread notification count badge number.
    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    /// Returns active toast notifications currently popping up on screen.
    pub fn active_toasts(&self) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|n| !n.dismissed && n.elapsed_secs < n.timeout_secs)
            .collect()
    }

    /// Clock tick update to increment auto-dismiss timers.
    pub fn update(&mut self, delta_time_secs: f32) {
        for n in self.notifications.iter_mut() {
            if !n.dismissed {
                n.elapsed_secs += delta_time_secs;
                if n.elapsed_secs >= n.timeout_secs {
                    n.dismissed = true;
                }
            }
        }
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}
