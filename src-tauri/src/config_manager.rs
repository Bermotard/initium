//! Configuration Manager Module
//!
//! Handles persistent configuration with auto-save
//! Supports platform-specific storage locations

use crate::config::Config;
use crate::launcher::Launcher;
use std::path::{Path, PathBuf};

/// Configuration manager with auto-save
pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    /// Get platform-specific config directory
    #[cfg(target_os = "linux")]
    fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("initium")
    }

    #[cfg(target_os = "windows")]
    fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("initium")
    }

    #[cfg(target_os = "macos")]
    fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("initium")
    }

    /// Get full config file path
    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir().join("config.json")
    }

    /// Get icons directory
    pub fn get_icons_dir() -> PathBuf {
        Self::get_config_dir().join("icons")
    }

    /// Get settings directory
    pub fn get_settings_dir() -> PathBuf {
        Self::get_config_dir().join("settings")
    }

    /// Create all necessary directories
    fn create_directories() -> Result<(), String> {
        let config_dir = Self::get_config_dir();
        let icons_dir = Self::get_icons_dir();
        let settings_dir = Self::get_settings_dir();

        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
        std::fs::create_dir_all(&icons_dir)
            .map_err(|e| format!("Failed to create icons directory: {}", e))?;
        std::fs::create_dir_all(&settings_dir)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;

        log::info!("Config directories created at: {}", config_dir.display());
        Ok(())
    }

    /// Create default launcher from file
    fn create_default_launcher() -> Launcher {
        let initial_launcher_json = include_str!("../initial-launcher.json");
        match serde_json::from_str::<Launcher>(initial_launcher_json) {
            Ok(launcher) => launcher,
            Err(_) => {
                log::warn!("Failed to parse initial-launcher.json, using fallback");
                Launcher::new(
                    "rhone_digital".to_string(),
                    "Rhône Digital".to_string(),
                    crate::launcher::LaunchType::Web,
                    "http://www.rhone-digital.fr".to_string(),
                )
            }
        }
    }

    /// Create default configuration with initial launcher
    fn default_config() -> Config {
        Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![Self::create_default_launcher()],
            background: None,
            language: "en".to_string(),
        }
    }

    /// Load configuration or create default if not exists
    pub fn load_or_default() -> Result<Self, String> {
        Self::create_directories()?;
        let config_path = Self::get_config_path();
        
        let config = if config_path.exists() {
            Config::load(&config_path)
                .map_err(|e| format!("Failed to load config: {}", e))?
        } else {
            let default = Self::default_config();
            default.save(&config_path)
                .map_err(|e| format!("Failed to save default config: {}", e))?;
            default
        };
        
        Ok(ConfigManager { config_path, config })
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<(), String> {
        self.config
            .save(&self.config_path)
            .map_err(|e| format!("Failed to save config: {}", e))
    }

    /// Add launcher with auto-save
    pub fn add_launcher(&mut self, launcher: Launcher) -> Result<(), String> {
        self.config.add_launcher(launcher);
        self.save()?;
        log::info!("Launcher added and config saved");
        Ok(())
    }

    /// Remove launcher with auto-save
    pub fn remove_launcher(&mut self, id: &str) -> Result<(), String> {
        self.config.remove_launcher(id);
        self.save()?;
        log::info!("Launcher removed and config saved");
        Ok(())
    }

    /// Get config reference
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get mutable config reference
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Get config path
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// Get language setting
    pub fn get_language(&self) -> String {
        self.config.language.clone()
    }

    /// Set language with auto-save
    pub fn set_language(&mut self, language: String) -> Result<(), String> {
        self.config.language = language;
        self.save()?;
        log::info!("Language setting updated");
        Ok(())
    }

    /// Reset all settings to default values
    pub fn reset_settings(&mut self) -> Result<(), String> {
        self.config.language = "en".to_string();
        self.config.background = None;
        self.save()?;
        log::info!("Settings reset to default values");
        Ok(())
    }

    /// Save all settings at once
    pub fn save_all_settings(&mut self, language: String, background: Option<String>) -> Result<(), String> {
        self.config.language = language;
        self.config.background = background;
        self.save()?;
        log::info!("All settings saved");
        Ok(())
    }

    /// Get config directory path
    pub fn get_config_dir_path() -> PathBuf {
        Self::get_config_dir()
    }

    /// Get icons directory path
    pub fn get_icons_dir_path() -> PathBuf {
        Self::get_icons_dir()
    }

    /// Get settings directory path
    pub fn get_settings_dir_path() -> PathBuf {
        Self::get_settings_dir()
    }

    pub fn export_to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Export failed: {}", e))
    }

    pub fn import_from_json(json: &str) -> Result<Self, String> {
        let config: Config = serde_json::from_str(json)
            .map_err(|e| format!("Import failed: {}", e))?;
        Ok(ConfigManager {
            config_path: Self::get_config_path(),
            config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::LaunchType;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn cleanup_test_config() {
        let path = ConfigManager::get_config_path();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_config_path_exists() {
        let _guard = TEST_LOCK.lock().unwrap();
        let path = ConfigManager::get_config_path();
        assert!(path.to_string_lossy().contains("initium"));
        assert!(path.to_string_lossy().contains("config.json"));
    }

    #[test]
    fn test_config_path_platform_specific() {
        let _guard = TEST_LOCK.lock().unwrap();
        let path = ConfigManager::get_config_path();

        #[cfg(target_os = "linux")]
        assert!(path.to_string_lossy().contains(".config"));

        #[cfg(target_os = "windows")]
        assert!(path.to_string_lossy().contains("initium"));

        #[cfg(target_os = "macos")]
        assert!(path.to_string_lossy().contains("initium"));
    }

    #[test]
    fn test_load_or_create_default() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let manager = ConfigManager::load_or_default().expect("Failed to load or create default config");
        assert_eq!(manager.config().version, "0.1.0");
        assert_eq!(manager.config().theme, "light");
        assert_eq!(manager.config().language, "en");
        
        cleanup_test_config();
    }

    #[test]
    fn test_get_language() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let manager = ConfigManager::load_or_default().expect("Failed to load");
        assert_eq!(manager.get_language(), "en");

        cleanup_test_config();
    }

    #[test]
    fn test_set_language() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let mut manager = ConfigManager::load_or_default().expect("Failed to load");
        manager.set_language("fr".to_string()).expect("Failed to set language");
        assert_eq!(manager.get_language(), "fr");

        cleanup_test_config();
    }

    #[test]
    fn test_auto_save_on_add_launcher() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let mut manager = ConfigManager::load_or_default().expect("Failed to load config");

        let launcher = Launcher {
            id: "test_add_12345".to_string(),
            name: "Test".to_string(),
            launch_type: LaunchType::Web,
            target: "https://example.com".to_string(),
            icon: Some("icon.png".to_string()),
            options: None,
        };

        manager.add_launcher(launcher).expect("Failed to add launcher");
        assert!(manager.config().launchers.iter().any(|l| l.id == "test_add_12345"));

        cleanup_test_config();
    }

    #[test]
    fn test_auto_save_on_remove_launcher() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let mut manager = ConfigManager::load_or_default().expect("Failed to load config");

        let launcher = Launcher {
            id: "test_remove_12345".to_string(),
            name: "Test".to_string(),
            launch_type: LaunchType::App,
            target: "sh".to_string(),
            icon: None,
            options: None,
        };

        manager.add_launcher(launcher).expect("Failed to add launcher");
        assert!(manager.config().launchers.iter().any(|l| l.id == "test_remove_12345"));

        manager.remove_launcher("test_remove_12345").expect("Failed to remove launcher");
        assert!(!manager.config().launchers.iter().any(|l| l.id == "test_remove_12345"));

        cleanup_test_config();
    }

    #[test]
    fn test_persist_across_reload() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();
        
        let config_dir = ConfigManager::get_config_dir();
        std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        
        let mut manager = ConfigManager::load_or_default().expect("Failed to load config");
        let launcher = Launcher::new(
            "persist_test_12345".to_string(),
            "Persist Test".to_string(),
            LaunchType::App,
            "test_app".to_string(),
        );
        manager.add_launcher(launcher).expect("Failed to add launcher");
        
        let config_path = ConfigManager::get_config_path();
        assert!(config_path.exists());
        
        let reloaded = ConfigManager::load_or_default().expect("Failed to reload config");
        assert!(reloaded.config().launchers.iter().any(|l| l.id == "persist_test_12345"));
        
        cleanup_test_config();
    }

    #[test]
    fn test_config_directory_created() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let _manager = ConfigManager::load_or_default().expect("Failed to load config");
        let dir = ConfigManager::get_config_dir();
        assert!(dir.exists());

        cleanup_test_config();
    }

    #[test]
    fn test_all_directories_created() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let _manager = ConfigManager::load_or_default().expect("Failed to load config");

        let config_dir = ConfigManager::get_config_dir();
        let icons_dir = ConfigManager::get_icons_dir();
        let settings_dir = ConfigManager::get_settings_dir();

        assert!(config_dir.exists());
        assert!(icons_dir.exists());
        assert!(settings_dir.exists());

        cleanup_test_config();
    }

    #[test]
    fn test_export_to_json() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();
        
        let mut manager = ConfigManager::load_or_default().expect("Failed to load");
        
        let launcher = Launcher {
            id: "export_test".to_string(),
            name: "Export Test".to_string(),
            launch_type: LaunchType::App,
            target: "/bin/app".to_string(),
            icon: None,
            options: None,
        };
        
        manager.add_launcher(launcher).expect("Failed to add");
        manager.save().expect("Failed to save");
        
        let json = manager.export_to_json().expect("Failed to export");
        assert!(json.contains("export_test"));
        assert!(json.contains("Export Test"));
        
        cleanup_test_config();
    }

    #[test]
    fn test_import_from_json() {
        let json = r#"{
            "version": "0.1.0",
            "theme": "light",
            "autostart": false,
            "launchers": [
                {
                    "id": "import_test",
                    "name": "Import Test",
                    "type": "app",
                    "target": "/bin/app",
                    "icon": null,
                    "options": null
                }
            ]
        }"#;
    
        let manager = ConfigManager::import_from_json(json).expect("Failed to import");
        assert_eq!(manager.config().launchers.len(), 1);
        assert_eq!(manager.config().launchers[0].id, "import_test");
    }

    #[test]
    fn test_default_launcher_created() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let manager = ConfigManager::load_or_default().expect("Failed to load");
        assert_eq!(manager.config().launchers.len(), 1);
        assert_eq!(manager.config().launchers[0].id, "rhone_digital");

        cleanup_test_config();
    }

    #[test]
    fn test_reset_settings() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let mut manager = ConfigManager::load_or_default().expect("Failed to load");
        manager.set_language("fr".to_string()).expect("Failed to set language");
        
        assert_eq!(manager.get_language(), "fr");
        
        manager.reset_settings().expect("Failed to reset");
        
        assert_eq!(manager.get_language(), "en");
        assert!(manager.config().background.is_none());

        cleanup_test_config();
    }

    #[test]
    fn test_save_all_settings() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let mut manager = ConfigManager::load_or_default().expect("Failed to load");
        
        manager.save_all_settings(
            "fr".to_string(),
            Some("gradient2".to_string())
        ).expect("Failed to save all");
        
        assert_eq!(manager.get_language(), "fr");
        assert_eq!(manager.config().background, Some("gradient2".to_string()));

        cleanup_test_config();
    }
}