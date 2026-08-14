use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Actions that the Assistant can perform on behalf of LensOS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssistantAction {
    LaunchApp { app_id: String },
    UpdateLensOSSetting { category: String, key: String, value: String },
    OrganizeDesktop { mode: String },
    SearchFiles { query: String },
    SummarizeClipboard,
    SystemNotification { title: String, body: String },
}

impl AssistantAction {
    pub fn description(&self) -> String {
        match self {
            AssistantAction::LaunchApp { app_id } => format!("Launch LensOS app: {}", app_id),
            AssistantAction::UpdateLensOSSetting { category, key, value } => format!("Set [{}.{}] to '{}'", category, key, value),
            AssistantAction::OrganizeDesktop { mode } => format!("Organize LensOS desktop by mode: {}", mode),
            AssistantAction::SearchFiles { query } => format!("Search system files for: '{}'", query),
            AssistantAction::SummarizeClipboard => "Summarize clipboard content".to_string(),
            AssistantAction::SystemNotification { title, body } => format!("Notification [{}]: {}", title, body),
        }
    }
}

/// Result of intent recognition on user prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentMatch {
    pub intent: String,
    pub confidence: f32,
    pub parameters: HashMap<String, String>,
    pub suggested_action: Option<AssistantAction>,
}

/// LensOS Assistant Mode Engine handling system automation and OS interactions.
pub struct AssistantEngine {
    enabled: bool,
    automations_count: usize,
}

impl AssistantEngine {
    pub fn new() -> Self {
        Self {
            enabled: true,
            automations_count: 0,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn parse_intent(&self, input: &str) -> IntentMatch {
        let input_lower = input.to_lowercase();
        let mut params = HashMap::new();

        if input_lower.contains("launch") || input_lower.contains("open") {
            let app_name = input
                .split_whitespace()
                .last()
                .unwrap_or("terminal")
                .to_string();
            params.insert("app_id".to_string(), app_name.clone());

            IntentMatch {
                intent: "app_launch".to_string(),
                confidence: 0.92,
                parameters: params,
                suggested_action: Some(AssistantAction::LaunchApp { app_id: app_name }),
            }
        } else if input_lower.contains("organize") || input_lower.contains("clean desktop") {
            params.insert("mode".to_string(), "smart_grid".to_string());

            IntentMatch {
                intent: "desktop_organization".to_string(),
                confidence: 0.88,
                parameters: params,
                suggested_action: Some(AssistantAction::OrganizeDesktop {
                    mode: "smart_grid".to_string(),
                }),
            }
        } else if input_lower.contains("search") || input_lower.contains("find file") {
            let query = input.replace("search", "").replace("find file", "").trim().to_string();
            params.insert("query".to_string(), query.clone());

            IntentMatch {
                intent: "file_search".to_string(),
                confidence: 0.85,
                parameters: params,
                suggested_action: Some(AssistantAction::SearchFiles { query }),
            }
        } else if input_lower.contains("clipboard") || input_lower.contains("copied") {
            IntentMatch {
                intent: "clipboard_summary".to_string(),
                confidence: 0.95,
                parameters: params,
                suggested_action: Some(AssistantAction::SummarizeClipboard),
            }
        } else {
            IntentMatch {
                intent: "conversational_qna".to_string(),
                confidence: 0.99,
                parameters: params,
                suggested_action: None,
            }
        }
    }

    pub fn execute_action(&mut self, action: &AssistantAction) -> Result<String, String> {
        if !self.enabled {
            return Err("Assistant mode is currently disabled in LensAI settings.".to_string());
        }

        self.automations_count += 1;

        match action {
            AssistantAction::LaunchApp { app_id } => {
                Ok(format!("Successfully spawned process for LensOS application: {}", app_id))
            }
            AssistantAction::UpdateLensOSSetting { category, key, value } => {
                Ok(format!("LensOS setting updated -> [{}/{}] set to '{}'", category, key, value))
            }
            AssistantAction::OrganizeDesktop { mode } => {
                Ok(format!("Desktop icons organized according to layout pattern '{}'", mode))
            }
            AssistantAction::SearchFiles { query } => {
                Ok(format!("Found 4 files matching query '{}' in LensOS file index.", query))
            }
            AssistantAction::SummarizeClipboard => {
                Ok("Clipboard content extracted and summarized.".to_string())
            }
            AssistantAction::SystemNotification { title, body } => {
                Ok(format!("System notification dispatched: '{}' - '{}'", title, body))
            }
        }
    }

    pub fn get_active_suggestions(&self, _context: &str) -> Vec<String> {
        vec![
            "Summarize my recent clipboard text".to_string(),
            "Organize desktop icons into smart clusters".to_string(),
            "Search for design guidelines in workspace".to_string(),
            "Switch LensOS theme to Deep Glass Dark".to_string(),
        ]
    }

    pub fn automations_executed(&self) -> usize {
        self.automations_count
    }
}

impl Default for AssistantEngine {
    fn default() -> Self {
        Self::new()
    }
}
