use initium::config_manager::ConfigManager;
use initium::launcher::{Launcher, LaunchType, generate_unique_id};
use serde_json::json;
use std::path::Path;

#[tauri::command]
fn set_background(background: String) -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    manager.config_mut().background = Some(background);
    manager.save()
}

#[tauri::command]
fn get_background() -> Result<Option<String>, String> {
    let manager = ConfigManager::load_or_default()?;
    Ok(manager.config().background.clone())
}

#[tauri::command]
fn set_language(language: String) -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    manager.set_language(language)
}

#[tauri::command]
fn get_language() -> Result<String, String> {
    let manager = ConfigManager::load_or_default()?;
    Ok(manager.get_language())
}

#[tauri::command]
fn get_settings() -> Result<serde_json::Value, String> {
    let manager = ConfigManager::load_or_default()?;
    Ok(json!({
        "language": manager.get_language(),
        "background": manager.config().background.clone(),
        "theme": manager.config().theme,
        "config_dir": ConfigManager::get_config_dir_path().to_string_lossy().to_string(),
        "icons_dir": ConfigManager::get_icons_dir_path().to_string_lossy().to_string(),
        "settings_dir": ConfigManager::get_settings_dir_path().to_string_lossy().to_string(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[tauri::command]
fn reset_settings() -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    manager.reset_settings()
}

#[tauri::command]
fn read_file_as_text(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file_as_base64(path: String) -> Result<String, String> {
    use std::io::Read;
    let mut file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("png");
    let mime = match ext { "svg" => "image/svg+xml", "jpg" | "jpeg" => "image/jpeg", "ico" => "image/x-icon", _ => "image/png" };
    Ok(format!("data:{};base64,{}", mime, base64_encode(&buffer)))
}

fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        let _ = write!(result, "{}{}{}{}", CHARS[b0>>2] as char, CHARS[((b0&3)<<4)|(b1>>4)] as char, if chunk.len()>1 {CHARS[((b1&15)<<2)|(b2>>6)] as char} else {'='}, if chunk.len()>2 {CHARS[b2&63] as char} else {'='});
    }
    result
}

#[tauri::command]
fn open_directory(path: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open").arg(&path).spawn().map_err(|e| e.to_string())?;
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer").arg(&path).spawn().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_all_settings(language: String, background: Option<String>) -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    manager.save_all_settings(language, background)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_launchers,
            add_launcher_cmd,
            remove_launcher_cmd,
            execute_launcher_cmd,
            export_config,
            import_config,
            set_background,
            get_background,
            set_language,
            get_language,
            get_settings,
            reset_settings,
            save_all_settings,
            open_directory,
            read_file_as_base64,
            write_file,
            read_file_as_text,
            // Drag & Drop commands
            is_executable,
            is_valid_url,
            extract_domain,
            is_desktop_file,
            is_url_file,
            parse_desktop_file,
            parse_url_file,
            extract_name_from_path,
            launcher_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Get all launchers from config
#[tauri::command]
fn get_launchers() -> Result<Vec<serde_json::Value>, String> {
    let manager = ConfigManager::load_or_default()?;
    let launchers = manager.config().launchers.clone();
    
    Ok(launchers.iter().map(|l| {
        serde_json::json!({
            "id": l.id,
            "name": l.name,
            "launch_type": if l.launch_type == LaunchType::Web { "web" } else { "app" },
            "target": l.target,
            "icon": l.icon,
        })
    }).collect())
}

/// Add a new launcher
#[tauri::command]
fn add_launcher_cmd(
    name: String,
    launch_type: String,
    target: String,
    icon: Option<String>,
) -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    
    let ltype = if launch_type == "web" {
        LaunchType::Web
    } else {
        LaunchType::App
    };
    
    // Générer l'ID automatiquement
    let existing_ids: Vec<String> = manager.config()
        .launchers
        .iter()
        .map(|l| l.id.clone())
        .collect();
    let id = generate_unique_id(&name, &existing_ids);
    let mut launcher = Launcher::new(id, name, ltype, target);
    launcher.icon = icon;
    
    manager.add_launcher(launcher)?;
    manager.save()
}

/// Remove a launcher
#[tauri::command]
fn remove_launcher_cmd(id: String) -> Result<(), String> {
    let mut manager = ConfigManager::load_or_default()?;
    manager.remove_launcher(&id)?;
    Ok(())
}

/// Execute a launcher
#[tauri::command]
async fn execute_launcher_cmd(id: String) -> Result<String, String> {
    let manager = ConfigManager::load_or_default()?;
    
    let launcher = manager.config().launchers.iter()
        .find(|l| l.id == id)
        .ok_or("Launcher not found")?
        .clone();
    
    launcher.execute().await?;
    Ok(format!("Launcher '{}' executed", launcher.name))
}

// ==================== Drag & Drop Commands ====================

/// Check if a file is executable
#[tauri::command]
fn is_executable(path: String) -> Result<bool, String> {
    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        Ok(metadata.permissions().mode() & 0o111 != 0)
    }
    
    #[cfg(windows)]
    {
        // On Windows, check if it's a .exe, .bat, .cmd, .lnk, or .msi file
        let path_lower = path.to_lowercase();
        let is_exe = path_lower.ends_with(".exe") || 
                     path_lower.ends_with(".bat") || 
                     path_lower.ends_with(".cmd") || 
                     path_lower.ends_with(".lnk") ||
                     path_lower.ends_with(".msi");
        Ok(is_exe)
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, check if it's an .app bundle or has execute permissions
        if path.ends_with(".app") {
            return Ok(true);
        }
        use std::os::unix::fs::PermissionsExt;
        Ok(metadata.permissions().mode() & 0o111 != 0)
    }
}

/// Validate if a string is a valid URL
#[tauri::command]
fn is_valid_url(url: String) -> bool {
    url.starts_with("http://") || 
    url.starts_with("https://") || 
    url.starts_with("ftp://") ||
    url.starts_with("file://")
}

/// Extract domain from URL
#[tauri::command]
fn extract_domain(url: String) -> String {
    // Remove protocol
    let without_protocol = url.split("://").last().unwrap_or(&url);
    // Get first part before /, ?, or #
    without_protocol.split('/').next()
        .and_then(|s| s.split('?').next())
        .and_then(|s| s.split('#').next())
        .unwrap_or("unknown")
        .to_string()
}

/// Check if a file is a .desktop file (Linux)
#[tauri::command]
fn is_desktop_file(path: String) -> bool {
    path.ends_with(".desktop")
}

/// Check if a file is a .url file (Windows)
#[tauri::command]
fn is_url_file(path: String) -> bool {
    path.to_lowercase().ends_with(".url")
}

/// Parse a .desktop file and extract Name, Exec, and Icon
#[tauri::command]
fn parse_desktop_file(path: String) -> Result<serde_json::Value, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    
    let mut name = String::new();
    let mut exec = String::new();
    let mut icon = String::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("Name=") {
            name = line[5..].trim().to_string();
        } else if line.starts_with("Exec=") {
            // Extract the command and remove %U, %F, etc. placeholders
            let cmd = line[5..].trim();
            exec = cmd.split_whitespace().next().unwrap_or("").to_string();
            // Remove %U, %F, %u, %f, etc.
            exec = exec.replace("%U", "").replace("%F", "").replace("%u", "").replace("%f", "");
        } else if line.starts_with("Icon=") {
            icon = line[5..].trim().to_string();
        }
    }
    
    Ok(json!({
        "name": name,
        "exec": exec,
        "icon": icon
    }))
}

/// Parse a .url file and extract the URL
#[tauri::command]
fn parse_url_file(path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    
    // Windows .url files have format: [InternetShortcut]
    // URL=https://example.com
    for line in content.lines() {
        let line = line.trim();
        if line.to_uppercase().starts_with("URL=") {
            return Ok(line[4..].trim().to_string());
        }
    }
    
    Err("No URL found in .url file".to_string())
}

/// Extract name from file path
#[tauri::command]
fn extract_name_from_path(path: String) -> String {
    let path_obj = Path::new(&path);
    let file_name = path_obj.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    // Remove extension
    let stem = Path::new(file_name).file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    
    stem.to_string()
}

/// Check if a launcher with the same target already exists
#[tauri::command]
fn launcher_exists(target: String) -> Result<bool, String> {
    let manager = ConfigManager::load_or_default()?;
    let exists = manager.config().launchers.iter()
        .any(|l| l.target == target);
    Ok(exists)
}

#[tauri::command]
fn export_config() -> Result<String, String> {
    let manager = ConfigManager::load_or_default()?;
    manager.export_to_json()
}

#[tauri::command]
fn import_config(json: String) -> Result<(), String> {
    let manager = ConfigManager::import_from_json(&json)?;
    manager.save()
}

// ==================== Drag & Drop Tests ====================

#[cfg(test)]
mod drag_drop_tests {
    use super::*;
    
    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://github.com".to_string()));
        assert!(is_valid_url("http://example.com".to_string()));
        assert!(is_valid_url("ftp://ftp.example.com".to_string()));
        assert!(!is_valid_url("/path/to/file".to_string()));
        assert!(!is_valid_url("not-a-url".to_string()));
    }
    
    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://github.com/user/repo".to_string()), "github.com");
        assert_eq!(extract_domain("http://localhost:3000".to_string()), "localhost:3000");
        assert_eq!(extract_domain("https://example.com/path?query=1".to_string()), "example.com");
        assert_eq!(extract_domain("ftp://ftp.example.com/file".to_string()), "ftp.example.com");
    }
    
    #[test]
    fn test_is_desktop_file() {
        assert!(is_desktop_file("/path/to/app.desktop".to_string()));
        assert!(!is_desktop_file("/path/to/app.exe".to_string()));
    }
    
    #[test]
    fn test_is_url_file() {
        assert!(is_url_file("/path/to/link.url".to_string()));
        assert!(is_url_file("/path/to/LINK.URL".to_string()));
        assert!(!is_url_file("/path/to/link.txt".to_string()));
    }
    
    #[test]
    fn test_extract_name_from_path() {
        assert_eq!(extract_name_from_path("/usr/bin/firefox".to_string()), "firefox");
        assert_eq!(extract_name_from_path("/path/to/MyApp.exe".to_string()), "MyApp");
        assert_eq!(extract_name_from_path("firefox.desktop".to_string()), "firefox");
        assert_eq!(extract_name_from_path("no_extension".to_string()), "no_extension");
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    run();
}

#[cfg(target_os = "macos")]
fn main() {
    run();
}
