use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

pub mod components;
pub mod dialogs;
pub mod tauri_bindings;
pub mod theme;

use components::{DownloadTable, Footer, IconDelete, IconPause, IconPlus, IconResume, IconSettings, Sidebar, Titlebar};
use dialogs::{AddDownloadModal, SettingsModal};
use tauri_bindings::{invoke, invoke_no_args};
use theme::THEME_DARK;
use xeton_models::{AddDownloadRequest, CategoryType, DownloadItem, DownloadStatus};

#[component]
pub fn App() -> Element {
    let current_theme = use_signal(|| THEME_DARK);
    let selected_category = use_signal(|| CategoryType::All);
    let mut selected_ids = use_signal(|| HashSet::new());
    let search_query = use_signal(|| "".to_string());
    let mut is_add_modal_open = use_signal(|| false);
    let mut is_settings_modal_open = use_signal(|| false);

    let mut downloads = use_signal(|| vec![
        DownloadItem {
            id: "1".into(),
            name: "ubuntu-24.04-desktop-amd64.iso".into(),
            url: "https://releases.ubuntu.com/24.04/ubuntu-24.04-desktop-amd64.iso".into(),
            category: CategoryType::Compressed,
            status: DownloadStatus::Downloading,
            total_size: Some(6_111_000_000),
            downloaded_size: 2_450_000_000,
            speed: 12_500_000,
            eta_seconds: Some(292),
            connections: 16,
            date_added: 1732431000,
            date_finished: None,
            file_path: "/home/jrad/Downloads/ubuntu-24.04-desktop-amd64.iso".into(),
            error_message: None,
        },
        DownloadItem {
            id: "2".into(),
            name: "Rust-Book-2024.pdf".into(),
            url: "https://doc.rust-lang.org/book/book.pdf".into(),
            category: CategoryType::Documents,
            status: DownloadStatus::Completed,
            total_size: Some(15_400_000),
            downloaded_size: 15_400_000,
            speed: 0,
            eta_seconds: None,
            connections: 8,
            date_added: 1732420000,
            date_finished: Some(1732420015),
            file_path: "/home/jrad/Downloads/Rust-Book-2024.pdf".into(),
            error_message: None,
        },
    ]);

    // Initial fetch on mount
    use_future(move || async move {
        if let Ok(items) = invoke_no_args::<Vec<DownloadItem>>("get_downloads").await {
            if !items.is_empty() {
                *downloads.write() = items;
            }
        }
    });

    let download_counts = use_memo(move || {
        let mut map = HashMap::new();
        let list = downloads.read();
        map.insert(CategoryType::All, list.len());
        for item in list.iter() {
            *map.entry(item.category).or_insert(0) += 1;
        }
        map
    });

    let filtered_downloads = use_memo(move || {
        let cat = *selected_category.read();
        let query = search_query.read().to_lowercase();
        let list = downloads.read();
        list.iter().filter(|d| {
            let matches_cat = cat == CategoryType::All || d.category == cat;
            let matches_search = query.is_empty() || d.name.to_lowercase().contains(&query);
            matches_cat && matches_search
        }).cloned().collect()
    });

    let active_count = use_memo(move || {
        downloads.read().iter().filter(|d| d.status == DownloadStatus::Downloading).count()
    });

    let total_speed = use_memo(move || {
        downloads.read().iter().filter(|d| d.status == DownloadStatus::Downloading).map(|d| d.speed).sum::<u64>()
    });

    let theme = *current_theme.read();

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100vw;
                background-color: {theme.background};
                color: {theme.on_background};
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
                overflow: hidden;
            ",
            // Custom Titlebar with Top Menu and Search Box
            Titlebar {
                theme: theme,
                title: "Xeton Download Manager".to_string(),
                search_query: search_query,
            }
            // Action Toolbar (Buttons matching Xeton Download Manager)
            div {
                class: "toolbar",
                style: "
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 16px;
                    background-color: {theme.surface};
                    border-bottom: 1px solid {theme.border};
                ",
                button {
                    onclick: move |_| *is_add_modal_open.write() = true,
                    style: "
                        display: flex; align-items: center; gap: 6px;
                        padding: 6px 12px; border-radius: 4px;
                        background-color: {theme.primary}; border: none;
                        color: {theme.on_primary}; font-weight: 600; font-size: 13px;
                        cursor: pointer;
                    ",
                    IconPlus { size: 16 }
                    span { "New Download" }
                }
                button {
                    onclick: move |_| {
                        let ids: Vec<String> = downloads.peek().iter().map(|d| d.id.clone()).collect();
                        spawn(async move {
                            for id in ids {
                                #[derive(serde::Serialize)]
                                struct IdPayload { id: String }
                                let _ = invoke::<(), _>("resume_download", &IdPayload { id }).await;
                            }
                        });
                    },
                    style: "
                        display: flex; align-items: center; gap: 6px;
                        padding: 6px 12px; border-radius: 4px;
                        background-color: transparent; border: 1px solid {theme.border};
                        color: {theme.on_surface}; font-size: 13px; cursor: pointer;
                    ",
                    IconResume { size: 14 }
                    span { "Resume All" }
                }
                button {
                    onclick: move |_| {
                        let ids: Vec<String> = downloads.peek().iter().map(|d| d.id.clone()).collect();
                        spawn(async move {
                            for id in ids {
                                #[derive(serde::Serialize)]
                                struct IdPayload { id: String }
                                let _ = invoke::<(), _>("pause_download", &IdPayload { id }).await;
                            }
                        });
                    },
                    style: "
                        display: flex; align-items: center; gap: 6px;
                        padding: 6px 12px; border-radius: 4px;
                        background-color: transparent; border: 1px solid {theme.border};
                        color: {theme.on_surface}; font-size: 13px; cursor: pointer;
                    ",
                    IconPause { size: 14 }
                    span { "Pause All" }
                }
                button {
                    onclick: move |_| {
                        let selected = selected_ids.peek().clone();
                        spawn(async move {
                            for id in selected {
                                #[derive(serde::Serialize)]
                                struct IdPayload { id: String }
                                let _ = invoke::<(), _>("delete_download", &IdPayload { id }).await;
                            }
                        });
                        downloads.write().retain(|d| !selected_ids.peek().contains(&d.id));
                        selected_ids.write().clear();
                    },
                    style: "
                        display: flex; align-items: center; gap: 6px;
                        padding: 6px 12px; border-radius: 4px;
                        background-color: transparent; border: 1px solid {theme.border};
                        color: {theme.error}; font-size: 13px; cursor: pointer;
                    ",
                    IconDelete { size: 14 }
                    span { "Delete" }
                }
                div { style: "flex: 1;" }
                button {
                    onclick: move |_| *is_settings_modal_open.write() = true,
                    style: "
                        display: flex; align-items: center; justify-content: center;
                        width: 32px; height: 32px; border-radius: 4px;
                        background-color: transparent; border: 1px solid {theme.border};
                        color: {theme.on_surface}; cursor: pointer;
                    ",
                    IconSettings { size: 16 }
                }
            }
            // Main Content Area
            div {
                style: "flex: 1; display: flex; overflow: hidden;",
                Sidebar {
                    theme: theme,
                    selected_category: selected_category,
                    download_counts: download_counts,
                }
                DownloadTable {
                    theme: theme,
                    downloads: filtered_downloads,
                    selected_ids: selected_ids,
                }
            }
            // Real-Time Footer Bar
            Footer {
                theme: theme,
                active_count: *active_count.read(),
                total_speed: *total_speed.read(),
            }
            // Add Download Modal Dialog
            AddDownloadModal {
                theme: theme,
                is_open: is_add_modal_open,
                on_submit: move |req: AddDownloadRequest| {
                    let req_clone = req.clone();
                    spawn(async move {
                        #[derive(serde::Serialize)]
                        struct AddPayload {
                            request: AddDownloadRequest,
                        }
                        if let Ok(new_item) = invoke::<DownloadItem, _>("add_download", &AddPayload { request: req_clone.clone() }).await {
                            downloads.write().push(new_item);
                        } else {
                            // Fallback if running outside Tauri
                            let next_id = format!("{}", downloads.peek().len() + 1);
                            downloads.write().push(DownloadItem {
                                id: next_id,
                                name: req_clone.file_name.unwrap_or_else(|| "downloaded_file".into()),
                                url: req_clone.url,
                                category: req_clone.category.unwrap_or(CategoryType::Other),
                                status: DownloadStatus::Downloading,
                                total_size: None,
                                downloaded_size: 0,
                                speed: 0,
                                eta_seconds: None,
                                connections: req_clone.connections.unwrap_or(8),
                                date_added: chrono::Utc::now().timestamp(),
                                date_finished: None,
                                file_path: "".into(),
                                error_message: None,
                            });
                        }
                    });
                }
            }
            // Settings Modal Dialog
            SettingsModal {
                theme: theme,
                is_open: is_settings_modal_open,
                current_theme: current_theme,
            }
        }
    }
}
