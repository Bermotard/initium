use initium::config_manager::ConfigManager;
use initium::launcher::{Launcher, LaunchType, generate_unique_id};
use serde_json::json;

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

#[cfg(not(target_os = "macos"))]
fn main() {
    run();
}

#[cfg(target_os = "macos")]
fn main() {
    run();
}
