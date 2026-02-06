//! Module de gestion de configuration
//! Charge et sauvegarde la configuration depuis config.json

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::launcher::Launcher;

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
