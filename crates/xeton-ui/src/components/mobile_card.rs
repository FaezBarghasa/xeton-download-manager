use crate::components::icons::{IconPause, IconResume};
use crate::theme::ThemePalette;
use dioxus::prelude::*;
use xeton_models::{DownloadItem, DownloadStatus};

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

    let is_downloading = item.status == DownloadStatus::Downloading;
    let status_color = match item.status {
        DownloadStatus::Downloading => theme.primary,
        DownloadStatus::Completed => theme.success,
        DownloadStatus::Paused => theme.warning,
        DownloadStatus::Failed => theme.error,
        DownloadStatus::Pending => theme.on_surface_muted,
        DownloadStatus::Scheduled => theme.info,
        DownloadStatus::Canceled => theme.on_surface_muted,
    };

    let speed_text = if item.speed > 0 {
        format!("{:.2} MB/s", item.speed as f64 / (1024.0 * 1024.0))
    } else {
        "-".to_string()
    };

    let bar_class = if is_downloading {
        "xeton-progress-active"
    } else {
        ""
    };
    let bar_bg = if is_downloading {
        String::new()
    } else {
        format!("background-color: {};", status_color)
    };

    rsx! {
        div {
            class: "xeton-mobile-card",
            onclick: move |_| on_click.call(()),
            // Title & Pause/Resume Action (48dp touch target)
            div {
                style: "display: flex; justify-content: space-between; align-items: center; gap: 10px; margin-bottom: 10px;",
                span {
                    style: "
                        font-size: 14px; font-weight: 600; color: {theme.on_surface};
                        overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;
                    ",
                    "{item.name}"
                }
                button {
                    class: "xeton-touch-target",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_toggle_pause.call(());
                    },
                    style: "
                        border-radius: 50%; border: 1px solid {theme.border};
                        background-color: {theme.background}; color: {theme.on_surface};
                        flex-shrink: 0;
                    ",
                    if is_downloading {
                        IconPause { size: 16 }
                    } else {
                        IconResume { size: 16 }
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
                    class: "{bar_class}",
                    style: "
                        height: 100%; width: {progress_pct:.1}%; {bar_bg}
                        border-radius: 3px; transition: width 0.25s var(--ease-expo);
                    "
                }
            }
            // Stats Row: Progress %, Speed, Status
            div {
                style: "display: flex; justify-content: space-between; align-items: center; font-size: 12px; color: {theme.on_surface_muted};",
                span {
                    class: "tabular-nums",
                    "{progress_pct:.1}%"
                }
                span {
                    class: "tabular-nums",
                    "{speed_text}"
                }
                span {
                    style: "color: {status_color}; font-weight: 600;",
                    "{item.status:?}"
                }
            }
        }
    }
}
