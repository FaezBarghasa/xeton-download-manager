use crate::components::icons::{IconChevronDown, IconChevronUp};
use crate::theme::ThemePalette;
use dioxus::prelude::*;
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

fn item_progress(item: &DownloadItem) -> f64 {
    if let Some(total) = item.total_size {
        if total > 0 {
            return (item.downloaded_size as f64 / total as f64) * 100.0;
        }
    }
    0.0
}

#[component]
pub fn DownloadTable(
    theme: ThemePalette,
    downloads: ReadOnlySignal<Vec<DownloadItem>>,
    selected_ids: Signal<std::collections::HashSet<String>>,
    #[props(default = "normal".to_string())] density: String,
) -> Element {
    let mut sort_col = use_signal(|| "name");
    let mut sort_asc = use_signal(|| true);
    let mut focused_idx = use_signal(|| 0usize);

    let mut sorted_items = downloads.read().clone();
    let col = *sort_col.read();
    let asc = *sort_asc.read();
    sorted_items.sort_by(|a, b| {
        let ord = match col {
            "size" => a.total_size.unwrap_or(0).cmp(&b.total_size.unwrap_or(0)),
            "progress" => item_progress(a)
                .partial_cmp(&item_progress(b))
                .unwrap_or(std::cmp::Ordering::Equal),
            "speed" => a.speed.cmp(&b.speed),
            "status" => format!("{:?}", a.status).cmp(&format!("{:?}", b.status)),
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        };
        if asc { ord } else { ord.reverse() }
    });

    let row_padding = match density.as_str() {
        "compact" => "6px 12px",
        "comfortable" => "14px 12px",
        _ => "10px 12px",
    };

    let item_ids: Vec<String> = sorted_items.iter().map(|i| i.id.clone()).collect();
    let total_rows = item_ids.len();

    rsx! {
        div {
            class: "table-container",
            tabindex: "0",
            onkeydown: move |e| {
                if total_rows == 0 {
                    return;
                }
                match e.key() {
                    Key::ArrowDown => {
                        let next = (*focused_idx.peek() + 1).min(total_rows.saturating_sub(1));
                        focused_idx.set(next);
                    }
                    Key::ArrowUp => {
                        let prev = focused_idx.peek().saturating_sub(1);
                        focused_idx.set(prev);
                    }
                    Key::Character(ref c) if c == " " => {
                        if let Some(id) = item_ids.get(*focused_idx.peek()) {
                            let mut ids = selected_ids.write();
                            if ids.contains(id) {
                                ids.remove(id);
                            } else {
                                ids.insert(id.clone());
                            }
                        }
                    }
                    _ => {}
                }
            },
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                background-color: {theme.background};
                overflow: hidden;
                outline: none;
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
                    color: {theme.on_surface_muted};
                ",
                div {
                    input {
                        r#type: "checkbox",
                        checked: !sorted_items.is_empty() && selected_ids.read().len() == sorted_items.len(),
                        onchange: move |_| {
                            let all_len = sorted_items.len();
                            let mut ids = selected_ids.write();
                            if ids.len() == all_len && all_len > 0 {
                                ids.clear();
                            } else {
                                for it in sorted_items.iter() {
                                    ids.insert(it.id.clone());
                                }
                            }
                        }
                    }
                }
                for (key_id, label) in [
                    ("name", "Name"),
                    ("size", "Size"),
                    ("progress", "Progress"),
                    ("speed", "Speed"),
                    ("eta", "ETA"),
                    ("status", "Status"),
                ] {
                    div {
                        class: "xeton-table-header-cell",
                        style: if *sort_col.read() == key_id { format!("color: {};", theme.primary) } else { String::new() },
                        onclick: move |_| {
                            if *sort_col.peek() == key_id {
                                let curr = *sort_asc.peek();
                                sort_asc.set(!curr);
                            } else {
                                sort_col.set(key_id);
                                sort_asc.set(true);
                            }
                        },
                        span { "{label}" }
                        if *sort_col.read() == key_id {
                            if *sort_asc.read() {
                                IconChevronUp { size: 12 }
                            } else {
                                IconChevronDown { size: 12 }
                            }
                        }
                    }
                }
            }
            // Table Body
            div {
                style: "flex: 1; overflow-y: auto;",
                if sorted_items.is_empty() {
                    div {
                        style: "padding: 48px; text-align: center; color: {theme.on_surface_muted}; font-size: 14px;",
                        "No downloads found in this category."
                    }
                } else {
                    for (idx, item) in sorted_items.iter().enumerate() {
                        {
                            let item_id = item.id.clone();
                            let is_checked = selected_ids.read().contains(&item.id);
                            let is_focused = *focused_idx.read() == idx;
                            let percent = item_progress(item);
                            let is_downloading = item.status == DownloadStatus::Downloading;

                            let status_color = match item.status {
                                DownloadStatus::Downloading => theme.primary,
                                DownloadStatus::Completed => theme.success,
                                DownloadStatus::Failed => theme.error,
                                DownloadStatus::Paused => theme.warning,
                                _ => theme.on_surface_muted,
                            };

                            let id_for_click = item_id.clone();
                            let row_class = if is_checked { "xeton-table-row is-selected" } else { "xeton-table-row" };
                            let focus_border = if is_focused { format!("box-shadow: inset 2px 0 0 {};", theme.primary) } else { String::new() };
                            let bar_class = if is_downloading { "xeton-progress-active" } else { "" };
                            let bar_bg = if is_downloading { String::new() } else { format!("background-color: {};", status_color) };

                            rsx! {
                                div {
                                    key: "{item.id}",
                                    class: "{row_class}",
                                    onclick: move |_| {
                                        focused_idx.set(idx);
                                        let mut ids = selected_ids.write();
                                        if ids.contains(&id_for_click) {
                                            ids.remove(&id_for_click);
                                        } else {
                                            ids.insert(id_for_click.clone());
                                        }
                                    },
                                    style: "padding: {row_padding}; {focus_border}",
                                    div {
                                        input {
                                            r#type: "checkbox",
                                            checked: is_checked,
                                            onchange: move |_| {}
                                        }
                                    }
                                    div {
                                        style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 500; color: {theme.on_background}; padding-right: 12px;",
                                        "{item.name}"
                                    }
                                    div {
                                        class: "tabular-nums",
                                        style: "color: {theme.on_surface_muted};",
                                        "{format_size(item.total_size.unwrap_or(0))}"
                                    }
                                    div {
                                        style: "display: flex; flex-direction: column; gap: 4px; padding-right: 16px;",
                                        div {
                                            style: "height: 6px; width: 100%; background-color: {theme.border}; border-radius: 3px; overflow: hidden;",
                                            div {
                                                class: "{bar_class}",
                                                style: "height: 100%; width: {percent:.1}%; {bar_bg} border-radius: 3px; transition: width 0.2s var(--ease-expo);",
                                            }
                                        }
                                        span {
                                            class: "tabular-nums",
                                            style: "font-size: 11px; color: {theme.on_surface_muted};",
                                            "{percent:.1}% ({format_size(item.downloaded_size)})"
                                        }
                                    }
                                    div {
                                        class: "tabular-nums",
                                        style: "color: {theme.on_background}; font-weight: 500;",
                                        "{format_speed(item.speed)}"
                                    }
                                    div {
                                        class: "tabular-nums",
                                        style: "color: {theme.on_surface_muted};",
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
