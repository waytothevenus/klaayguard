mod osquery;
use osquery::install;
use serde_json::Value;
use std::{
    collections::HashMap,
    process::{Command, Stdio},
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[tauri::command]
async fn execute_query(table_names: Vec<String>) -> Result<HashMap<String, Value>, String> {
    use std::os::windows::process::CommandExt;
    use serde_json::Value;
    use std::collections::HashMap;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut all_results = HashMap::new();

    for table_name in table_names {
        // Execute osquery command
        let output = Command::new("osqueryi")
            .args(&[
                "--json",
                &format!("SELECT * FROM {}", table_name),
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
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
            check_osquery
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
