mod osquery;
use osquery::{install, service::OsqueryService};
use serde_json::Value;
use std::collections::HashMap;
#[tauri::command]
async fn execute_query(table_names: Vec<String>) -> Result<HashMap<String, Value>, String> {
    let mut all_results = HashMap::new();

    for table_name in table_names {
        let query = format!("SELECT * from {};", table_name);
        let service = OsqueryService;
        match service.query(&query) {
            Ok(result) => {
                let converted_result: Value = serde_json::to_value(result).map_err(|e| {
                    format!("Failed to convert result for table {}: {}", table_name, e)
                })?;
                all_results.insert(table_name.to_string(), converted_result);
            }
            Err(e) => {
                return Err(format!("Failed to query table {}: {}", table_name, e));
            }
        }
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute_query, install_osquery, check_osquery])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
