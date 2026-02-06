//! Module de gestion des lanceurs
//! Responsable du lancement d'URLs et d'applications

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaunchType {
    Web,
    App,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub launch_type: LaunchType,
    pub target: String,
    pub icon: String,
}

pub async fn execute_launcher(launcher: &Launcher) -> Result<(), String> {
    match launcher.launch_type {
        LaunchType::Web => open_url(&launcher.target).await,
        LaunchType::App => execute_binary(&launcher.target).await,
    }
}

#[cfg(target_os = "linux")]
async fn open_url(url: &str) -> Result<(), String> {
    std::process::Command::new("xdg-open")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "windows")]
async fn open_url(url: &str) -> Result<(), String> {
    std::process::Command::new("cmd")
        .args(&["/C", "start", url])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "macos")]
async fn open_url(url: &str) -> Result<(), String> {
    std::process::Command::new("open")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "linux")]
async fn execute_binary(path: &str) -> Result<(), String> {
    std::process::Command::new(path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "windows")]
async fn execute_binary(path: &str) -> Result<(), String> {
    std::process::Command::new("cmd")
        .args(&["/C", path])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "macos")]
async fn execute_binary(path: &str) -> Result<(), String> {
    std::process::Command::new(path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_creation() {
        let launcher = Launcher {
            id: "test_001".to_string(),
            name: "Test App".to_string(),
            launch_type: LaunchType::App,
            target: "/usr/bin/test".to_string(),
            icon: "icon.svg".to_string(),
        };
        assert_eq!(launcher.name, "Test App");
    }
}
