use crate::theme::ThemePalette;
use dioxus::prelude::*;
use xeton_models::{BatchDownloadItem, BatchDownloadRequest, CategoryType};

#[component]
pub fn BatchDownloadModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    on_submit: EventHandler<BatchDownloadRequest>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut raw_urls = use_signal(|| "".to_string());
    let mut parsed_items = use_signal(Vec::<BatchDownloadItem>::new);
    let mut output_dir = use_signal(|| "~/Downloads".to_string());
    let mut start_now = use_signal(|| true);

    let mut parse_urls = move || {
        let text = raw_urls.read().clone();
        let lines: Vec<&str> = text.lines().collect();
        let mut list = Vec::new();
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || !trimmed.starts_with("http") {
                continue;
            }
            let name = trimmed.split('/').last().unwrap_or("file").to_string();
            let category = if name.ends_with(".zip") || name.ends_with(".tar.gz") {
                CategoryType::Compressed
            } else if name.ends_with(".mp4") || name.ends_with(".mkv") {
                CategoryType::Videos
            } else if name.ends_with(".pdf") {
                CategoryType::Documents
            } else {
                CategoryType::Other
            };
            list.push(BatchDownloadItem {
                url: trimmed.to_string(),
                file_name: name,
                size: None,
                selected: true,
                category,
            });
        }
        *parsed_items.write() = list;
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
                    width: 720px; max-height: 85vh;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px; padding: 22px;
                    display: flex; flex-direction: column; gap: 16px;
                    box-shadow: 0 16px 36px rgba(0,0,0,0.6);
                ",
                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid {theme.border}; padding-bottom: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "font-size: 16px; font-weight: 700; color: {theme.on_surface};", "Add Multiple / Batch Downloads" }
                        span { style: "font-size: 12px; background: {theme.primary}22; color: {theme.primary}; padding: 2px 8px; border-radius: 10px;", "{parsed_items.read().len()} links" }
                    }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: none; color: {theme.on_surface}; opacity: 0.7; cursor: pointer; font-size: 16px;",
                        "✕"
                    }
                }

                // URL Input Textarea
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    div {
                        style: "display: flex; justify-content: space-between;",
                        label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Paste URLs (one per line):" }
                        button {
                            onclick: move |_| parse_urls(),
                            style: "background: {theme.primary}22; border: 1px solid {theme.primary}; color: {theme.primary}; border-radius: 4px; padding: 2px 10px; font-size: 11px; font-weight: 600; cursor: pointer;",
                            "Extract Links"
                        }
                    }
                    textarea {
                        rows: "4",
                        value: "{raw_urls}",
                        oninput: move |e| *raw_urls.write() = e.value(),
                        placeholder: "https://example.com/file1.zip\nhttps://example.com/file2.zip",
                        style: "
                            background: {theme.background}; color: {theme.on_background};
                            border: 1px solid {theme.border}; border-radius: 6px; padding: 8px 12px;
                            font-size: 12px; font-family: monospace; resize: vertical; outline: none;
                        "
                    }
                }

                // Table of Parsed Links
                div {
                    style: "
                        flex: 1; max-height: 220px; overflow-y: auto;
                        border: 1px solid {theme.border}; border-radius: 6px;
                        background: {theme.background};
                    ",
                    table {
                        style: "width: 100%; border-collapse: collapse; font-size: 12px; text-align: left;",
                        thead {
                            style: "position: sticky; top: 0; background: {theme.surface}; border-bottom: 1px solid {theme.border};",
                            tr {
                                th { style: "padding: 8px; width: 40px;", "Select" }
                                th { style: "padding: 8px;", "Filename" }
                                th { style: "padding: 8px;", "Target URL" }
                            }
                        }
                        tbody {
                            for (idx, item) in parsed_items.read().iter().enumerate() {
                                tr {
                                    style: "border-bottom: 1px solid {theme.border}44;",
                                    td {
                                        style: "padding: 8px; text-align: center;",
                                        input {
                                            r#type: "checkbox",
                                            checked: item.selected,
                                            onchange: move |_| {
                                                let mut items = parsed_items.write();
                                                items[idx].selected = !items[idx].selected;
                                            }
                                        }
                                    }
                                    td {
                                        style: "padding: 8px; font-weight: 500; color: {theme.on_surface};",
                                        "{item.file_name}"
                                    }
                                    td {
                                        style: "padding: 8px; color: {theme.on_surface}; opacity: 0.6; overflow: hidden; text-overflow: ellipsis; max-width: 280px; white-space: nowrap;",
                                        "{item.url}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Destination Directory & Options
                div {
                    style: "display: flex; gap: 12px; align-items: center;",
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 11px; font-weight: 600; color: {theme.on_surface};", "Download Folder" }
                        input {
                            r#type: "text",
                            value: "{output_dir}",
                            oninput: move |e| *output_dir.write() = e.value(),
                            style: "background: {theme.background}; border: 1px solid {theme.border}; border-radius: 4px; padding: 6px 10px; font-size: 12px; color: {theme.on_background}; outline: none;"
                        }
                    }
                    label {
                        style: "display: flex; align-items: center; gap: 6px; font-size: 12px; color: {theme.on_surface}; margin-top: 18px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: *start_now.read(),
                            onchange: move |e| *start_now.write() = e.checked()
                        }
                        "Start Immediately"
                    }
                }

                // Footer Buttons
                div {
                    style: "display: flex; justify-content: flex-end; gap: 10px; margin-top: 6px;",
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: 1px solid {theme.border}; color: {theme.on_surface}; border-radius: 4px; padding: 6px 16px; font-size: 12px; cursor: pointer;",
                        "Cancel"
                    }
                    button {
                        onclick: move |_| {
                            let req = BatchDownloadRequest {
                                items: parsed_items.read().clone(),
                                output_dir: Some(output_dir.read().clone()),
                                queue_id: None,
                                start_now: *start_now.read(),
                            };
                            on_submit.call(req);
                            *is_open.write() = false;
                        },
                        style: "background: {theme.primary}; border: none; color: #FFFFFF; font-weight: 600; border-radius: 4px; padding: 6px 18px; font-size: 12px; cursor: pointer;",
                        "Add Selected to Queue"
                    }
                }
            }
        }
    }
}
