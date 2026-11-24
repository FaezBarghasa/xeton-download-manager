use dioxus::prelude::*;
use crate::components::icons::{IconClose, IconMaximize, IconMinimize};
use crate::theme::ThemePalette;

#[component]
pub fn Titlebar(
    theme: ThemePalette,
    title: String,
    search_query: Signal<String>,
) -> Element {
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
            ",
            // Menu Items (File, Edit, View, Tools, Help)
            div {
                style: "display: flex; align-items: center; gap: 4px; margin-right: 12px;",
                span {
                    style: "font-weight: 700; color: {theme.primary}; margin-right: 8px; font-size: 14px;",
                    "ABDM"
                }
                for item in ["File", "Edit", "View", "Tools", "Help"].iter() {
                    button {
                        style: "
                            background: transparent; border: none;
                            color: {theme.on_surface}; opacity: 0.85;
                            font-size: 12px; font-weight: 500;
                            padding: 4px 8px; border-radius: 4px; cursor: pointer;
                        ",
                        "{item}"
                    }
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
                    IconMaximize { size: 12 }
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
