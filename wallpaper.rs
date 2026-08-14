use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WallpaperFit {
    Fill,
    Fit,
    Stretch,
    Center,
    Tile,
    Span,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WallpaperItem {
    pub id: String,
    pub title: String,
    pub file_path: String,
    pub thumbnail_path: String,
    pub is_dynamic: bool,
    pub is_ai_generated: bool,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WallpaperSettings {
    pub current_wallpaper_id: String,
    pub current_path: String,
    pub fit_mode: WallpaperFit,
    pub slideshow_enabled: bool,
    pub slideshow_interval_secs: u64,
    pub slideshow_folder: Option<String>,
    pub blur_overlay: f32, // 0.0 = crisp, 1.0 = heavy blur for focus
    pub dim_in_dark_mode: bool,
    pub dim_level: f32, // 0.0 to 1.0
    pub ai_prompt: Option<String>,
    pub presets: Vec<WallpaperItem>,
}

impl Default for WallpaperSettings {
    fn default() -> Self {
        let presets = vec![
            WallpaperItem {
                id: "lens_aurora".to_string(),
                title: "LensOS Frosted Aurora".to_string(),
                file_path: "/system/wallpapers/frosted_aurora.png".to_string(),
                thumbnail_path: "/system/wallpapers/thumbs/frosted_aurora.png".to_string(),
                is_dynamic: true,
                is_ai_generated: false,
                category: "LensOS Minimal".to_string(),
            },
            WallpaperItem {
                id: "cyber_mesh".to_string(),
                title: "Cyber Gradient Mesh".to_string(),
                file_path: "/system/wallpapers/cyber_mesh.png".to_string(),
                thumbnail_path: "/system/wallpapers/thumbs/cyber_mesh.png".to_string(),
                is_dynamic: false,
                is_ai_generated: true,
                category: "AI Abstract".to_string(),
            },
            WallpaperItem {
                id: "glass_nebula".to_string(),
                title: "Glass Nebula Deep Dark".to_string(),
                file_path: "/system/wallpapers/glass_nebula.png".to_string(),
                thumbnail_path: "/system/wallpapers/thumbs/glass_nebula.png".to_string(),
                is_dynamic: true,
                is_ai_generated: false,
                category: "Space".to_string(),
            },
        ];

        Self {
            current_wallpaper_id: "lens_aurora".to_string(),
            current_path: "/system/wallpapers/frosted_aurora.png".to_string(),
            fit_mode: WallpaperFit::Fill,
            slideshow_enabled: false,
            slideshow_interval_secs: 1800, // 30 minutes
            slideshow_folder: None,
            blur_overlay: 0.1,
            dim_in_dark_mode: true,
            dim_level: 0.2,
            ai_prompt: Some("Frosted glass geometric shapes in cyan and deep purple".to_string()),
            presets,
        }
    }
}

impl WallpaperSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_wallpaper(&mut self, wallpaper_id: &str) -> Result<(), String> {
        if let Some(item) = self.presets.iter().find(|w| w.id == wallpaper_id) {
            self.current_wallpaper_id = item.id.clone();
            self.current_path = item.file_path.clone();
            Ok(())
        } else {
            Err(format!("Wallpaper with ID '{}' not found in presets", wallpaper_id))
        }
    }

    pub fn set_custom_path(&mut self, path: String, fit: WallpaperFit) {
        self.current_wallpaper_id = "custom_path".to_string();
        self.current_path = path;
        self.fit_mode = fit;
    }

    pub fn toggle_slideshow(&mut self, enabled: bool, interval_secs: Option<u64>) {
        self.slideshow_enabled = enabled;
        if let Some(interval) = interval_secs {
            self.slideshow_interval_secs = interval.max(60);
        }
    }

    pub fn add_preset(&mut self, item: WallpaperItem) {
        if !self.presets.iter().any(|p| p.id == item.id) {
            self.presets.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_defaults() {
        let wp = WallpaperSettings::default();
        assert_eq!(wp.current_wallpaper_id, "lens_aurora");
        assert_eq!(wp.presets.len(), 3);
    }

    #[test]
    fn test_set_wallpaper() {
        let mut wp = WallpaperSettings::default();
        assert!(wp.set_wallpaper("cyber_mesh").is_ok());
        assert_eq!(wp.current_wallpaper_id, "cyber_mesh");
        assert_eq!(wp.current_path, "/system/wallpapers/cyber_mesh.png");

        assert!(wp.set_wallpaper("non_existent").is_err());
    }

    #[test]
    fn test_slideshow() {
        let mut wp = WallpaperSettings::default();
        wp.toggle_slideshow(true, Some(3600));
        assert!(wp.slideshow_enabled);
        assert_eq!(wp.slideshow_interval_secs, 3600);
    }
}
