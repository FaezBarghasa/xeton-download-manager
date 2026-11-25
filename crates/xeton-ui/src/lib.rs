use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

use xeton_models::{
    AddDownloadRequest, CategoryType, CustomCategory, DownloadItem, DownloadStatus,
};

mod components;
mod dialogs;
mod tauri_bindings;
mod theme;

use components::footer::Footer;
use components::icons::{IconDelete, IconPause, IconPlus, IconResume, IconSettings, IconZap};
use components::mobile_bottom_bar::MobileBottomBar;
use components::mobile_card::MobileCard;
use components::mobile_header::MobileHeader;
use components::sidebar::Sidebar;
use components::table::DownloadTable;
use components::titlebar::Titlebar;

use dialogs::{
    AboutModal, AddDownloadModal, BatchDownloadModal, CategoryModal, ChecksumModal, ConfirmKind,
    ConfirmModal, DownloadDetailsModal, DownloadErrorModal, EditDownloadModal, MobileAddModal,
    MobileAddSheet, MobileBrowserPage, MobileMenuSheet, MobileSettingsPage, MobileSetupWizard,
    PerHostModal, QRCodeModal, QueueManagerModal, SettingsModal,
};
use tauri_bindings::{invoke, invoke_no_args};
use theme::{THEME_DARK, ThemePalette};

#[component]
pub fn App() -> Element {
    let mut current_theme = use_signal(|| THEME_DARK);
    let mut selected_category = use_signal(|| CategoryType::All);
    let mut search_query = use_signal(|| String::new());
    let mut downloads = use_signal(|| {
        vec![
            DownloadItem {
                id: "1".into(),
                name: "ubuntu-24.04-desktop-amd64.iso".into(),
                url: "https://releases.ubuntu.com/24.04/ubuntu-24.04-desktop-amd64.iso".into(),
                category: CategoryType::Compressed,
                status: DownloadStatus::Downloading,
                total_size: Some(6_111_000_000),
                downloaded_size: 2_450_000_000,
                speed: 14_500_000,
                eta_seconds: Some(252),
                connections: 16,
                date_added: 1713900000,
                date_finished: None,
                file_path: "/home/user/Downloads/ubuntu-24.04-desktop-amd64.iso".into(),
                error_message: None,
            },
            DownloadItem {
                id: "2".into(),
                name: "Blender-4.1.0-linux-x64.tar.xz".into(),
                url:
                    "https://download.blender.org/release/Blender4.1/blender-4.1.0-linux-x64.tar.xz"
                        .into(),
                category: CategoryType::Programs,
                status: DownloadStatus::Paused,
                total_size: Some(340_000_000),
                downloaded_size: 170_000_000,
                speed: 0,
                eta_seconds: None,
                connections: 8,
                date_added: 1713800000,
                date_finished: None,
                file_path: "/home/user/Downloads/Blender-4.1.0-linux-x64.tar.xz".into(),
                error_message: None,
            },
            DownloadItem {
                id: "3".into(),
                name: "Rust-2024-Edition-Guide.pdf".into(),
                url: "https://doc.rust-lang.org/guide.pdf".into(),
                category: CategoryType::Documents,
                status: DownloadStatus::Completed,
                total_size: Some(15_400_000),
                downloaded_size: 15_400_000,
                speed: 0,
                eta_seconds: Some(0),
                connections: 4,
                date_added: 1713700000,
                date_finished: Some(1713700500),
                file_path: "/home/user/Downloads/Rust-2024-Edition-Guide.pdf".into(),
                error_message: None,
            },
        ]
    });
    let mut selected_ids = use_signal(|| HashSet::<String>::new());

    // Mobile / Desktop Mode Detection
    let is_mobile = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(width) = win.inner_width() {
                    if let Some(w) = width.as_f64() {
                        return w < 768.0;
                    }
                }
            }
        }
        false
    });

    // Mobile Wizard & Screen States
    let mut is_setup_wizard_open = use_signal(|| false);
    let mut is_mobile_add_sheet_open = use_signal(|| false);
    let mut is_mobile_add_modal_open = use_signal(|| false);
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut is_mobile_settings_open = use_signal(|| false);
    let mut is_mobile_browser_open = use_signal(|| false);
    let mut is_mobile_search_active = use_signal(|| false);
    let mut table_density = use_signal(|| "normal".to_string());

    // Desktop Modal Visibility States
    let mut is_add_modal_open = use_signal(|| false);
    let mut is_batch_modal_open = use_signal(|| false);
    let mut is_details_modal_open = use_signal(|| false);
    let mut is_queue_modal_open = use_signal(|| false);
    let mut is_category_modal_open = use_signal(|| false);
    let mut is_checksum_modal_open = use_signal(|| false);
    let mut is_edit_modal_open = use_signal(|| false);
    let mut is_per_host_modal_open = use_signal(|| false);
    let mut is_about_modal_open = use_signal(|| false);
    let mut is_confirm_delete_open = use_signal(|| false);
    let mut is_settings_modal_open = use_signal(|| false);
    let mut is_error_modal_open = use_signal(|| false);
    let mut is_qr_modal_open = use_signal(|| false);
    let mut error_dialog_title = use_signal(|| "Download Error".to_string());
    let mut error_dialog_message = use_signal(|| "Failed to connect to remote server".to_string());
    let mut qr_dialog_url =
        use_signal(|| "https://github.com/FaezBarghasa/xeton-download-manager".to_string());

    let mut targeted_item = use_signal(|| Option::<DownloadItem>::None);
    let parts_info = use_signal(|| Option::<xeton_models::DownloadPartsInfo>::None);
    let mut targeted_checksum_path = use_signal(|| Option::<String>::None);

    use_future(move || async move {
        loop {
            if let Ok(items) = invoke_no_args::<Vec<DownloadItem>>("get_downloads").await {
                if !items.is_empty() {
                    *downloads.write() = items;
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    if let Some(win) = web_sys::window() {
                        let _ = win
                            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1000);
                    }
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            break;
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

    let filtered_downloads = use_memo(move || -> Vec<DownloadItem> {
        let cat = *selected_category.read();
        let query = search_query.read().to_lowercase();
        let list = downloads.read();
        list.iter()
            .filter(|d| {
                let matches_cat = cat == CategoryType::All || d.category == cat;
                let matches_search = query.is_empty() || d.name.to_lowercase().contains(&query);
                matches_cat && matches_search
            })
            .cloned()
            .collect()
    });

    let active_count = use_memo(move || {
        downloads
            .read()
            .iter()
            .filter(|d| d.status == DownloadStatus::Downloading)
            .count()
    });

    let total_speed = use_memo(move || {
        downloads
            .read()
            .iter()
            .filter(|d| d.status == DownloadStatus::Downloading)
            .map(|d| d.speed)
            .sum::<u64>()
    });

    let theme = *current_theme.read();

    rsx! {
        style { "{include_str!(\"theme/xeton.css\")}" }
        div {
            style: "
                --bg: {theme.background};
                --on-bg: {theme.on_background};
                --surface: {theme.surface};
                --on-surface: {theme.on_surface};
                --on-surface-muted: {theme.on_surface_muted};
                --border: {theme.border};
                --primary: {theme.primary};
                --primary-hover: {theme.primary_variant};
                --on-primary: {theme.on_primary};
                --success: {theme.success};
                --warning: {theme.warning};
                --error: {theme.error};
                --info: {theme.info};
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100vw;
                background-color: {theme.background};
                color: {theme.on_background};
                font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
                overflow: hidden;
            ",
            // Conditional Render: Mobile Mode vs. Desktop Mode
            if *is_mobile.read() {
                // MOBILE MODE
                div {
                    style: "display: flex; flex-direction: column; height: 100vh; width: 100vw; position: relative;",
                    // Mobile Header
                    MobileHeader {
                        theme: theme,
                        title: "Xeton Download Manager".to_string(),
                        on_search_click: move |_| {
                            let curr = *is_mobile_search_active.read();
                            is_mobile_search_active.set(!curr);
                        }
                    }

                    // Optional Inline Search Bar in Mobile
                    if *is_mobile_search_active.read() {
                        div {
                            style: "padding: 8px 16px; background-color: {theme.surface}; border-bottom: 1px solid {theme.border};",
                            input {
                                r#type: "text",
                                placeholder: "Search downloads...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                                style: "
                                    width: 100%; height: 36px; border-radius: 8px; border: 1px solid {theme.border};
                                    background-color: {theme.background}; color: {theme.on_surface}; padding: 0 12px;
                                    box-sizing: border-box; outline: none; font-size: 14px;
                                "
                            }
                        }
                    }

                    // Mobile Downloads List / Empty State
                    div {
                        style: "flex: 1; overflow-y: auto; padding: 14px 16px 80px 16px; box-sizing: border-box;",
                        if filtered_downloads.read().is_empty() {
                            div {
                                style: "height: 60vh; display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                span {
                                    style: "font-size: 16px; font-weight: 500; color: {theme.on_surface_muted};",
                                    "List is empty!"
                                }
                            }
                        } else {
                            for item in filtered_downloads.read().iter() {
                                {
                                    let current_item = item.clone();
                                    let item_for_click = item.clone();
                                    let item_id = item.id.clone();
                                    let is_downloading = item.status == DownloadStatus::Downloading;
                                    rsx! {
                                        MobileCard {
                                            theme: theme,
                                            item: current_item,
                                            on_click: move |_| {
                                                targeted_item.set(Some(item_for_click.clone()));
                                                is_details_modal_open.set(true);
                                            },
                                            on_toggle_pause: move |_| {
                                                let id = item_id.clone();
                                                spawn(async move {
                                                    #[derive(serde::Serialize)]
                                                    struct IdPayload { id: String }
                                                    if is_downloading {
                                                        let _ = invoke::<(), _>("pause_download", &IdPayload { id }).await;
                                                    } else {
                                                        let _ = invoke::<(), _>("resume_download", &IdPayload { id }).await;
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Mobile Floating Bottom Navigation Pill
                    MobileBottomBar {
                        theme: theme,
                        selected_category: *selected_category.read(),
                        on_select_category: move |cat| selected_category.set(cat),
                        on_menu_click: move |_| is_mobile_menu_open.set(true),
                        on_search_click: move |_| {
                            let curr = *is_mobile_search_active.read();
                            is_mobile_search_active.set(!curr);
                        },
                        on_add_click: move |_| is_mobile_add_sheet_open.set(true),
                    }
                }
            } else {
                // DESKTOP MODE
                Titlebar {
                        theme: theme,
                        title: "Xeton Download Manager".to_string(),
                        search_query: search_query,
                        on_menu_action: move |action: String| {
                            match action.as_str() {
                                "new_download" => *is_add_modal_open.write() = true,
                                "batch_download" => is_batch_modal_open.set(true),
                                "queue_manager" => is_queue_modal_open.set(true),
                                "categories" => is_category_modal_open.set(true),
                                "checksum" => {
                                    targeted_checksum_path.set(None);
                                    is_checksum_modal_open.set(true);
                                },
                                "per_host" => is_per_host_modal_open.set(true),
                                "preferences" => *is_settings_modal_open.write() = true,
                                "about" => is_about_modal_open.set(true),
                                "delete" => {
                                    if !selected_ids.peek().is_empty() {
                                        is_confirm_delete_open.set(true);
                                    }
                                },
                                "select_all" => {
                                    let all_ids: HashSet<String> = downloads.peek().iter().map(|d| d.id.clone()).collect();
                                    *selected_ids.write() = all_ids;
                                },
                                "exit" => {
                                    #[cfg(target_arch = "wasm32")]
                                    crate::tauri_bindings::close_window();
                                },
                                _ => {}
                            }
                        }
                    }
                    div {
                        class: "toolbar",
                        style: "
                            display: flex; align-items: center; gap: 8px; padding: 8px 16px;
                            background-color: {theme.surface}; border-bottom: 1px solid {theme.border};
                        ",
                        button {
                            class: "xeton-btn xeton-btn-primary",
                            onclick: move |_| *is_add_modal_open.write() = true,
                            IconPlus { size: 15 }
                            span { "New Download" }
                        }
                        button {
                            class: "xeton-btn",
                            onclick: move |_| is_batch_modal_open.set(true),
                            IconZap { size: 14, color: Some(theme.primary.to_string()) }
                            span { "Batch Add" }
                        }
                        button {
                            class: "xeton-btn",
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
                            IconResume { size: 13 }
                            span { "Resume All" }
                        }
                        button {
                            class: "xeton-btn",
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
                            IconPause { size: 13 }
                            span { "Pause All" }
                        }
                        button {
                            class: "xeton-btn xeton-btn-danger",
                            disabled: selected_ids.read().is_empty(),
                            onclick: move |_| {
                                if !selected_ids.peek().is_empty() {
                                    is_confirm_delete_open.set(true);
                                }
                            },
                            IconDelete { size: 14 }
                            span { "Delete" }
                        }
                        div { style: "flex: 1;" }
                        button {
                            class: "xeton-icon-btn",
                            title: "Preferences & Settings",
                            onclick: move |_| *is_settings_modal_open.write() = true,
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
                            density: table_density.read().clone(),
                        }
                    }
                    // Real-Time Footer Bar
                    Footer {
                        theme: theme,
                        active_count: *active_count.read(),
                        total_speed: *total_speed.read(),
                        density: table_density.read().clone(),
                        on_cycle_density: move |_| {
                            let next = match table_density.peek().as_str() {
                                "compact" => "normal",
                                "normal" => "comfortable",
                                _ => "compact",
                            };
                            table_density.set(next.to_string());
                        },
                    }
            }

            // --- MOBILE SHEETS & DIALOGS ---
            if *is_setup_wizard_open.read() {
                MobileSetupWizard {
                    theme: theme,
                    on_finish: move |_| is_setup_wizard_open.set(false),
                }
            }

            if *is_mobile_add_sheet_open.read() {
                MobileAddSheet {
                    theme: theme,
                    on_close: move |_| is_mobile_add_sheet_open.set(false),
                    on_select_option: move |opt: String| {
                        is_mobile_add_sheet_open.set(false);
                        match opt.as_str() {
                            "clipboard" => is_mobile_add_modal_open.set(true),
                            "new" => is_mobile_add_modal_open.set(true),
                            "batch" => is_batch_modal_open.set(true),
                            _ => {}
                        }
                    }
                }
            }

            if *is_mobile_add_modal_open.read() {
                MobileAddModal {
                    theme: theme,
                    on_close: move |_| is_mobile_add_modal_open.set(false),
                    on_start: move |(url, name): (String, String)| {
                        is_mobile_add_modal_open.set(false);
                        spawn(async move {
                            let _ = invoke::<DownloadItem, _>("add_download", &AddDownloadRequest {
                                url,
                                file_name: if name.is_empty() { None } else { Some(name) },
                                output_dir: None,
                                connections: Some(8),
                                category: None,
                                start_now: true,
                            }).await;
                        });
                    }
                }
            }

            if *is_mobile_menu_open.read() {
                MobileMenuSheet {
                    theme: theme,
                    on_close: move |_| is_mobile_menu_open.set(false),
                    on_select_item: move |item: String| {
                        is_mobile_menu_open.set(false);
                        match item.as_str() {
                            "browser" => is_mobile_browser_open.set(true),
                            "settings" => is_mobile_settings_open.set(true),
                            "per_host" => is_per_host_modal_open.set(true),
                            "about" => is_about_modal_open.set(true),
                            "delete" => {
                                if !selected_ids.peek().is_empty() {
                                    is_confirm_delete_open.set(true);
                                }
                            },
                            "resume_all" => {
                                let ids: Vec<String> = downloads.peek().iter().map(|d| d.id.clone()).collect();
                                spawn(async move {
                                    for id in ids {
                                        #[derive(serde::Serialize)]
                                        struct IdPayload { id: String }
                                        let _ = invoke::<(), _>("resume_download", &IdPayload { id }).await;
                                    }
                                });
                            },
                            "stop_all" => {
                                let ids: Vec<String> = downloads.peek().iter().map(|d| d.id.clone()).collect();
                                spawn(async move {
                                    for id in ids {
                                        #[derive(serde::Serialize)]
                                        struct IdPayload { id: String }
                                        let _ = invoke::<(), _>("pause_download", &IdPayload { id }).await;
                                    }
                                });
                            },
                            _ => {}
                        }
                    }
                }
            }

            if *is_mobile_settings_open.read() {
                MobileSettingsPage {
                    theme: theme,
                    on_back: move |_| is_mobile_settings_open.set(false),
                }
            }

            if *is_mobile_browser_open.read() {
                MobileBrowserPage {
                    theme: theme,
                    on_back: move |_| is_mobile_browser_open.set(false),
                }
            }

            // --- SHARED MODALS (Desktop & Mobile) ---
            AddDownloadModal {
                theme: theme,
                is_open: is_add_modal_open,
                on_submit: move |req: AddDownloadRequest| {
                    spawn(async move {
                        if let Ok(item) = invoke::<DownloadItem, _>("add_download", &req).await {
                            downloads.write().push(item);
                        }
                    });
                }
            }

            BatchDownloadModal {
                theme: theme,
                is_open: is_batch_modal_open,
                on_submit: move |req: xeton_models::BatchDownloadRequest| {
                    spawn(async move {
                        let _ = invoke::<Vec<DownloadItem>, _>("add_batch_downloads", &req).await;
                    });
                }
            }

            DownloadDetailsModal {
                theme: theme,
                is_open: is_details_modal_open,
                item: targeted_item,
                parts_info: parts_info,
            }

            SettingsModal {
                theme: theme,
                is_open: is_settings_modal_open,
                current_theme: current_theme,
            }

            if is_queue_modal_open() {
                QueueManagerModal {
                    on_close: move |_| is_queue_modal_open.set(false),
                }
            }

            if is_category_modal_open() {
                CategoryModal {
                    category: None,
                    on_close: move |_| is_category_modal_open.set(false),
                    on_save: move |_cat: CustomCategory| {}
                }
            }

            if is_checksum_modal_open() {
                ChecksumModal {
                    file_path: targeted_checksum_path(),
                    on_close: move |_| is_checksum_modal_open.set(false),
                }
            }

            if is_edit_modal_open() {
                if let Some(item) = targeted_item() {
                    EditDownloadModal {
                        item: item,
                        on_close: move |_| is_edit_modal_open.set(false),
                        on_save: move |(id, new_url, new_name)| {
                            if let Some(it) = downloads.write().iter_mut().find(|d| d.id == id) {
                                it.url = new_url;
                                it.name = new_name;
                            }
                        }
                    }
                }
            }

            if is_per_host_modal_open() {
                PerHostModal {
                    on_close: move |_| is_per_host_modal_open.set(false),
                }
            }

            if is_about_modal_open() {
                AboutModal {
                    on_close: move |_| is_about_modal_open.set(false),
                }
            }

            DownloadErrorModal {
                theme: theme,
                is_open: is_error_modal_open,
                error_title: error_dialog_title.read().clone(),
                error_message: error_dialog_message.read().clone(),
                http_status: Some(404),
                on_retry: move |_| {
                    if let Some(item) = targeted_item.peek().clone() {
                        spawn(async move {
                            #[derive(serde::Serialize)]
                            struct IdPayload { id: String }
                            let _ = invoke::<(), _>("resume_download", &IdPayload { id: item.id }).await;
                        });
                    }
                },
                on_refresh_link: move |_| {
                    is_edit_modal_open.set(true);
                },
            }

            QRCodeModal {
                theme: theme,
                is_open: is_qr_modal_open,
                title: "Share Download / Mobile Sync".to_string(),
                data: qr_dialog_url.read().clone(),
            }

            if is_confirm_delete_open() {
                ConfirmModal {
                    kind: ConfirmKind::DeleteDownload,
                    title: "Delete Downloads".to_string(),
                    message: format!("Are you sure you want to remove {} selected download(s) from the list?", selected_ids.peek().len()),
                    on_close: move |_| is_confirm_delete_open.set(false),
                    on_confirm: move |_delete_file_from_disk: bool| {
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
                        is_confirm_delete_open.set(false);
                    }
                }
            }
        }
    }
}
