use dioxus::prelude::*;
use crate::components::icons::{IconClose, IconMaximize, IconMinimize};
use crate::theme::ThemePalette;

#[component]
pub fn Titlebar(theme: ThemePalette, title: String) -> Element {
    rsx! {
        div {
            class: "titlebar",
            style: "
                display: flex;
                align-items: center;
                justify-content: space-between;
                height: 38px;
                background-color: {theme.surface};
                border-bottom: 1px solid {theme.border};
                user-select: none;
                padding-left: 12px;
            ",
            // Drag Region
            div {
                "data-tauri-drag-region": "true",
                style: "flex: 1; display: flex; align-items: center; height: 100%; cursor: default;",
                span {
                    style: "font-size: 13px; font-weight: 600; color: {theme.on_surface};",
                    "{title}"
                }
            }
            // Window Action Buttons
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
