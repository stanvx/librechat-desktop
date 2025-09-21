#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};

use tauri_plugin_global_shortcut::GlobalShortcutExt;

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
            let handle = app.handle();
            register_global_shortcuts(&handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_greeting])
        .run(tauri::generate_context!())
        .expect("error while running LibreChat Desktop");
}

fn register_global_shortcuts(app: &tauri::AppHandle) {
    let manager = app.global_shortcut();
    if let Err(err) = manager.register("CmdOrCtrl+Shift+L") {
        eprintln!("Global shortcut registration pending: {err}");
    }
}
