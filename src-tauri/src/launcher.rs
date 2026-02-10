//! Launcher Module
//!
//! Handles launching applications and opening URLs.
//! Features:
//! - Support for web URLs and applications
//! - Cross-platform execution (Linux, Windows, macOS)
//! - Command-line arguments support
//! - Environment variables support
//! - Global timeout (default 30 seconds)
//! - Comprehensive error handling and logging
//! - Uses shell execution for compatibility with Tauri

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Type of launcher target
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LaunchType {
    /// Web URL launcher
    Web,
    /// Application launcher
    App,
}

/// Launch options with arguments, timeout, and environment variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOptions {
    /// Command-line arguments
    #[serde(default)]
    pub args: Vec<String>,
    /// Execution timeout in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    /// Optional environment variables
    #[serde(default)]
    pub env_vars: Option<Vec<(String, String)>>,
}

fn default_timeout() -> u64 {
    30
}

impl Default for LaunchOptions {
    fn default() -> Self {
        LaunchOptions {
            args: Vec::new(),
            timeout_secs: 30,
            env_vars: None,
        }
    }
}

/// Launcher configuration item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Type of launcher
    #[serde(rename = "type")]
    pub launch_type: LaunchType,
    /// Target URL or application path
    pub target: String,
    /// Optional icon path
    #[serde(default)]
    pub icon: Option<String>,
    /// Launch options (arguments, timeout, env vars)
    #[serde(default)]
    pub options: Option<LaunchOptions>,
}

impl Launcher {
    /// Create a new launcher
    pub fn new(id: String, name: String, launch_type: LaunchType, target: String) -> Self {
        Launcher {
            id,
            name,
            launch_type,
            target,
            icon: None,
            options: None,
        }
    }

    /// Create a new launcher with options
    pub fn with_options(
        id: String,
        name: String,
        launch_type: LaunchType,
        target: String,
        options: LaunchOptions,
    ) -> Self {
        Launcher {
            id,
            name,
            launch_type,
            target,
            icon: None,
            options: Some(options),
        }
    }

    /// Execute the launcher
    pub async fn execute(&self) -> Result<(), String> {
        let options = self.options.clone().unwrap_or_default();
        match self.launch_type {
            LaunchType::Web => execute_url(&self.target, &options).await,
            LaunchType::App => execute_app(&self.target, &options).await,
        }
    }
}

/// Open a URL with platform-specific implementation (Linux)
#[cfg(target_os = "linux")]
pub async fn execute_url(url: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Opening URL (Linux): {}", url);

    // Build command line: xdg-open URL [args...]
    let mut cmd_line = format!("xdg-open '{}'", url);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("'{}'", arg));
    }

    log::info!("Executing: sh -c '{}'", cmd_line);

    let mut cmd = tokio::process::Command::new("sh");
    cmd.arg("-c").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("URL opened successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Open a URL with platform-specific implementation (Windows)
#[cfg(target_os = "windows")]
pub async fn execute_url(url: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Opening URL (Windows): {}", url);

    // Build command line: start URL [args...]
    let mut cmd_line = format!("start \"\" \"{}\"", url);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("\"{}\"", arg));
    }

    log::info!("Executing: cmd /C {}", cmd_line);

    let mut cmd = tokio::process::Command::new("cmd");
    cmd.arg("/C").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("URL opened successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Open a URL with platform-specific implementation (macOS)
#[cfg(target_os = "macos")]
pub async fn execute_url(url: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Opening URL (macOS): {}", url);

    // Build command line: open URL [args...]
    let mut cmd_line = format!("open '{}'", url);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("'{}'", arg));
    }

    log::info!("Executing: sh -c '{}'", cmd_line);

    let mut cmd = tokio::process::Command::new("sh");
    cmd.arg("-c").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("URL opened successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Execute an application with platform-specific implementation (Linux)
#[cfg(target_os = "linux")]
pub async fn execute_app(path: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Executing app (Linux): {}", path);

    // Build command line: app_path [args...]
    let mut cmd_line = format!("'{}'", path);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("'{}'", arg));
    }

    log::info!("Executing: sh -c '{}'", cmd_line);

    let mut cmd = tokio::process::Command::new("sh");
    cmd.arg("-c").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("Application executed successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Execute an application with platform-specific implementation (Windows)
#[cfg(target_os = "windows")]
pub async fn execute_app(path: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Executing app (Windows): {}", path);

    // Build command line: app_path [args...]
    let mut cmd_line = format!("\"{}\"", path);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("\"{}\"", arg));
    }

    log::info!("Executing: cmd /C {}", cmd_line);

    let mut cmd = tokio::process::Command::new("cmd");
    cmd.arg("/C").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("Application executed successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

/// Execute an application with platform-specific implementation (macOS)
#[cfg(target_os = "macos")]
pub async fn execute_app(path: &str, options: &LaunchOptions) -> Result<(), String> {
    log::info!("Executing app (macOS): {}", path);

    // Build command line: app_path [args...]
    let mut cmd_line = format!("'{}'", path);
    for arg in &options.args {
        cmd_line.push(' ');
        cmd_line.push_str(&format!("'{}'", arg));
    }

    log::info!("Executing: sh -c '{}'", cmd_line);

    let mut cmd = tokio::process::Command::new("sh");
    cmd.arg("-c").arg(&cmd_line);

    if let Some(env_vars) = &options.env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let timeout = Duration::from_secs(options.timeout_secs);

    match tokio::time::timeout(timeout, cmd.spawn().map_err(|e| e.to_string())?.wait()).await {
        Ok(Ok(status)) => {
            if status.success() {
                log::info!("Application executed successfully");
                Ok(())
            } else {
                let msg = format!("Process exited with status: {}", status);
                log::error!("{}", msg);
                Err(msg)
            }
        }
        Ok(Err(e)) => {
            let msg = format!("Execution error: {}", e);
            log::error!("{}", msg);
            Err(msg)
        }
        Err(_) => {
            let msg = format!("Timeout exceeded ({} seconds)", options.timeout_secs);
            log::error!("{}", msg);
            Err(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_creation() {
        let launcher = Launcher::new(
            "test".to_string(),
            "Test".to_string(),
            LaunchType::Web,
            "https://example.com".to_string(),
        );
        assert_eq!(launcher.id, "test");
        assert_eq!(launcher.launch_type, LaunchType::Web);
    }

    #[test]
    fn test_launcher_with_icon() {
        let mut launcher = Launcher::new(
            "test".to_string(),
            "Test".to_string(),
            LaunchType::App,
            "sh".to_string(),
        );
        launcher.icon = Some("icon.png".to_string());
        assert!(launcher.icon.is_some());
    }

    #[test]
    fn test_launch_options_default() {
        let options = LaunchOptions::default();
        assert_eq!(options.timeout_secs, 30);
        assert!(options.args.is_empty());
        assert!(options.env_vars.is_none());
    }

    #[test]
    fn test_launcher_with_options() {
        let options = LaunchOptions {
            args: vec!["arg1".to_string(), "arg2".to_string()],
            timeout_secs: 60,
            env_vars: Some(vec![("KEY".to_string(), "value".to_string())]),
        };
        let launcher = Launcher::with_options(
            "test".to_string(),
            "Test".to_string(),
            LaunchType::App,
            "sh".to_string(),
            options,
        );
        assert_eq!(launcher.options.unwrap().timeout_secs, 60);
    }

    #[tokio::test]
    async fn test_execute_app_with_args() {
        let options = LaunchOptions {
            args: vec!["-c".to_string(), "echo test".to_string()],
            timeout_secs: 5,
            env_vars: None,
        };
        let result = execute_app("sh", &options).await;
        assert!(result.is_ok(), "Failed: {:?}", result);
    }

    #[tokio::test]
    async fn test_launcher_execute_web() {
        let launcher = Launcher::new(
            "test".to_string(),
            "Test".to_string(),
            LaunchType::Web,
            "https://example.com".to_string(),
        );
        let result = launcher.execute().await;
        let _ = result;
    }

    #[tokio::test]
    async fn test_launcher_execute_app() {
        let launcher = Launcher::new(
            "test".to_string(),
            "Test".to_string(),
            LaunchType::App,
            "sh".to_string(),
        );
        let result = launcher.execute().await;
        let _ = result;
    }

    #[tokio::test]
    async fn test_timeout_exceeded() {
        let options = LaunchOptions {
            args: vec!["-c".to_string(), "sleep 10".to_string()],
            timeout_secs: 1,
            env_vars: None,
        };
        let result = execute_app("sh", &options).await;
        assert!(result.is_err(), "Should timeout");
    }

    #[tokio::test]
    async fn test_launcher_with_env_vars() {
        let options = LaunchOptions {
            args: vec!["-c".to_string(), "echo $TEST_VAR".to_string()],
            timeout_secs: 5,
            env_vars: Some(vec![("TEST_VAR".to_string(), "hello".to_string())]),
        };
        let result = execute_app("sh", &options).await;
        assert!(result.is_ok(), "Failed: {:?}", result);
    }
}