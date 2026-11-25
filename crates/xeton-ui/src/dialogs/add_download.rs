use crate::theme::ThemePalette;
use dioxus::prelude::*;
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

    let mut active_tab = use_signal(|| "general"); // "general", "auth", "connection"
    let mut url = use_signal(|| "".to_string());
    let mut file_name = use_signal(|| "".to_string());
    let mut output_dir = use_signal(|| "~/Downloads".to_string());
    let category = use_signal(|| CategoryType::All);
    let mut selected_queue = use_signal(|| "Default Queue".to_string());
    let mut threads = use_signal(|| 8u32);
    let mut auth_user = use_signal(|| "".to_string());
    let mut auth_pass = use_signal(|| "".to_string());
    let mut custom_ua = use_signal(|| "".to_string());
    let mut custom_cookie = use_signal(|| "".to_string());

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
                    width: 560px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    box-shadow: 0 12px 32px rgba(0,0,0,0.5);
                    display: flex; flex-direction: column;
                    overflow: hidden;
                ",
                // Header
                div {
                    style: "
                        display: flex; justify-content: space-between; align-items: center;
                        padding: 14px 20px; border-bottom: 1px solid {theme.border};
                    ",
                    span { style: "font-size: 15px; font-weight: 700; color: {theme.on_surface};", "Add Download" }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: none; color: {theme.on_surface_muted}; cursor: pointer; font-size: 16px;",
                        "✕"
                    }
                }

                // Tab Switcher
                div {
                    style: "
                        display: flex; gap: 4px; padding: 0 16px;
                        background-color: {theme.background}; border-bottom: 1px solid {theme.border};
                    ",
                    {
                        let is_gen = *active_tab.read() == "general";
                        let border_color = if is_gen { theme.primary } else { "transparent" };
                        let text_color = if is_gen { theme.primary } else { theme.on_surface_muted };
                        rsx! {
                            button {
                                onclick: move |_| active_tab.set("general"),
                                style: "
                                    padding: 10px 14px; background: transparent; border: none;
                                    border-bottom: 2px solid {border_color};
                                    color: {text_color};
                                    font-size: 13px; font-weight: 600; cursor: pointer;
                                ",
                                "General"
                            }
                        }
                    }
                    {
                        let is_auth = *active_tab.read() == "auth";
                        let border_color = if is_auth { theme.primary } else { "transparent" };
                        let text_color = if is_auth { theme.primary } else { theme.on_surface_muted };
                        rsx! {
                            button {
                                onclick: move |_| active_tab.set("auth"),
                                style: "
                                    padding: 10px 14px; background: transparent; border: none;
                                    border-bottom: 2px solid {border_color};
                                    color: {text_color};
                                    font-size: 13px; font-weight: 600; cursor: pointer;
                                ",
                                "Authentication"
                            }
                        }
                    }
                    {
                        let is_conn = *active_tab.read() == "connection";
                        let border_color = if is_conn { theme.primary } else { "transparent" };
                        let text_color = if is_conn { theme.primary } else { theme.on_surface_muted };
                        rsx! {
                            button {
                                onclick: move |_| active_tab.set("connection"),
                                style: "
                                    padding: 10px 14px; background: transparent; border: none;
                                    border-bottom: 2px solid {border_color};
                                    color: {text_color};
                                    font-size: 13px; font-weight: 600; cursor: pointer;
                                ",
                                "Connections & Speed"
                            }
                        }
                    }
                }

                // Tab Body (Fixed height to eliminate layout shift across tabs)
                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 14px; height: 248px; overflow-y: auto;",
                    if *active_tab.read() == "general" {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",
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
                                        background-color: {theme.background}; border: 1px solid {theme.border};
                                        color: {theme.on_background}; font-size: 13px; outline: none;
                                    "
                                }
                            }
                            div {
                                style: "display: flex; gap: 12px;",
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; gap: 6px;",
                                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "File Name (Optional)" }
                                    input {
                                        r#type: "text",
                                        placeholder: "Auto-detected from server",
                                        value: "{file_name}",
                                        oninput: move |e| *file_name.write() = e.value(),
                                        style: "
                                            padding: 8px 12px; border-radius: 4px;
                                            background-color: {theme.background}; border: 1px solid {theme.border};
                                            color: {theme.on_background}; font-size: 13px; outline: none;
                                        "
                                    }
                                }
                                div {
                                    style: "width: 140px; display: flex; flex-direction: column; gap: 6px;",
                                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Queue" }
                                    select {
                                        value: "{selected_queue}",
                                        onchange: move |e| selected_queue.set(e.value()),
                                        style: "
                                            padding: 8px; border-radius: 4px;
                                            background-color: {theme.background}; border: 1px solid {theme.border};
                                            color: {theme.on_background}; font-size: 13px; outline: none;
                                        ",
                                        option { value: "Default Queue", "Default Queue" }
                                        option { value: "Night Queue", "Night Queue" }
                                    }
                                }
                            }
                            div {
                                style: "display: flex; flex-direction: column; gap: 6px;",
                                label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Save Destination" }
                                input {
                                    r#type: "text",
                                    value: "{output_dir}",
                                    oninput: move |e| output_dir.set(e.value()),
                                    style: "
                                        padding: 8px 12px; border-radius: 4px;
                                        background-color: {theme.background}; border: 1px solid {theme.border};
                                        color: {theme.on_background}; font-size: 13px; outline: none;
                                    "
                                }
                            }
                        }
                    } else if *active_tab.read() == "auth" {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",
                            div {
                                style: "display: flex; gap: 12px;",
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; gap: 6px;",
                                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "HTTP Username" }
                                    input {
                                        r#type: "text",
                                        placeholder: "Optional",
                                        value: "{auth_user}",
                                        oninput: move |e| auth_user.set(e.value()),
                                        style: "padding: 8px 12px; border-radius: 4px; background-color: {theme.background}; border: 1px solid {theme.border}; color: {theme.on_background}; font-size: 13px; outline: none;"
                                    }
                                }
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; gap: 6px;",
                                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "HTTP Password" }
                                    input {
                                        r#type: "password",
                                        placeholder: "Optional",
                                        value: "{auth_pass}",
                                        oninput: move |e| auth_pass.set(e.value()),
                                        style: "padding: 8px 12px; border-radius: 4px; background-color: {theme.background}; border: 1px solid {theme.border}; color: {theme.on_background}; font-size: 13px; outline: none;"
                                    }
                                }
                            }
                            div {
                                style: "display: flex; flex-direction: column; gap: 6px;",
                                label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Custom User-Agent" }
                                input {
                                    r#type: "text",
                                    placeholder: "Default (Mozilla/5.0...)",
                                    value: "{custom_ua}",
                                    oninput: move |e| custom_ua.set(e.value()),
                                    style: "padding: 8px 12px; border-radius: 4px; background-color: {theme.background}; border: 1px solid {theme.border}; color: {theme.on_background}; font-size: 13px; outline: none;"
                                }
                            }
                            div {
                                style: "display: flex; flex-direction: column; gap: 6px;",
                                label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Cookie Header" }
                                input {
                                    r#type: "text",
                                    placeholder: "session_id=...; token=...",
                                    value: "{custom_cookie}",
                                    oninput: move |e| custom_cookie.set(e.value()),
                                    style: "padding: 8px 12px; border-radius: 4px; background-color: {theme.background}; border: 1px solid {theme.border}; color: {theme.on_background}; font-size: 13px; outline: none;"
                                }
                            }
                        }
                    } else {
                        div {
                            style: "display: flex; flex-direction: column; gap: 16px;",
                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",
                                div {
                                    style: "display: flex; justify-content: space-between; align-items: center;",
                                    label { style: "font-size: 12px; font-weight: 600; color: {theme.on_surface};", "Number of Threads / Partitions" }
                                    span { style: "font-size: 13px; font-weight: 700; color: {theme.primary};", "{threads}" }
                                }
                                input {
                                    r#type: "range",
                                    min: "1",
                                    max: "256",
                                    value: "{threads}",
                                    oninput: move |e| {
                                        if let Ok(val) = e.value().parse::<u32>() {
                                            threads.set(val);
                                        }
                                    },
                                    style: "width: 100%; cursor: pointer;"
                                }
                                span { style: "font-size: 11px; color: {theme.on_surface_muted};", "Configurable up to 256 parallel connections. Tokio green tasks maximize line-rate throughput without OS thread contention." }
                            }
                        }
                    }
                }

                // Footer
                div {
                    style: "
                        display: flex; justify-content: flex-end; gap: 10px;
                        padding: 14px 20px; border-top: 1px solid {theme.border};
                        background-color: {theme.surface};
                    ",
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "
                            padding: 8px 16px; border-radius: 6px;
                            background-color: transparent; border: 1px solid {theme.border};
                            color: {theme.on_surface}; font-size: 13px; cursor: pointer;
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
                                    output_dir: Some(output_dir.read().clone()),
                                    category: Some(*category.read()),
                                    connections: Some(*threads.read()),
                                    start_now: true,
                                });
                                *is_open.write() = false;
                            }
                        },
                        style: "
                            padding: 8px 18px; border-radius: 6px;
                            background-color: {theme.primary}; border: none;
                            color: {theme.on_primary}; font-size: 13px; font-weight: 600;
                            cursor: pointer;
                        ",
                        "Start Download"
                    }
                }
            }
        }
    }
}
