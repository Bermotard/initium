//! Enregistrement autostart système
use cfg_if::cfg_if;

pub struct AutostartManager;

impl AutostartManager {
    pub fn enable() -> Result<(), String> {
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                Self::enable_linux()
            } else if #[cfg(target_os = "windows")] {
                Self::enable_windows()
            } else if #[cfg(target_os = "macos")] {
                Self::enable_macos()
            } else {
                Err("Unsupported platform".to_string())
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn enable_linux() -> Result<(), String> {
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn enable_windows() -> Result<(), String> {
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn enable_macos() -> Result<(), String> {
        Ok(())
    }
}
