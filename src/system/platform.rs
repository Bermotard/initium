//! Abstractions platform-spécifiques
pub trait PlatformManager {
    fn open_url(&self, url: &str) -> Result<(), String>;
    fn execute_binary(&self, path: &str) -> Result<(), String>;
    fn enable_autostart(&self) -> Result<(), String>;
}
