use crate::components::icons::{IconClose, IconMaximize, IconMinimize, IconXetonLogo};
use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn Titlebar(
    theme: ThemePalette,
    title: String,
    search_query: Signal<String>,
    on_menu_action: Option<EventHandler<String>>,
) -> Element {
    let mut active_menu = use_signal(|| Option::<String>::None);

    let menu_options = vec![
        (
            "File",
            vec![
                ("new_download", "New Download..."),
                ("batch_download", "Add Batch Downloads..."),
                ("separator", ""),
                ("exit", "Exit"),
            ],
        ),
        (
            "Edit",
            vec![("select_all", "Select All"), ("delete", "Delete Selected")],
        ),
        ("View", vec![("refresh", "Refresh List")]),
        (
            "Tools",
            vec![
                ("queue_manager", "Queue Manager..."),
                ("categories", "Manage Categories..."),
                ("checksum", "Verify File Integrity (Checksum)..."),
                ("per_host", "Per-Host Connection Rules..."),
                ("separator", ""),
                ("preferences", "Preferences & Settings..."),
            ],
        ),
        (
            "Help",
            vec![
                ("documentation", "Documentation"),
                ("check_updates", "Check for Updates..."),
                ("separator", ""),
                ("about", "About Xeton Download Manager"),
            ],
        ),
    ];

    rsx! {
        div {
            class: "titlebar",
            style: "
                display: flex;
                align-items: center;
                height: 40px;
                background-color: {theme.surface};
                border-bottom: 1px solid {theme.border};
                user-select: none;
                padding: 0 0 0 12px;
                font-size: 13px;
                position: relative;
                z-index: 40;
            ",
            // Menu Items (File, Edit, View, Tools, Help)
            div {
                style: "display: flex; align-items: center; gap: 6px; margin-right: 14px; position: relative;",
                IconXetonLogo { size: Some(22) }
                span {
                    style: "font-weight: 800; letter-spacing: 0.5px; color: {theme.primary}; margin-right: 10px; font-size: 13px;",
                    "XETON"
                }
                for (menu_name, items) in menu_options {
                    div {
                        key: "{menu_name}",
                        class: "relative",
                        button {
                            style: format!(
                                "background: transparent; border: none; color: {}; opacity: 0.85; font-size: 12px; font-weight: 500; padding: 4px 8px; border-radius: 4px; cursor: pointer;",
                                theme.on_surface
                            ),
                            onclick: move |_| {
                                let current = active_menu();
                                if current.as_deref() == Some(menu_name) {
                                    active_menu.set(None);
                                } else {
                                    active_menu.set(Some(menu_name.to_string()));
                                }
                            },
                            "{menu_name}"
                        }
                        if active_menu().as_deref() == Some(menu_name) {
                            div {
                                class: "absolute left-0 top-full mt-1 w-56 bg-surface border border-border rounded-lg shadow-xl py-1 z-50 animate-in fade-in duration-100",
                                for (action, label) in items {
                                    if action == "separator" {
                                        div { class: "border-t border-border my-1" }
                                    } else {
                                        button {
                                            key: "{action}",
                                            class: "w-full text-left px-3 py-1.5 text-xs text-on-surface hover:bg-surface-variant/50 transition-colors block",
                                            onclick: {
                                                let act = action.to_string();
                                                move |_| {
                                                    active_menu.set(None);
                                                    if let Some(ref handler) = on_menu_action {
                                                        handler.call(act.clone());
                                                    }
                                                }
                                            },
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Backdrop dismissal for menus
            if active_menu().is_some() {
                div {
                    class: "fixed inset-0 z-30",
                    onclick: move |_| active_menu.set(None),
                }
            }

            // Draggable Middle Area
            div {
                "data-tauri-drag-region": "true",
                style: "
                    flex: 1; display: flex; align-items: center; justify-content: center;
                    height: 100%; cursor: default;
                ",
                span {
                    style: "font-size: 12px; font-weight: 600; color: {theme.on_surface}; opacity: 0.5;",
                    "{title}"
                }
            }
            // Search Input in Titlebar
            div {
                style: "margin-right: 12px; display: flex; align-items: center;",
                input {
                    r#type: "text",
                    placeholder: "Search downloads...",
                    value: "{search_query}",
                    oninput: move |e| *search_query.write() = e.value(),
                    style: "
                        padding: 4px 10px; font-size: 12px;
                        border-radius: 4px; border: 1px solid {theme.border};
                        background-color: {theme.background}; color: {theme.on_background};
                        width: 180px; outline: none;
                    "
                }
            }
            // Window Action Controls
            div {
                style: "display: flex; height: 100%;",
                button {
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        crate::tauri_bindings::minimize_window();
                    },
                    style: "
                        display: flex; align-items: center; justify-content: center;
                        width: 44px; height: 100%; background: transparent; border: none;
                        color: {theme.on_surface}; cursor: pointer;
                    ",
                    IconMinimize { size: 14 }
                }
                button {
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        crate::tauri_bindings::toggle_maximize_window();
                    },
                    style: "
                        display: flex; align-items: center; justify-content: center;
                        width: 44px; height: 100%; background: transparent; border: none;
                        color: {theme.on_surface}; cursor: pointer;
                    ",
                    IconMaximize { size: 14 }
                }
                button {
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        crate::tauri_bindings::close_window();
                    },
                    style: "
                        display: flex; align-items: center; justify-content: center;
                        width: 44px; height: 100%; background: transparent; border: none;
                        color: {theme.on_surface}; cursor: pointer;
                    ",
                    IconClose { size: 14 }
                }
            }
        }
    }
}
