use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: Theme,
    pub sidebar_collapsed: bool,
    pub sidebar_width: f64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            sidebar_collapsed: false,
            sidebar_width: 240.0,
        }
    }
}

pub struct AppState {
    pub settings: Mutex<AppSettings>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new(app: &tauri::App) -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = app.path().app_data_dir()?;
        std::fs::create_dir_all(&data_dir)?;
        let settings = Self::load_settings(&data_dir).unwrap_or_default();
        Ok(Self {
            settings: Mutex::new(settings),
            data_dir,
        })
    }

    fn settings_path(data_dir: &std::path::Path) -> PathBuf {
        data_dir.join("settings.json")
    }

    fn load_settings(data_dir: &std::path::Path) -> Option<AppSettings> {
        let path = Self::settings_path(data_dir);
        if path.exists() {
            let content = std::fs::read_to_string(&path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn save_settings(&self) -> Result<(), crate::error::AppError> {
        let path = Self::settings_path(&self.data_dir);
        let settings = self.settings.lock().map_err(|e| {
            crate::error::AppError::Message(format!("Lock error: {e}"))
        })?;
        let content = serde_json::to_string_pretty(&*settings)?;
        std::fs::write(&path, content)?;
        tracing::info!("Settings saved to {:?}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.theme, Theme::System);
        assert!(!settings.sidebar_collapsed);
        assert!((settings.sidebar_width - 240.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_theme_default() {
        assert_eq!(Theme::default(), Theme::System);
    }

    #[test]
    fn test_theme_serialization() {
        let json = serde_json::to_string(&Theme::Dark).unwrap();
        assert_eq!(json, "\"dark\"");
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Theme::Dark);
    }

    #[test]
    fn test_settings_serialization_roundtrip() {
        let settings = AppSettings {
            theme: Theme::Light,
            sidebar_collapsed: true,
            sidebar_width: 300.0,
        };
        let json = serde_json::to_string_pretty(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.theme, Theme::Light);
        assert!(deserialized.sidebar_collapsed);
        assert!((deserialized.sidebar_width - 300.0).abs() < f64::EPSILON);
    }
}
