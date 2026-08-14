use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Supported AI models within LensOS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIModel {
    GeminiPro,
    GeminiFlash,
    LensLocalUltra,
    LensLocalLite,
    Custom(String),
}

impl AIModel {
    pub fn name(&self) -> &str {
        match self {
            AIModel::GeminiPro => "Gemini 1.5 Pro",
            AIModel::GeminiFlash => "Gemini 1.5 Flash",
            AIModel::LensLocalUltra => "Lens Local Ultra 7B",
            AIModel::LensLocalLite => "Lens Local Lite 3B",
            AIModel::Custom(name) => name.as_str(),
        }
    }

    pub fn context_window(&self) -> usize {
        match self {
            AIModel::GeminiPro => 1_000_000,
            AIModel::GeminiFlash => 1_000_000,
            AIModel::LensLocalUltra => 32_768,
            AIModel::LensLocalLite => 8_192,
            AIModel::Custom(_) => 16_384,
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, AIModel::LensLocalUltra | AIModel::LensLocalLite)
    }
}

impl Default for AIModel {
    fn default() -> Self {
        AIModel::GeminiFlash
    }
}

/// Configuration settings for the AI provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub model: AIModel,
    pub api_key: Option<String>,
    pub endpoint: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub system_instruction: Option<String>,
    pub enable_safety_filters: bool,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            model: AIModel::default(),
            api_key: None,
            endpoint: "https://api.lensos.internal/v1/ai".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            system_instruction: Some("You are LensAI, an intelligent assistant integrated into LensOS with a refined dark glass aesthetic and system automation capabilities.".to_string()),
            enable_safety_filters: true,
        }
    }
}

/// Response returned from AI generation requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub finish_reason: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub latency_ms: u128,
    pub model_used: String,
}

/// Error types for AI operations.
#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIError {
    #[error("API Key is missing for remote model")]
    ApiKeyMissing,

    #[error("Network request failed: {0}")]
    NetworkError(String),

    #[error("AI generation failed: {0}")]
    GenerationFailed(String),

    #[error("Invalid prompt input: {0}")]
    InvalidPrompt(String),

    #[error("Rate limit exceeded. Retry after {0} seconds")]
    RateLimited(u64),

    #[error("Model '{0}' is currently unavailable")]
    ModelUnavailable(String),
}

/// Core AI Service engine managing model requests and responses.
pub struct AIService {
    config: AIConfig,
    active: bool,
}

impl AIService {
    pub fn new(config: AIConfig) -> Self {
        Self {
            config,
            active: true,
        }
    }

    pub fn config(&self) -> &AIConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: AIConfig) {
        self.config = config;
    }

    pub fn set_model(&mut self, model: AIModel) {
        self.config.model = model;
    }

    pub fn generate_text(&self, prompt: &str) -> Result<AIResponse, AIError> {
        if prompt.trim().is_empty() {
            return Err(AIError::InvalidPrompt("Prompt cannot be empty.".to_string()));
        }

        if !self.config.model.is_local() && self.config.api_key.is_none() {
            // If remote and no API key configured, return fallback or explicit key error
            // For LensOS internal engine fallback:
            if self.config.endpoint.contains("lensos.internal") {
                // Internal mock bridge
            } else {
                return Err(AIError::ApiKeyMissing);
            }
        }

        let start_time = std::time::Instant::now();

        // Synthetic response engine simulating LensOS AI integration
        let prompt_tokens = self.estimate_tokens(prompt);
        let completion_text = format!(
            "LensAI [{}] processed request: \"{}\". Ready for further LensOS context operations.",
            self.config.model.name(),
            if prompt.len() > 60 { &prompt[..60] } else { prompt }
        );
        let completion_tokens = self.estimate_tokens(&completion_text);
        let elapsed = start_time.elapsed().as_millis();

        Ok(AIResponse {
            text: completion_text,
            finish_reason: "STOP".to_string(),
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
            latency_ms: elapsed,
            model_used: self.config.model.name().to_string(),
        })
    }

    pub fn generate_stream(&self, prompt: &str) -> Result<Vec<String>, AIError> {
        if prompt.trim().is_empty() {
            return Err(AIError::InvalidPrompt("Prompt cannot be empty.".to_string()));
        }

        let full_text = format!(
            "LensAI streaming response for: {}",
            prompt
        );

        let chunks: Vec<String> = full_text
            .split_whitespace()
            .map(|word| format!("{} ", word))
            .collect();

        Ok(chunks)
    }

    pub fn estimate_tokens(&self, text: &str) -> usize {
        // Approximate token estimation: ~4 chars per token
        (text.len() as f64 / 4.0).ceil() as usize
    }

    pub fn health_check(&self) -> bool {
        self.active
    }
}

impl Default for AIService {
    fn default() -> Self {
        Self::new(AIConfig::default())
    }
}
