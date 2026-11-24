use dioxus::prelude::*;
use xeton_models::{DownloadItem, DownloadStatus};
use crate::theme::ThemePalette;

#[component]
pub fn MobileCard(
    theme: ThemePalette,
    item: DownloadItem,
    on_click: EventHandler<()>,
    on_toggle_pause: EventHandler<()>,
) -> Element {
    let progress_pct = if let Some(total) = item.total_size {
        if total > 0 {
            (item.downloaded_size as f64 / total as f64 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    let status_color = match item.status {
        DownloadStatus::Downloading => theme.primary,
        DownloadStatus::Completed => "#4CAF50",
        DownloadStatus::Paused => "#FFA000",
        DownloadStatus::Failed => "#E53935",
        DownloadStatus::Pending => theme.on_surface_muted,
    };

    let speed_text = if item.speed > 0 {
        format!("{:.2} MB/s", item.speed as f64 / (1024.0 * 1024.0))
    } else {
        "-".to_string()
    };

    rsx! {
        div {
            onclick: move |_| on_click.call(()),
            style: "
                background-color: {theme.surface};
                border: 1px solid {theme.border};
                border-radius: 12px;
                padding: 14px 16px;
                margin-bottom: 10px;
                cursor: pointer;
                box-sizing: border-box;
            ",
            // Title & Pause/Resume Action
            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 8px; margin-bottom: 8px;",
                div {
                    style: "flex: 1; font-size: 14px; font-weight: 600; color: {theme.on_surface}; word-break: break-all; line-height: 1.3;",
                    "{item.name}"
                }
                button {
                    onclick: move |e| {
                        e.stop_propagation();
                        on_toggle_pause.call(());
                    },
                    style: "
                        width: 32px; height: 32px; border-radius: 50%; border: none;
                        background-color: {theme.background}; color: {theme.on_surface};
                        display: flex; align-items: center; justify-content: center; cursor: pointer; flex-shrink: 0;
                    ",
                    if item.status == DownloadStatus::Downloading {
                        svg {
                            view_box: "0 0 24 24", width: "16", height: "16", stroke: "currentColor", fill: "none", stroke_width: "2.5",
                            line { x1: "10", y1: "4", x2: "10", y2: "20" }
                            line { x1: "14", y1: "4", x2: "14", y2: "20" }
                        }
                    } else {
                        svg {
                            view_box: "0 0 24 24", width: "16", height: "16", stroke: "currentColor", fill: "none", stroke_width: "2.5",
                            polygon { points: "5 3 19 12 5 21 5 3" }
                        }
                    }
                }
            }
            // Progress Bar
            div {
                style: "
                    width: 100%; height: 6px; background-color: {theme.border};
                    border-radius: 3px; overflow: hidden; margin-bottom: 8px;
                ",
                div {
                    style: "
                        height: 100%; width: {progress_pct}%; background-color: {status_color};
                        border-radius: 3px; transition: width 0.3s ease;
                    "
                }
            }
            // Stats Row: Progress %, Speed, Status
            div {
                style: "display: flex; justify-content: space-between; align-items: center; font-size: 12px; color: {theme.on_surface_muted};",
                span {
                    "{progress_pct:.1}%"
                }
                span {
                    "{speed_text}"
                }
                span {
                    style: "color: {status_color}; font-weight: 600;",
                    format!("{:?}", item.status)
                }
            }
        }
    }
}
