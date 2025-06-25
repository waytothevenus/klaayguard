mod osquery;
use osquery::install;
use serde_json::Value;
use std::{
    collections::HashMap,
    path::PathBuf,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
    AppHandle,
    Runtime,
    Manager as _,
    Window,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct InstallationStatus {
    installed: bool,
    timestamp: u64,
    version: Option<String>,
    platform: String,
    error_count: u32,
    last_error: Option<String>,
}

impl Default for InstallationStatus {
    fn default() -> Self {
        Self {
            installed: false,
            timestamp: 0,
            version: None,
            platform: std::env::consts::OS.to_string(),
            error_count: 0,
            last_error: None,
        }
    }
}

fn get_installation_status_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not determine home directory".to_string())?;
    
    let klaayguard_dir = home_dir.join(".klaayguard");
    Ok(klaayguard_dir.join("installation_status.json"))
}

fn load_installation_status() -> Result<InstallationStatus, String> {
    let status_path = get_installation_status_path()?;
    
    if !status_path.exists() {
        return Ok(InstallationStatus::default());
    }
    
    let content = fs::read_to_string(&status_path)
        .map_err(|e| format!("Failed to read installation status: {}", e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse installation status: {}", e))
}

fn save_installation_status(status: &InstallationStatus) -> Result<(), String> {
    let status_path = get_installation_status_path()?;
    let klaayguard_dir = status_path.parent()
        .ok_or_else(|| "Invalid status path".to_string())?;
    
    // Create directory if it doesn't exist
    fs::create_dir_all(klaayguard_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    let content = serde_json::to_string_pretty(status)
        .map_err(|e| format!("Failed to serialize installation status: {}", e))?;
    
    fs::write(&status_path, content)
        .map_err(|e| format!("Failed to write installation status: {}", e))
}

// will return a different id every call if you don't have a hardware id until
// a build with https://github.com/osquery/osquery/pull/8616 is released
#[tauri::command]
async fn get_device_uuid() -> Result<String, String> {
    let tables = vec!("system_info".to_string());
    let query_result = execute_query(tables).await?;
    
    // Navigate the nested structure:
    // 1. Get "system_info" array
    // 2. Get first item in array
    // 3. Get "uuid" from that item
    let uuid = query_result
        .get("system_info")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|obj| obj.get("uuid"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Couldn't find device uuid".to_string())?;
    
    Ok(uuid.to_string())
}

#[tauri::command]
async fn execute_query(table_names: Vec<String>) -> Result<HashMap<String, Value>, String> {
    use serde_json::Value;
    use std::collections::HashMap;
    use std::process::{Command, Stdio};

    #[cfg(windows)]
    use std::os::windows::process::CommandExt;

    let mut all_results = HashMap::new();

    for table_name in table_names {
        // Configure the command
        let mut cmd = Command::new("osqueryi");
        
        cmd.args(&[
            "--json",
            &format!("SELECT * FROM {}", table_name),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

        // Windows-specific: Hide console window
        #[cfg(windows)]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        // Execute the command
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to run osquery: {}", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "Osquery failed for table {}: {}",
                table_name,
                error_msg.trim()
            ));
        }

        // Handle output
        let stdout_str = String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 output for table {}: {}", table_name, e))?;

        let parsed_result: Value = serde_json::from_str(&stdout_str)
            .map_err(|e| format!(
                "Failed to parse JSON for table {} (content: '{}'): {}", 
                table_name, 
                stdout_str.trim(), 
                e
            ))?;

        all_results.insert(table_name, parsed_result);
    }

    Ok(all_results)
}

#[tauri::command]
async fn check_osquery() -> Result<bool, String> {
    Ok(install::is_osquery_installed())
}

#[tauri::command]
async fn install_osquery() -> Result<(), String> {
    install::install_osquery().map_err(|e| e.to_string())
}

fn install_osquery_with_progress<R: Runtime>(_app: &AppHandle<R>) -> Result<(), String> {
    // For now, just call the regular install function
    // Progress events can be added later when we figure out the correct Tauri 2.0 API
    match install::install_osquery() {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn auto_install_osquery<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let result = install_osquery_with_progress(&app);
    if result.is_ok() {
        mark_first_launch_complete().await?;
    }
    result
}

#[tauri::command]
async fn is_first_launch() -> Result<bool, String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not determine home directory".to_string())?;
    
    let klaayguard_dir = home_dir.join(".klaayguard");
    let first_launch_file = klaayguard_dir.join("first_launch_complete");
    
    // Check if the first launch file exists
    Ok(!first_launch_file.exists())
}

#[tauri::command]
async fn mark_first_launch_complete() -> Result<(), String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not determine home directory".to_string())?;
    
    let klaayguard_dir = home_dir.join(".klaayguard");
    let first_launch_file = klaayguard_dir.join("first_launch_complete");
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&klaayguard_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    
    // Create the first launch file
    fs::write(&first_launch_file, "osquery_installed")
        .map_err(|e| format!("Failed to create first launch file: {}", e))?;
    
    // Update installation status
    let mut status = load_installation_status()?;
    status.installed = true;
    status.timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get timestamp: {}", e))?
        .as_secs();
    status.version = Some("5.17.0".to_string()); // osquery version
    status.error_count = 0;
    status.last_error = None;
    
    save_installation_status(&status)?;
    
    Ok(())
}

#[tauri::command]
async fn get_installation_status() -> Result<InstallationStatus, String> {
    load_installation_status()
}

#[tauri::command]
async fn record_installation_error(error_message: String) -> Result<(), String> {
    let mut status = load_installation_status()?;
    status.error_count += 1;
    status.last_error = Some(error_message);
    status.timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get timestamp: {}", e))?
        .as_secs();
    
    save_installation_status(&status)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_ = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    window_.hide().unwrap();
                    api.prevent_close();
                }
            });
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i, &show_i, &hide_i])?;
            TrayIconBuilder::new()
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Enter { .. } => {
                        tray.set_tooltip(Some("Klaay Guard".to_string())).unwrap();
                    }
                    TrayIconEvent::Leave { .. } => {
                        tray.set_tooltip(Some("")).unwrap();
                    }
                    _ => {}
                })
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            execute_query,
            install_osquery,
            check_osquery,
            get_device_uuid,
            is_first_launch,
            mark_first_launch_complete,
            auto_install_osquery,
            get_installation_status,
            record_installation_error,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
