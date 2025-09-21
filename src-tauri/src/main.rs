#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};

use tauri::Manager;
use tauri_plugin_global_shortcut::ShortcutManagerExt;

#[tauri::command]
async fn get_greeting() -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();
    Ok(format!(
        "Welcome back! LibreChat Desktop is online (heartbeat: {now})."
    ))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            register_global_shortcuts(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_greeting])
        .run(tauri::generate_context!())
        .expect("error while running LibreChat Desktop");
}

fn register_global_shortcuts(app: &tauri::AppHandle) -> tauri::Result<()> {
    let manager = app.global_shortcut();
    manager.register("CmdOrCtrl+Shift+L", || {
        println!("Global shortcut triggered: CmdOrCtrl+Shift+L");
    })?;
    Ok(())
}
