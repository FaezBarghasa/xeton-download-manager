use dioxus::prelude::*;
use crate::theme::ThemePalette;
use xeton_models::{DownloadItem, DownloadStatus};

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_speed(speed: u64) -> String {
    if speed == 0 {
        return "-".into();
    }
    format!("{}/s", format_size(speed))
}

#[component]
pub fn DownloadTable(
    theme: ThemePalette,
    downloads: ReadOnlySignal<Vec<DownloadItem>>,
    selected_ids: Signal<std::collections::HashSet<String>>,
) -> Element {
    let items = downloads.read();

    rsx! {
        div {
            class: "table-container",
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                background-color: {theme.background};
                overflow: hidden;
            ",
            // Table Header
            div {
                style: "
                    display: grid;
                    grid-template-columns: 40px 2.5fr 1fr 1.5fr 1fr 1fr 1fr;
                    padding: 8px 12px;
                    background-color: {theme.surface};
                    border-bottom: 1px solid {theme.border};
                    font-size: 12px;
                    font-weight: 600;
                    color: {theme.on_surface};
                ",
                div { "" }
                div { "Name" }
                div { "Size" }
                div { "Progress" }
                div { "Speed" }
                div { "ETA" }
                div { "Status" }
            }
            // Table Body
            div {
                style: "flex: 1; overflow-y: auto;",
                if items.is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: {theme.on_surface}; opacity: 0.6; font-size: 14px;",
                        "No downloads found in this category."
                    }
                } else {
                    for item in items.iter() {
                        {
                            let item_id = item.id.clone();
                            let is_checked = selected_ids.read().contains(&item.id);
                            let percent = if let Some(total) = item.total_size {
                                if total > 0 {
                                    (item.downloaded_size as f64 / total as f64) * 100.0
                                } else { 0.0 }
                            } else { 0.0 };

                            let status_color = match item.status {
                                DownloadStatus::Downloading => theme.primary,
                                DownloadStatus::Completed => theme.success,
                                DownloadStatus::Failed => theme.error,
                                DownloadStatus::Paused => theme.warning,
                                _ => theme.on_surface,
                            };

                            let id_for_click = item_id.clone();
                            rsx! {
                                div {
                                    key: "{item.id}",
                                    onclick: move |_| {
                                        let mut ids = selected_ids.write();
                                        if ids.contains(&id_for_click) {
                                            ids.remove(&id_for_click);
                                        } else {
                                            ids.insert(id_for_click.clone());
                                        }
                                    },
                                    style: "
                                        display: grid;
                                        grid-template-columns: 40px 2.5fr 1fr 1.5fr 1fr 1fr 1fr;
                                        padding: 10px 12px;
                                        border-bottom: 1px solid {theme.border};
                                        font-size: 13px;
                                        align-items: center;
                                        background-color: {if is_checked { theme.surface } else { 'transparent' }};
                                        cursor: pointer;
                                    ",
                                    div {
                                        input {
                                            r#type: "checkbox",
                                            checked: is_checked,
                                            onchange: move |_| {}
                                        }
                                    }
                                    div {
                                        style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 500; color: {theme.on_background};",
                                        "{item.name}"
                                    }
                                    div {
                                        style: "color: {theme.on_background}; opacity: 0.8;",
                                        "{format_size(item.total_size.unwrap_or(0))}"
                                    }
                                    div {
                                        style: "display: flex; flex-direction: column; gap: 4px; padding-right: 12px;",
                                        div {
                                            style: "height: 6px; width: 100%; background-color: {theme.border}; border-radius: 3px; overflow: hidden;",
                                            div {
                                                style: "height: 100%; width: {percent:.1}%; background-color: {theme.primary}; border-radius: 3px; transition: width 0.1s ease;",
                                            }
                                        }
                                        span {
                                            style: "font-size: 10px; color: {theme.on_background}; opacity: 0.7;",
                                            "{percent:.1}% ({format_size(item.downloaded_size)})"
                                        }
                                    }
                                    div {
                                        style: "color: {theme.on_background};",
                                        "{format_speed(item.speed)}"
                                    }
                                    div {
                                        style: "color: {theme.on_background}; opacity: 0.8;",
                                        if let Some(eta) = item.eta_seconds {
                                            "{eta}s"
                                        } else {
                                            "-"
                                        }
                                    }
                                    div {
                                        style: "color: {status_color}; font-weight: 600; font-size: 12px;",
                                        "{item.status:?}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
