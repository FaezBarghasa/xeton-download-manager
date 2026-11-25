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

            // Browser native-host bridge on 127.0.0.1:18512
            let manager = app.state::<AppState>().manager.clone();
            tauri::async_runtime::spawn(async move {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};
                if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:18512").await {
                    while let Ok((mut stream, _)) = listener.accept().await {
                        let mgr = manager.clone();
                        tauri::async_runtime::spawn(async move {
                            let mut buf = vec![0u8; 8192];
                            if let Ok(n) = stream.read(&mut buf).await {
                                let req = String::from_utf8_lossy(&buf[..n]);
                                if let Some(body) = req.split("\r\n\r\n").nth(1) {
                                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body.trim_matches('\0')) {
                                        if let Some(url) = val.get("url").and_then(|v| v.as_str()) {
                                            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                                            let folder = format!("{}/Downloads", home);
                                            let name = val
                                                .get("file_name")
                                                .and_then(|v| v.as_str())
                                                .map(String::from)
                                                .unwrap_or_else(|| url.split('/').last().unwrap_or("download").to_string());
                                            let props = xeton_core::models::NewDownloadProps {
                                                folder,
                                                name,
                                                link: url.to_string(),
                                                protocol: xeton_core::models::DownloadProtocol::Http,
                                                preferred_connections: Some(8),
                                                speed_limit: 0,
                                                on_duplicate: xeton_core::models::DuplicateStrategy::AddNumbered,
                                            };
                                            if let Ok(id) = mgr.add_download(props).await {
                                                let _ = mgr.resume(id).await;
                                            }
                                        }
                                    }
                                }
                            }
                            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK").await;
                        });
                    }
                }
            });

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
