//! Module de gestion de configuration
//! Charge et sauvegarde la configuration depuis config.json
use crate::launcher::Launcher;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: String,
    pub theme: String,
    pub autostart: bool,
    pub launchers: Vec<Launcher>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn add_launcher(&mut self, launcher: Launcher) {
        self.launchers.push(launcher);
    }

    pub fn remove_launcher(&mut self, id: &str) {
        self.launchers.retain(|l| l.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launcher::LaunchType;

    #[test]
    fn test_config_creation() {
        let config = Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![],
        };
        assert_eq!(config.launchers.len(), 0);
    }

    #[test]
    fn test_add_launcher() {
        let mut config = Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![],
        };

        let launcher = Launcher {
            id: "test".to_string(),
            name: "Test".to_string(),
            launch_type: LaunchType::Web,
            target: "https://example.com".to_string(),
            icon: Some("icon.png".to_string()),
            options: None,
        };

        config.add_launcher(launcher);
        assert_eq!(config.launchers.len(), 1);
    }

    #[test]
    fn test_remove_launcher() {
        let mut config = Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![],
        };

        let launcher = Launcher {
            id: "test".to_string(),
            name: "Test".to_string(),
            launch_type: LaunchType::Web,
            target: "https://example.com".to_string(),
            icon: Some("icon.png".to_string()),
            options: None,
        };

        config.add_launcher(launcher);
        assert_eq!(config.launchers.len(), 1);

        config.remove_launcher("test");
        assert_eq!(config.launchers.len(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let config = Config {
            version: "0.1.0".to_string(),
            theme: "light".to_string(),
            autostart: false,
            launchers: vec![],
        };

        let path = "test_config.json";
        config.save(path).expect("Failed to save");

        let loaded = Config::load(path).expect("Failed to load");
        assert_eq!(loaded.version, "0.1.0");

        std::fs::remove_file(path).ok();
    }
}
