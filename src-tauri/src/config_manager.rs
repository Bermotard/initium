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

    /// Create default configuration
    fn default_config() -> Config {
        Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![],
        }
    }

    /// Load configuration or create default if not exists
    pub fn load_or_default() -> Result<Self, String> {
        let config_path = Self::get_config_path();
        let config_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
        
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::LaunchType;
    use std::sync::Mutex;

    // Lock global pour sérialiser les tests
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

        let manager =
            ConfigManager::load_or_default().expect("Failed to load or create default config");

        assert_eq!(manager.config().version, "0.1.0");
        assert_eq!(manager.config().theme, "light");
        
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

        manager
            .add_launcher(launcher)
            .expect("Failed to add launcher");

        assert!(manager
            .config()
            .launchers
            .iter()
            .any(|l| l.id == "test_add_12345"));

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

        manager
            .add_launcher(launcher)
            .expect("Failed to add launcher");

        assert!(manager
            .config()
            .launchers
            .iter()
            .any(|l| l.id == "test_remove_12345"));

        manager
            .remove_launcher("test_remove_12345")
            .expect("Failed to remove launcher");

        assert!(!manager
            .config()
            .launchers
            .iter()
            .any(|l| l.id == "test_remove_12345"));

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
        manager
            .add_launcher(launcher)
            .expect("Failed to add launcher");
        
        let config_path = ConfigManager::get_config_path();
        assert!(config_path.exists(), "Config file should exist after add_launcher");
        
        let reloaded = ConfigManager::load_or_default().expect("Failed to reload config");
        assert!(reloaded
            .config()
            .launchers
            .iter()
            .any(|l| l.id == "persist_test_12345"), 
            "Launcher should persist after reload");
        
        cleanup_test_config();
    }

    #[test]
    fn test_config_directory_created() {
        let _guard = TEST_LOCK.lock().unwrap();
        cleanup_test_config();

        let _manager = ConfigManager::load_or_default().expect("Failed to load config");

        let dir = ConfigManager::get_config_dir();
        assert!(dir.exists(), "Config directory should be created");

        cleanup_test_config();
    }
}
