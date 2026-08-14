pub mod ai;
pub mod assistant;
pub mod chat;
pub mod history;
pub mod image;
pub mod settings;
pub mod summarizer;
pub mod translation;
pub mod ui;

use crate::ai::{AIConfig, AIError, AIResponse, AIService};
use crate::assistant::{AssistantEngine, IntentMatch};
use crate::chat::{ChatSession, MessageRole};
use crate::history::{HistoryEntry, HistoryManager};
use crate::image::{ImageAnalysisResult, ImageProcessor};
use crate::settings::AISettings;
use crate::summarizer::{SummaryOptions, SummaryResult, Summarizer};
use crate::translation::{TranslationOptions, TranslationResult, Translator};
use crate::ui::{ActiveView, UIState};

/// Main central application struct for LensAI in LensOS.
pub struct LensAIApp {
    pub ai_service: AIService,
    pub chat_session: ChatSession,
    pub assistant: AssistantEngine,
    pub translator: Translator,
    pub summarizer: Summarizer,
    pub image_processor: ImageProcessor,
    pub settings: AISettings,
    pub history: HistoryManager,
    pub ui_state: UIState,
}

impl LensAIApp {
    /// Creates a new LensAIApp instance with default configuration.
    pub fn new() -> Self {
        let settings = AISettings::default();
        Self::with_settings(settings)
    }

    /// Creates a LensAIApp instance with custom settings.
    pub fn with_settings(settings: AISettings) -> Self {
        let ai_config = AIConfig {
            model: settings.default_model.clone(),
            temperature: settings.temperature,
            max_tokens: settings.max_tokens,
            system_instruction: Some(settings.system_prompt.clone()),
            ..Default::default()
        };

        let ai_service = AIService::new(ai_config);
        let chat_session = ChatSession::new("session_init", "New Conversation")
            .with_model(settings.default_model.clone());
        let assistant = AssistantEngine::new();
        let translator = Translator::new();
        let summarizer = Summarizer::new();
        let image_processor = ImageProcessor::new();
        let history = HistoryManager::new();
        let ui_state = UIState::new();

        Self {
            ai_service,
            chat_session,
            assistant,
            translator,
            summarizer,
            image_processor,
            settings,
            history,
            ui_state,
        }
    }

    /// Sends a user query to the AI engine and records conversation in active session.
    pub fn send_message(&mut self, text: &str) -> Result<AIResponse, AIError> {
        self.ui_state.is_processing = true;
        self.ui_state.set_status("LensAI Generating Response...");

        // Record user message
        self.chat_session.add_message(MessageRole::User, text);

        // Process generation via AI Service
        let response_result = self.ai_service.generate_text(text);

        match &response_result {
            Ok(response) => {
                // Record assistant response
                self.chat_session.add_message(MessageRole::Assistant, &response.text);

                if self.settings.auto_save_history {
                    let _ = self.history.save_session(&self.chat_session);
                }

                self.ui_state.set_status("Ready");
            }
            Err(err) => {
                self.ui_state.set_status(format!("Error: {}", err));
            }
        }

        self.ui_state.is_processing = false;
        response_result
    }

    /// Parses user prompt for system automation actions and executes if applicable.
    pub fn run_assistant_automation(&mut self, prompt: &str) -> Result<String, String> {
        let intent_match: IntentMatch = self.assistant.parse_intent(prompt);

        if let Some(action) = &intent_match.suggested_action {
            let result = self.assistant.execute_action(action)?;
            self.chat_session.add_message(
                MessageRole::System,
                format!("[Automation Action]: {}\nResult: {}", action.description(), result),
            );
            Ok(result)
        } else {
            Err("No system automation action recognized for prompt.".to_string())
        }
    }

    /// Translates text using the translation module.
    pub fn translate_text(
        &self,
        text: &str,
        options: TranslationOptions,
    ) -> Result<TranslationResult, String> {
        self.translator.translate(text, options)
    }

    /// Summarizes document or long text content.
    pub fn summarize_text(
        &self,
        text: &str,
        options: SummaryOptions,
    ) -> Result<SummaryResult, String> {
        self.summarizer.summarize(text, options)
    }

    /// Analyzes image bytes and returns vision metrics and OCR text.
    pub fn analyze_image(&self, bytes: &[u8]) -> Result<ImageAnalysisResult, String> {
        self.image_processor.analyze_image(bytes)
    }

    /// Switches the active view tab in the UI state.
    pub fn switch_view(&mut self, view: ActiveView) {
        self.ui_state.switch_view(view);
    }

    /// Saves the active chat session to history.
    pub fn save_current_session(&mut self) -> Result<(), String> {
        self.history.save_session(&self.chat_session)
    }

    /// Loads a chat session from history by session ID.
    pub fn load_session(&mut self, session_id: &str) -> Result<(), String> {
        if let Some(session) = self.history.get_session(session_id) {
            self.chat_session = session.clone();
            Ok(())
        } else {
            Err(format!("Session '{}' not found in history.", session_id))
        }
    }

    /// Lists history entries.
    pub fn list_history(&self) -> Vec<HistoryEntry> {
        self.history.list_entries()
    }

    /// Updates app settings and synchronizes across modules and LensOS desktop.
    pub fn update_settings(&mut self, settings: AISettings) -> Result<(), String> {
        self.ai_service.set_model(settings.default_model.clone());
        self.settings.sync_to_lens_os_desktop()?;
        self.settings = settings;
        Ok(())
    }

    /// Renders current UI frame layout specification.
    pub fn render_ui_frame(&self) -> String {
        self.ui_state.render_frame_spec()
    }
}

impl Default for LensAIApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lens_ai_app_initialization() {
        let app = LensAIApp::new();
        assert_eq!(app.ui_state.active_view, ActiveView::Chat);
        assert!(app.ai_service.health_check());
    }

    #[test]
    fn test_send_message_and_history() {
        let mut app = LensAIApp::new();
        let res = app.send_message("Hello LensAI");
        assert!(res.is_ok());
        assert_eq!(app.chat_session.message_count(), 2);
    }

    #[test]
    fn test_assistant_automation() {
        let mut app = LensAIApp::new();
        let res = app.run_assistant_automation("Launch terminal");
        assert!(res.is_ok());
    }

    #[test]
    fn test_translation_and_summarizer() {
        let app = LensAIApp::new();
        let trans = app.translate_text("Hello world", TranslationOptions::default());
        assert!(trans.is_ok());

        let sum = app.summarize_text("LensOS is a modern operating system with frosted glass design.", SummaryOptions::default());
        assert!(sum.is_ok());
    }
}
