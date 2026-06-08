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

    /// Create default launcher from file (kept for backward compatibility)
    #[allow(dead_code)]
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

    /// Create default configuration from embedded config.json
    /// This is ONLY used for first-time installation when no config exists
    fn default_config() -> Config {
        // Load default config from embedded JSON
        let default_config_json = include_str!("../resources/config.json");
        match serde_json::from_str::<Config>(default_config_json) {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to parse default config: {}, using fallback", e);
                // Fallback to hardcoded config
                use crate::launcher::LaunchType;
                let mut config = Config {
                    version: "1.10.0".to_string(),
                    theme: "light".to_string(),
                    autostart: false,
                    launchers: vec![],
                    background: Some("fond.png".to_string()),
                    language: "fr".to_string(),
                };
                config.launchers.push(Launcher::new(
                    "rhone_digital".to_string(),
                    "Rhône Digital".to_string(),
                    LaunchType::Web,
                    "http://www.rhone-digital.fr".to_string(),
                ));
                config.launchers.push(Launcher::new(
                    "firefox".to_string(),
                    "Firefox".to_string(),
                    LaunchType::App,
                    "firefox".to_string(),
                ));
                config.launchers.push(Launcher::new(
                    "youtube".to_string(),
                    "YouTube".to_string(),
                    LaunchType::Web,
                    "https://www.youtube.com".to_string(),
                ));
                config
            }
        }
    }

    /// Copy default resources (fond.png, icons/) to user config directory
    /// IMPORTANT: Only copies files that don't already exist - NEVER overwrites user files
    /// Uses embedded resources via include_bytes! to ensure they're always available
    fn copy_default_resources() -> Result<(), String> {
        use std::fs;
        
        let config_dir = Self::get_config_dir();
        let icons_dir = Self::get_icons_dir();
        
        // Create icons directory if it doesn't exist
        fs::create_dir_all(&icons_dir)
            .map_err(|e| format!("Failed to create icons directory: {}", e))?;
        
        // Copy fond.png ONLY if it doesn't exist
        let fond_dst = config_dir.join("fond.png");
        if !fond_dst.exists() {
            // Use include_bytes! to embed the image directly
            let fond_bytes = include_bytes!("../resources/fond.png");
            fs::write(&fond_dst, fond_bytes)
                .map_err(|e| format!("Failed to write fond.png: {}", e))?;
            log::info!("Copied default background image");
        }
        
        // Copy ALL icons from embedded resources ONLY if they don't exist
        // This ensures icons are available even when the app is installed as a binary
        // We embed each icon individually using include_bytes!
        
        // List of icons to copy (must match files in src-tauri/resources/icons/)
        // Using &[u8] to handle different sized icons
        let icon_files: &[(&str, &[u8])] = &[
            ("1.png", include_bytes!("../resources/icons/1.png")),
            ("A.jpeg", include_bytes!("../resources/icons/A.jpeg")),
            ("claudeconsole.png", include_bytes!("../resources/icons/claudeconsole.png")),
            ("codage.png", include_bytes!("../resources/icons/codage.png")),
            ("deepseek.png", include_bytes!("../resources/icons/deepseek.png")),
            ("firefox.png", include_bytes!("../resources/icons/firefox.png")),
            ("fortuneo.png", include_bytes!("../resources/icons/fortuneo.png")),
            ("gitea.png", include_bytes!("../resources/icons/gitea.png")),
            ("gmail.png", include_bytes!("../resources/icons/gmail.png")),
            ("HibpLogo.svg", include_bytes!("../resources/icons/HibpLogo.svg")),
            ("icons8-google-drive-100.png", include_bytes!("../resources/icons/icons8-google-drive-100.png")),
            ("icons8-whatsapp-48.png", include_bytes!("../resources/icons/icons8-whatsapp-48.png")),
            ("openclaw.svg", include_bytes!("../resources/icons/openclaw.svg")),
            ("protonmail.png", include_bytes!("../resources/icons/protonmail.png")),
            ("youtube.png", include_bytes!("../resources/icons/youtube.png")),
        ];
        
        for (file_name, icon_bytes) in icon_files {
            let dst_path = icons_dir.join(file_name);
            if !dst_path.exists() {
                fs::write(&dst_path, icon_bytes)
                    .map_err(|e| format!("Failed to write icon {}: {}", file_name, e))?;
                log::info!("Copied default icon: {}", file_name);
            }
        }
        
        Ok(())
    }

    /// Load configuration or create default if not exists
    /// 
    /// IMPORTANT: This function will NEVER overwrite an existing config file.
    /// If the config file exists, it loads it. If it doesn't exist, it creates default.
    /// If the config exists but cannot be loaded, it returns an error WITHOUT modifying the file.
    /// This GUARANTEES user data is never lost.
    pub fn load_or_default() -> Result<Self, String> {
        Self::create_directories()?;
        let config_path = Self::get_config_path();
        
        // FIRST: Check if config exists
        let config_exists = config_path.exists();
        
        let config = if config_exists {
            // If config exists, ALWAYS try to load it - NEVER overwrite, NEVER create default
            Config::load(&config_path)
                .map_err(|e| format!("ERREUR CRITIQUE: Impossible de charger la configuration existante à {}: {}. \
\nNE MODIFIEZ PAS CE FICHIER! Votre configuration n'a PAS été écrasée. \
Corrigez le fichier manuellement ou faites une sauvegarde avant de relancer.", 
                    config_path.display(), e))?
        } else {
            // Only create default config if file DOES NOT exist
            Self::copy_default_resources()?;
            
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
        assert_eq!(manager.config().version, "1.10.0");
        assert_eq!(manager.config().theme, "light");
        assert_eq!(manager.get_language(), "fr");
        
        cleanup_test_config();
    }

    #[test]
    fn test_get_language() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let manager = ConfigManager::load_or_default().expect("Failed to load");
        assert_eq!(manager.get_language(), "fr");

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
        // Now we have Rhone Digital, firefox and youtube as default launchers
        assert_eq!(manager.config().launchers.len(), 3);
        let ids: Vec<&str> = manager.config().launchers.iter().map(|l| l.id.as_str()).collect();
        assert!(ids.contains(&"rhone_digital"));
        assert!(ids.contains(&"firefox"));
        assert!(ids.contains(&"youtube"));

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
#[cfg(test)]
mod test_default_config {
    use super::*;
    use std::sync::Mutex;
    
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    
    fn cleanup_test_config() {
        let path = ConfigManager::get_config_path();
        let _ = std::fs::remove_file(&path);
        let icons_dir = ConfigManager::get_icons_dir();
        let _ = std::fs::remove_dir_all(&icons_dir);
    }
    
    #[test]
    fn test_default_config_loads() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();
        
        // This should create the config with default resources
        let manager = ConfigManager::load_or_default().expect("Failed to load or create default config");
        
        // Check we have the expected launchers (3: rhone_digital, firefox, youtube)
        assert_eq!(manager.config().launchers.len(), 3);
        
        // Check launcher IDs
        let ids: Vec<&str> = manager.config().launchers.iter().map(|l| l.id.as_str()).collect();
        assert!(ids.contains(&"rhone_digital"));
        assert!(ids.contains(&"firefox"));
        assert!(ids.contains(&"youtube"));
        
        // Check background is set
        assert!(manager.config().background.is_some());
        assert!(manager.config().background.as_ref().unwrap().contains("fond.png"));
        
        // Check language
        assert_eq!(manager.get_language(), "fr");
        
        // Check icons were copied
        let icons_dir = ConfigManager::get_icons_dir();
        assert!(icons_dir.exists());
        assert!(icons_dir.join("firefox.png").exists());
        assert!(icons_dir.join("youtube.png").exists());
        
        cleanup_test_config();
    }
}
