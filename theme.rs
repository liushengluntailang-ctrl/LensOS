use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub is_built_in: bool,
    pub primary_accent: String,
    pub secondary_accent: String,
    pub background_dark: String,
    pub card_glass_background: String,
    pub card_glass_opacity: f32,
    pub card_border_color: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
}

impl Theme {
    pub fn lensos_dark_default() -> Self {
        Self {
            id: "lensos_dark".to_string(),
            name: "LensOS Frosted Dark".to_string(),
            description: "Default LensOS dark theme with deep obsidian glass and cyan accents"
                .to_string(),
            author: "LensOS Core Team".to_string(),
            is_built_in: true,
            primary_accent: "#00B4D8".to_string(),
            secondary_accent: "#7209B7".to_string(),
            background_dark: "#0F111A".to_string(),
            card_glass_background: "rgba(255, 255, 255, 0.06)".to_string(),
            card_glass_opacity: 0.12,
            card_border_color: "rgba(255, 255, 255, 0.12)".to_string(),
            text_primary: "#F8FAFC".to_string(),
            text_secondary: "#94A3B8".to_string(),
            text_muted: "#64748B".to_string(),
        }
    }

    pub fn lensos_cyber_neon() -> Self {
        Self {
            id: "lensos_cyber".to_string(),
            name: "Cyber Neon Glass".to_string(),
            description: "High-contrast theme with vivid electric purple and magenta accents"
                .to_string(),
            author: "LensOS Core Team".to_string(),
            is_built_in: true,
            primary_accent: "#F72585".to_string(),
            secondary_accent: "#4CC9F0".to_string(),
            background_dark: "#080711".to_string(),
            card_glass_background: "rgba(247, 37, 133, 0.08)".to_string(),
            card_glass_opacity: 0.15,
            card_border_color: "rgba(247, 37, 133, 0.25)".to_string(),
            text_primary: "#FFFFFF".to_string(),
            text_secondary: "#E2E8F0".to_string(),
            text_muted: "#A0AEC0".to_string(),
        }
    }

    pub fn frosted_light() -> Self {
        Self {
            id: "frosted_light".to_string(),
            name: "Frosted Light".to_string(),
            description: "Clean, high-legibility light mode with soft glass shadows".to_string(),
            author: "LensOS Core Team".to_string(),
            is_built_in: true,
            primary_accent: "#0284C7".to_string(),
            secondary_accent: "#4F46E5".to_string(),
            background_dark: "#F8FAFC".to_string(),
            card_glass_background: "rgba(255, 255, 255, 0.75)".to_string(),
            card_glass_opacity: 0.85,
            card_border_color: "rgba(0, 0, 0, 0.08)".to_string(),
            text_primary: "#0F172A".to_string(),
            text_secondary: "#475569".to_string(),
            text_muted: "#94A3B8".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemeManager {
    pub active_theme_id: String,
    pub available_themes: Vec<Theme>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let default_theme = Theme::lensos_dark_default();
        let cyber_theme = Theme::lensos_cyber_neon();
        let light_theme = Theme::frosted_light();

        Self {
            active_theme_id: default_theme.id.clone(),
            available_themes: vec![default_theme, cyber_theme, light_theme],
        }
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_active_theme(&self) -> &Theme {
        self.available_themes
            .iter()
            .find(|t| t.id == self.active_theme_id)
            .unwrap_or_else(|| &self.available_themes[0])
    }

    pub fn set_active_theme(&mut self, theme_id: &str) -> Result<&Theme, String> {
        if self.available_themes.iter().any(|t| t.id == theme_id) {
            self.active_theme_id = theme_id.to_string();
            Ok(self.get_active_theme())
        } else {
            Err(format!("Theme ID '{}' not found", theme_id))
        }
    }

    pub fn add_custom_theme(&mut self, theme: Theme) {
        if let Some(pos) = self.available_themes.iter().position(|t| t.id == theme.id) {
            self.available_themes[pos] = theme;
        } else {
            self.available_themes.push(theme);
        }
    }

    pub fn delete_custom_theme(&mut self, theme_id: &str) -> Result<(), String> {
        if let Some(pos) = self.available_themes.iter().position(|t| t.id == theme_id) {
            if self.available_themes[pos].is_built_in {
                return Err("Cannot delete built-in LensOS themes".to_string());
            }
            self.available_themes.remove(pos);
            if self.active_theme_id == theme_id {
                self.active_theme_id = "lensos_dark".to_string();
            }
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_defaults() {
        let tm = ThemeManager::default();
        assert_eq!(tm.active_theme_id, "lensos_dark");
        assert_eq!(tm.get_active_theme().name, "LensOS Frosted Dark");
    }

    #[test]
    fn test_set_active_theme() {
        let mut tm = ThemeManager::default();
        assert!(tm.set_active_theme("lensos_cyber").is_ok());
        assert_eq!(tm.get_active_theme().id, "lensos_cyber");
    }

    #[test]
    fn test_cannot_delete_builtin_theme() {
        let mut tm = ThemeManager::default();
        assert!(tm.delete_custom_theme("lensos_dark").is_err());
    }
}
