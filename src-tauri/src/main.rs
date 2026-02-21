use initium::config_manager::ConfigManager;
use initium::launcher::{Launcher, LaunchType};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_launchers,
            add_launcher_cmd,
            remove_launcher_cmd,
            execute_launcher_cmd,
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
    id: String,
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
    
    let mut launcher = Launcher::new(id, name, ltype, target);
    launcher.icon = icon;
    manager.add_launcher(launcher)?;
    
    Ok(())
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

#[cfg(not(target_os = "macos"))]
fn main() {
    run();
}

#[cfg(target_os = "macos")]
fn main() {
    run();
}