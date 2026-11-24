use dioxus::prelude::*;
use crate::theme::ThemePalette;
use xeton_models::{AddDownloadRequest, CategoryType};

#[component]
pub fn AddDownloadModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    on_submit: EventHandler<AddDownloadRequest>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut url = use_signal(|| "".to_string());
    let mut file_name = use_signal(|| "".to_string());
    let category = use_signal(|| CategoryType::All);

    rsx! {
        div {
            class: "modal-backdrop",
            style: "
                position: fixed; top: 0; left: 0; right: 0; bottom: 0;
                background-color: rgba(0, 0, 0, 0.65);
                display: flex; align-items: center; justify-content: center;
                z-index: 1000;
            ",
            div {
                class: "modal-dialog",
                style: "
                    width: 500px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    padding: 20px;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.5);
                    display: flex; flex-direction: column; gap: 16px;
                ",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span { style: "font-size: 16px; font-weight: 700; color: {theme.on_surface};", "Add New Download" }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: none; color: {theme.on_surface}; cursor: pointer;",
                        "✕"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Download URL" }
                    input {
                        r#type: "text",
                        placeholder: "https://example.com/file.zip",
                        value: "{url}",
                        oninput: move |e| *url.write() = e.value(),
                        style: "
                            padding: 8px 12px; border-radius: 4px;
                            background-color: {theme.background};
                            border: 1px solid {theme.border};
                            color: {theme.on_background};
                            font-size: 13px;
                        "
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "File Name (Optional)" }
                    input {
                        r#type: "text",
                        placeholder: "file.zip",
                        value: "{file_name}",
                        oninput: move |e| *file_name.write() = e.value(),
                        style: "
                            padding: 8px 12px; border-radius: 4px;
                            background-color: {theme.background};
                            border: 1px solid {theme.border};
                            color: {theme.on_background};
                            font-size: 13px;
                        "
                    }
                }
                div {
                    style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;",
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "
                            padding: 8px 16px; border-radius: 4px;
                            background-color: transparent;
                            border: 1px solid {theme.border};
                            color: {theme.on_surface}; cursor: pointer;
                        ",
                        "Cancel"
                    }
                    button {
                        onclick: move |_| {
                            let u = url.read().clone();
                            if !u.is_empty() {
                                on_submit.call(AddDownloadRequest {
                                    url: u,
                                    file_name: if file_name.read().is_empty() { None } else { Some(file_name.read().clone()) },
                                    output_dir: None,
                                    category: Some(*category.read()),
                                    connections: Some(8),
                                    start_now: true,
                                });
                                *is_open.write() = false;
                            }
                        },
                        style: "
                            padding: 8px 16px; border-radius: 4px;
                            background-color: {theme.primary};
                            border: none;
                            color: {theme.on_primary}; font-weight: 600; cursor: pointer;
                        ",
                        "Start Download"
                    }
                }
            }
        }
    }
}
