mod commands;

use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    eprintln!("[XETON] Starting xeton-desktop...");
    let state = tauri::async_runtime::block_on(AppState::new())
        .expect("failed to initialize AppState");
    eprintln!("[XETON] AppState initialized successfully");

    tauri::Builder::default()
        .setup(|app| {
            eprintln!("[XETON] Tauri setup hook reached");
            for (label, win) in app.webview_windows() {
                eprintln!("[XETON] Found window: {}", label);
                let _ = win.show();
            }
            Ok(())
        })
        .on_page_load(|_window, payload| {
            eprintln!("[XETON] Page load: {:?} url: {}", payload.event(), payload.url());
        })
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_downloads,
            add_download,
            pause_download,
            resume_download,
            delete_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running Xeton Download Manager");
}
