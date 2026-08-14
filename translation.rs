use serde::{Deserialize, Serialize};

/// Supported languages for LensAI translation module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    French,
    German,
    Japanese,
    Chinese,
    Korean,
    Custom(String),
}

impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Japanese => "ja",
            Language::Chinese => "zh",
            Language::Korean => "ko",
            Language::Custom(code) => code.as_str(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Spanish => "Spanish",
            Language::French => "French",
            Language::German => "German",
            Language::Japanese => "Japanese",
            Language::Chinese => "Chinese (Simplified)",
            Language::Korean => "Korean",
            Language::Custom(name) => name.as_str(),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// Stylistic tone for output translation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranslationTone {
    Standard,
    Formal,
    Casual,
    Technical,
}

impl Default for TranslationTone {
    fn default() -> Self {
        TranslationTone::Standard
    }
}

/// Options configuring translation requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationOptions {
    pub source_lang: Option<Language>,
    pub target_lang: Language,
    pub tone: TranslationTone,
    pub preserve_formatting: bool,
}

impl Default for TranslationOptions {
    fn default() -> Self {
        Self {
            source_lang: None,
            target_lang: Language::English,
            tone: TranslationTone::Standard,
            preserve_formatting: true,
        }
    }
}

/// Result produced by translation execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub original_text: String,
    pub translated_text: String,
    pub detected_source: Language,
    pub target_language: Language,
    pub confidence: f32,
}

/// LensAI Translation service module.
pub struct Translator {
    cache_enabled: bool,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            cache_enabled: true,
        }
    }

    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }

    pub fn translate(&self, text: &str, options: TranslationOptions) -> Result<TranslationResult, String> {
        if text.trim().is_empty() {
            return Err("Input text for translation cannot be empty.".to_string());
        }

        let detected = options.source_lang.clone().unwrap_or_else(|| self.detect_language(text));

        // Translation logic simulation for LensAI
        let translated = format!(
            "[{}] -> [{}] ({:?} tone): {}",
            detected.display_name(),
            options.target_lang.display_name(),
            options.tone,
            text
        );

        Ok(TranslationResult {
            original_text: text.to_string(),
            translated_text: translated,
            detected_source: detected,
            target_language: options.target_lang,
            confidence: 0.96,
        })
    }

    pub fn detect_language(&self, text: &str) -> Language {
        let text_lower = text.to_lowercase();
        if text_lower.chars().any(|c| ('\u{3040}'..='\u{30ff}').contains(&c)) {
            Language::Japanese
        } else if text_lower.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)) {
            Language::Chinese
        } else if text_lower.chars().any(|c| ('\u{ac00}'..='\u{d7af}').contains(&c)) {
            Language::Korean
        } else if text_lower.contains("el ") || text_lower.contains("la ") || text_lower.contains("gracias") {
            Language::Spanish
        } else if text_lower.contains("le ") || text_lower.contains("la ") || text_lower.contains("bonjour") {
            Language::French
        } else if text_lower.contains("das ") || text_lower.contains("der ") || text_lower.contains("danke") {
            Language::German
        } else {
            Language::English
        }
    }

    pub fn supported_languages(&self) -> Vec<Language> {
        vec![
            Language::English,
            Language::Spanish,
            Language::French,
            Language::German,
            Language::Japanese,
            Language::Chinese,
            Language::Korean,
        ]
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}
