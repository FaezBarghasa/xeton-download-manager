use crate::theme::ThemePalette;
use dioxus::prelude::*;
use xeton_models::{DownloadItem, DownloadPartsInfo, DownloadStatus};

#[component]
pub fn DownloadDetailsModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    item: Signal<Option<DownloadItem>>,
    parts_info: Signal<Option<DownloadPartsInfo>>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let current = match item.read().clone() {
        Some(i) => i,
        None => return rsx! {},
    };

    let is_completed = current.status == DownloadStatus::Completed;
    let total_mb = current.total_size.unwrap_or(0) as f64 / (1024.0 * 1024.0);
    let downloaded_mb = current.downloaded_size as f64 / (1024.0 * 1024.0);
    let progress = if let Some(tot) = current.total_size {
        if tot > 0 {
            (current.downloaded_size as f64 / tot as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    rsx! {
        div {
            class: "modal-backdrop",
            style: "
                position: fixed; top: 0; left: 0; right: 0; bottom: 0;
                background-color: rgba(0, 0, 0, 0.7);
                display: flex; align-items: center; justify-content: center;
                z-index: 1000;
            ",
            div {
                class: "modal-dialog",
                style: "
                    width: 620px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px; padding: 22px;
                    display: flex; flex-direction: column; gap: 18px;
                    box-shadow: 0 16px 36px rgba(0,0,0,0.6);
                ",
                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid {theme.border}; padding-bottom: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "font-size: 16px; font-weight: 700; color: {theme.on_surface};", "{current.name}" }
                        span {
                            style: "
                                font-size: 11px; font-weight: 600; text-transform: uppercase;
                                padding: 2px 8px; border-radius: 12px;
                                background: if is_completed {{ \"#10B98122\" }} else {{ \"#38BDF822\" }};
                                color: if is_completed {{ \"#10B981\" }} else {{ \"#38BDF8\" }};
                            ",
                            "{current.status:?}"
                        }
                    }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: none; color: {theme.on_surface}; opacity: 0.7; cursor: pointer; font-size: 16px;",
                        "✕"
                    }
                }

                // File Metrics Grid
                div {
                    style: "
                        display: grid; grid-template-columns: 1fr 1fr; gap: 10px;
                        background: {theme.background}; padding: 12px; border-radius: 6px;
                        border: 1px solid {theme.border}66; font-size: 12px;
                    ",
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "Downloaded: " }
                        span { style: "font-weight: 600; color: {theme.on_background};", "{downloaded_mb:.2} MB / {total_mb:.2} MB" }
                    }
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "Transfer Rate: " }
                        span { style: "font-weight: 600; color: {theme.primary};", "{current.speed / 1024} KB/s" }
                    }
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "Connections: " }
                        span { style: "font-weight: 600; color: {theme.on_background};", "{current.connections} active threads" }
                    }
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "ETA: " }
                        span { style: "font-weight: 600; color: {theme.on_background};", "{current.eta_seconds.unwrap_or(0)}s" }
                    }
                }

                // Segmented Multi-Thread Chunk Visualizer
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    div {
                        style: "display: flex; justify-content: space-between; font-size: 11px; font-weight: 600; color: {theme.on_surface};",
                        span { "Parallel Stream Visualizer (Parts)" }
                        span { "{progress:.1}%" }
                    }
                    // Overall Progress Bar
                    div {
                        style: "width: 100%; height: 8px; background: {theme.border}; border-radius: 4px; overflow: hidden;",
                        div {
                            style: "width: {progress}%; height: 100%; background: linear-gradient(90deg, #00F2FE, #38BDF8); transition: width 0.3s ease;"
                        }
                    }
                    // Individual Part Chunks
                    if let Some(parts) = parts_info.read().as_ref() {
                        div {
                            style: "display: flex; gap: 4px; height: 16px; margin-top: 4px;",
                            for part in parts.parts.iter() {
                                {
                                    let part_prog = if part.end_byte > part.start_byte {
                                        ((part.current_byte - part.start_byte) as f64 / (part.end_byte - part.start_byte) as f64) * 100.0
                                    } else { 100.0 };
                                    rsx! {
                                        div {
                                            style: "
                                                flex: 1; height: 100%; background: {theme.background};
                                                border: 1px solid {theme.border}; border-radius: 2px;
                                                overflow: hidden; position: relative;
                                            ",
                                            title: "Part {part.part_id}: {part_prog:.0}% ({part.speed / 1024} KB/s)",
                                            div {
                                                style: "width: {part_prog}%; height: 100%; background: {theme.primary}; opacity: 0.85;"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // File Path and URL Details
                div {
                    style: "display: flex; flex-direction: column; gap: 6px; font-size: 11px;",
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "Destination: " }
                        span { style: "color: {theme.on_surface}; font-family: monospace;", "{current.file_path}" }
                    }
                    div {
                        span { style: "color: {theme.on_surface}; opacity: 0.6;", "Source URL: " }
                        span { style: "color: {theme.on_surface}; font-family: monospace; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{current.url}" }
                    }
                }

                // Actions Footer
                div {
                    style: "display: flex; justify-content: flex-end; gap: 10px; border-top: 1px solid {theme.border}; padding-top: 14px;",
                    if is_completed {
                        button {
                            onclick: move |_| {
                                // Open file
                            },
                            style: "background: {theme.primary}; border: none; color: #FFFFFF; font-weight: 600; border-radius: 4px; padding: 6px 16px; font-size: 12px; cursor: pointer;",
                            "Open File"
                        }
                    }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: 1px solid {theme.border}; color: {theme.on_surface}; border-radius: 4px; padding: 6px 14px; font-size: 12px; cursor: pointer;",
                        "Close"
                    }
                }
            }
        }
    }
}
