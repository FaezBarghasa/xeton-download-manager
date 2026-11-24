mod commands;

use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    eprintln!("[XETON] Starting xeton-desktop...");
    let state = tauri::async_runtime::block_on(AppState::new())
        .expect("failed to initialize AppState");
    eprintln!("[XETON] AppState initialized successfully");

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
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
        });

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }));
    }

    builder
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .manage(QueueState::new())
        .manage(SettingsState::new())
        .invoke_handler(tauri::generate_handler![
            get_downloads,
            add_download,
            pause_download,
            resume_download,
            delete_download,
            get_queues,
            create_queue,
            delete_queue,
            get_app_settings,
            save_app_settings,
            calculate_checksum,
            add_batch_downloads,
            get_download_parts
        ])
        .run(tauri::generate_context!())
        .expect("error while running Xeton Download Manager");
}
