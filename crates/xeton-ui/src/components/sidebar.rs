use crate::components::icons::{IconPause, IconPlus, IconResume};
use crate::theme::ThemePalette;
use dioxus::prelude::*;
use xeton_models::CategoryType;

#[component]
pub fn Sidebar(
    theme: ThemePalette,
    selected_category: Signal<CategoryType>,
    download_counts: ReadOnlySignal<std::collections::HashMap<CategoryType, usize>>,
) -> Element {
    let categories = [
        (CategoryType::All, "All Downloads"),
        (CategoryType::Compressed, "Compressed"),
        (CategoryType::Documents, "Documents"),
        (CategoryType::Music, "Music"),
        (CategoryType::Programs, "Programs"),
        (CategoryType::Videos, "Videos"),
        (CategoryType::Other, "Other"),
    ];

    rsx! {
        div {
            class: "sidebar",
            style: "
                width: 220px;
                background-color: {theme.surface};
                border-right: 1px solid {theme.border};
                display: flex;
                flex-direction: column;
                padding: 12px 8px;
                height: 100%;
                box-sizing: border-box;
            ",
            div {
                style: "font-size: 11px; font-weight: 700; text-transform: uppercase; color: {theme.primary}; padding: 4px 8px; margin-bottom: 4px;",
                "Categories"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                for (cat, label) in categories.iter() {
                    {
                        let is_selected = *selected_category.read() == *cat;
                        let bg = if is_selected { theme.primary } else { "transparent" };
                        let fg = if is_selected { theme.on_primary } else { theme.on_surface };
                        let cat_copy = *cat;
                        let count = download_counts.read().get(cat).copied().unwrap_or(0);
                        rsx! {
                            button {
                                onclick: move |_| {
                                    *selected_category.write() = cat_copy;
                                },
                                style: "
                                    display: flex;
                                    align-items: center;
                                    justify-content: space-between;
                                    padding: 8px 12px;
                                    border-radius: 6px;
                                    border: none;
                                    background-color: {bg};
                                    color: {fg};
                                    font-size: 13px;
                                    font-weight: 500;
                                    cursor: pointer;
                                    text-align: left;
                                    transition: background-color 0.15s ease;
                                ",
                                span { "{label}" }
                                span {
                                    style: "font-size: 11px; opacity: 0.8;",
                                    "{count}"
                                }
                            }
                        }
                    }
                }
            }

            // Queues Section (Matching Desktop AB Download Manager)
            div {
                style: "margin-top: 20px; font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.04em; color: {theme.primary}; padding: 4px 8px; display: flex; justify-content: space-between; align-items: center;",
                span { "Queues" }
                button {
                    class: "xeton-icon-btn",
                    style: "min-width: 22px; min-height: 22px; padding: 2px;",
                    title: "Manage Queues",
                    onclick: move |_| {
                        spawn(async move {
                            let _ = crate::tauri_bindings::invoke_no_args::<Vec<xeton_models::QueueItem>>(
                                "get_queues",
                            )
                            .await;
                        });
                    },
                    IconPlus { size: 13 }
                }
            }
            div {
                style: "display: flex; flex-direction: column; gap: 2px;",
                button {
                    class: "xeton-btn",
                    style: "
                        justify-content: space-between; padding: 8px 12px; border: none;
                        border-radius: 6px; font-size: 13px; width: 100%;
                    ",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        IconResume { size: 12, color: Some(theme.primary.to_string()) }
                        span { "Default Queue" }
                    }
                    span { class: "tabular-nums", style: "font-size: 11px; color: {theme.on_surface_muted};", "3" }
                }
                button {
                    class: "xeton-btn",
                    style: "
                        justify-content: space-between; padding: 8px 12px; border: none;
                        border-radius: 6px; font-size: 13px; width: 100%; color: {theme.on_surface_muted};
                    ",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        IconPause { size: 12, color: Some(theme.on_surface_muted.to_string()) }
                        span { "Night Queue" }
                    }
                    span { class: "tabular-nums", style: "font-size: 11px; color: {theme.on_surface_muted};", "0" }
                }
            }
        }
    }
}
