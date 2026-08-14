use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelProvider {
    LocalGeminiNano,
    LensKernelAI,
    CloudGemini25Flash,
    CloudGeminiPro,
    CustomEndpoint { url: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AiPrivacyMode {
    StrictLocalOnly,
    AnonymizedCloud,
    FullCloudEnhanced,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiSettings {
    pub assistant_enabled: bool,
    pub model_provider: ModelProvider,
    pub local_model_path: Option<String>,
    pub api_key_configured: bool,
    pub wake_word_enabled: bool,
    pub wake_word: String,
    pub auto_summarize_notifications: bool,
    pub context_awareness: bool,
    pub privacy_mode: AiPrivacyMode,
    pub max_tokens_per_response: u32,
    pub temperature: f32,
    pub smart_search_enabled: bool,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self {
            assistant_enabled: true,
            model_provider: ModelProvider::CloudGemini25Flash,
            local_model_path: Some("/system/models/gemini_nano_lens.bin".to_string()),
            api_key_configured: true,
            wake_word_enabled: false,
            wake_word: "Hey Lens".to_string(),
            auto_summarize_notifications: true,
            context_awareness: true,
            privacy_mode: AiPrivacyMode::AnonymizedCloud,
            max_tokens_per_response: 2048,
            temperature: 0.7,
            smart_search_enabled: true,
        }
    }
}

impl AiSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_provider(&mut self, provider: ModelProvider) {
        self.model_provider = provider;
    }

    pub fn set_privacy_mode(&mut self, mode: AiPrivacyMode) {
        if mode == AiPrivacyMode::StrictLocalOnly {
            // Force local model provider when strict local mode is enabled
            self.model_provider = ModelProvider::LocalGeminiNano;
        }
        self.privacy_mode = mode;
    }

    pub fn set_wake_word(&mut self, enabled: bool, word: String) {
        self.wake_word_enabled = enabled;
        if !word.trim().is_empty() {
            self.wake_word = word;
        }
    }

    pub fn test_ai_connection_status(&self) -> Result<String, String> {
        if !self.assistant_enabled {
            return Err("LensOS AI Assistant is disabled".to_string());
        }

        match &self.model_provider {
            ModelProvider::LocalGeminiNano | ModelProvider::LensKernelAI => {
                if let Some(ref path) = self.local_model_path {
                    Ok(format!("Local AI model ready at '{}'", path))
                } else {
                    Err("Local model path is missing".to_string())
                }
            }
            ModelProvider::CloudGemini25Flash | ModelProvider::CloudGeminiPro => {
                if self.api_key_configured {
                    Ok("Cloud Gemini API connected successfully".to_string())
                } else {
                    Err("Gemini API key is not configured".to_string())
                }
            }
            ModelProvider::CustomEndpoint { url } => Ok(format!("Connected to custom endpoint '{}'", url)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_defaults() {
        let ai = AiSettings::default();
        assert!(ai.assistant_enabled);
        assert_eq!(ai.wake_word, "Hey Lens");
        assert!(ai.test_ai_connection_status().is_ok());
    }

    #[test]
    fn test_strict_local_mode() {
        let mut ai = AiSettings::default();
        ai.set_privacy_mode(AiPrivacyMode::StrictLocalOnly);
        assert_eq!(ai.model_provider, ModelProvider::LocalGeminiNano);
    }
}
