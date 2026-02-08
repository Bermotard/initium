//! Module de gestion des lanceurs
//! Responsable du lancement d'URLs et d'applications

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    #[test]
    fn test_launcher_web_type() {
        let launcher = Launcher {
            id: "google".to_string(),
            name: "Google".to_string(),
            launch_type: LaunchType::Web,
            target: "https://google.com".to_string(),
            icon: "google.png".to_string(),
        };
        assert_eq!(launcher.launch_type, LaunchType::Web);
    }

    #[test]
    fn test_launcher_app_type() {
        let launcher = Launcher {
            id: "vscode".to_string(),
            name: "VS Code".to_string(),
            launch_type: LaunchType::App,
            target: "/usr/bin/code".to_string(),
            icon: "vscode.png".to_string(),
        };
        assert_eq!(launcher.launch_type, LaunchType::App);
    }

    #[tokio::test]
    async fn test_execute_launcher_web() {
        let launcher = Launcher {
            id: "test".to_string(),
            name: "Test".to_string(),
            launch_type: LaunchType::Web,
            target: "https://example.com".to_string(),
            icon: "icon.png".to_string(),
        };
        let result = execute_launcher(&launcher).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_launcher_app() {
    // Sur macOS, utiliser 'true' au lieu de '/usr/bin/test'
    #[cfg(target_os = "macos")]
    let app_path = "/bin/true";
    
    #[cfg(not(target_os = "macos"))]
    let app_path = "/usr/bin/test";
    
    let launcher = Launcher {
        id: "test".to_string(),
        name: "Test".to_string(),
        launch_type: LaunchType::App,
        target: app_path.to_string(),
        icon: "icon.png".to_string(),
    };
    let result = execute_launcher(&launcher).await;
    assert!(result.is_ok());
}
}
