//! Language pack and localization management for LensOS v0.1.
//!
//! Language packs provide localized UI strings, date/time locale formatting,
//! font family fallback bindings, keyboard layout maps, and IME engine definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::installer::InstallerError;

/// Locale descriptor structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleInfo {
    /// BCP-47 language tag (e.g. "en-US", "ja-JP", "de-DE", "zh-CN").
    pub code: String,
    /// English display name.
    pub name_english: String,
    /// Native display name.
    pub name_native: String,
    /// Right-to-Left text flag (e.g., true for Arabic/Hebrew).
    pub is_rtl: bool,
}

/// Representation of a complete localized Language Pack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePack {
    /// Locale code string.
    pub locale: LocaleInfo,
    /// Version.
    pub version: String,
    /// Dictionary mapping translation keys to localized strings.
    pub string_table: HashMap<String, String>,
    /// Fallback font families required for rendering (e.g., "Noto Sans CJK").
    pub required_fonts: Vec<String>,
    /// Input Method Editor engine path if needed (e.g., "ime/mozc.so").
    pub ime_engine_entry: Option<String>,
}

impl LanguagePack {
    pub fn new_en_us() -> Self {
        let mut string_table = HashMap::new();
        string_table.insert("app.welcome".to_string(), "Welcome to LensOS".to_string());
        string_table.insert("installer.start".to_string(), "Start Installation".to_string());
        string_table.insert("installer.progress".to_string(), "Installing packages...".to_string());

        Self {
            locale: LocaleInfo {
                code: "en-US".to_string(),
                name_english: "English (United States)".to_string(),
                name_native: "English (US)".to_string(),
                is_rtl: false,
            },
            version: "0.1.0".to_string(),
            string_table,
            required_fonts: vec!["Plus Jakarta Sans".to_string()],
            ime_engine_entry: None,
        }
    }

    pub fn new_ja_jp() -> Self {
        let mut string_table = HashMap::new();
        string_table.insert("app.welcome".to_string(), "LensOSへようこそ".to_string());
        string_table.insert("installer.start".to_string(), "インストールを開始".to_string());
        string_table.insert("installer.progress".to_string(), "パッケージをインストール中...".to_string());

        Self {
            locale: LocaleInfo {
                code: "ja-JP".to_string(),
                name_english: "Japanese".to_string(),
                name_native: "日本語".to_string(),
                is_rtl: false,
            },
            version: "0.1.0".to_string(),
            string_table,
            required_fonts: vec!["Noto Sans CJK JP".to_string()],
            ime_engine_entry: Some("ime/mozc_jp.so".to_string()),
        }
    }
}

/// Summary info for language pack listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackInfo {
    pub locale_code: String,
    pub name_english: String,
    pub name_native: String,
    pub is_active: bool,
    pub total_keys: usize,
}

/// Manager for installing and activating language localization packs.
#[derive(Debug, Clone)]
pub struct LanguagePackManager {
    installed_packs: HashMap<String, LanguagePack>,
    active_locale: String,
}

impl Default for LanguagePackManager {
    fn default() -> Self {
        let en = LanguagePack::new_en_us();
        let ja = LanguagePack::new_ja_jp();

        let mut map = HashMap::new();
        let active = en.locale.code.clone();
        map.insert(en.locale.code.clone(), en);
        map.insert(ja.locale.code.clone(), ja);

        Self {
            installed_packs: map,
            active_locale: active,
        }
    }
}

impl LanguagePackManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Installs a new language pack.
    pub fn install_pack(&mut self, pack: LanguagePack) -> Result<String, InstallerError> {
        let code = pack.locale.code.clone();
        if code.trim().is_empty() {
            return Err(InstallerError::InvalidPackage(
                "Locale code cannot be empty".to_string(),
            ));
        }

        self.installed_packs.insert(code.clone(), pack);
        Ok(code)
    }

    /// Activates a language pack for system UI rendering.
    pub fn activate_language(&mut self, locale_code: &str) -> Result<(), InstallerError> {
        if self.installed_packs.contains_key(locale_code) {
            self.active_locale = locale_code.to_string();
            Ok(())
        } else {
            Err(InstallerError::PackageNotFound(format!(
                "Language pack for locale '{}' not found",
                locale_code
            )))
        }
    }

    /// Translates a string key using active language pack.
    pub fn translate<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(pack) = self.installed_packs.get(&self.active_locale) {
            if let Some(val) = pack.string_table.get(key) {
                return val.as_str();
            }
        }
        key
    }

    /// Lists installed language packs.
    pub fn list_installed(&self) -> Vec<LanguagePackInfo> {
        self.installed_packs
            .values()
            .map(|p| LanguagePackInfo {
                locale_code: p.locale.code.clone(),
                name_english: p.locale.name_english.clone(),
                name_native: p.locale.name_native.clone(),
                is_active: p.locale.code == self.active_locale,
                total_keys: p.string_table.len(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_pack_translation() {
        let mut manager = LanguagePackManager::new();

        assert_eq!(manager.translate("app.welcome"), "Welcome to LensOS");

        assert!(manager.activate_language("ja-JP").is_ok());
        assert_eq!(manager.translate("app.welcome"), "LensOSへようこそ");
    }
}
