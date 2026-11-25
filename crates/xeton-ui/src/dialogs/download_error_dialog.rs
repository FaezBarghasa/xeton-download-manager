use crate::components::icons::{IconAlertTriangle, IconClose};
use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn DownloadErrorModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    error_title: String,
    error_message: String,
    http_status: Option<u16>,
    on_retry: EventHandler<()>,
    on_refresh_link: EventHandler<()>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut copied_toast = use_signal(|| false);

    let status_str = match http_status {
        Some(code) => format!("HTTP Status: {}", code),
        None => "Network Connection Failure".to_string(),
    };

    let full_error_details = format!(
        "Error: {}\nDetails: {}\n{}",
        error_title, error_message, status_str
    );

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
                class: "modal-card",
                style: "
                    width: 520px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    display: flex; flex-direction: column;
                    box-shadow: 0 12px 32px rgba(0,0,0,0.5);
                    overflow: hidden;
                ",
                // Modal Header
                div {
                    style: "
                        display: flex; align-items: center; justify-content: space-between;
                        padding: 14px 20px; border-bottom: 1px solid {theme.border};
                        background-color: rgba(234, 76, 60, 0.12);
                    ",
                    div {
                        style: "display: flex; align-items: center; gap: 10px;",
                        IconAlertTriangle { size: 18, color: Some(theme.error.to_string()) }
                        span {
                            style: "font-weight: 600; font-size: 15px; color: {theme.error};",
                            "Download Failed"
                        }
                    }
                    button {
                        class: "xeton-icon-btn",
                        onclick: move |_| is_open.set(false),
                        IconClose { size: 16 }
                    }
                }

                // Modal Body
                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 14px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; color: {theme.on_surface};",
                        "{error_title}"
                    }
                    div {
                        style: "
                            padding: 12px; border-radius: 6px;
                            background-color: {theme.background}; border: 1px solid {theme.border};
                            font-family: monospace; font-size: 12px; color: {theme.error};
                            max-height: 140px; overflow-y: auto; white-space: pre-wrap; word-break: break-all;
                        ",
                        "{full_error_details}"
                    }

                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; font-size: 12px; color: {theme.on_surface_muted};",
                        span { "You can refresh the download link or retry the connection." }
                        button {
                            onclick: move |_| {
                                copied_toast.set(true);
                            },
                            style: "
                                background: transparent; border: 1px solid {theme.border};
                                border-radius: 4px; padding: 4px 8px; color: {theme.on_surface};
                                font-size: 12px; cursor: pointer;
                            ",
                            if *copied_toast.read() { "Copied ✓" } else { "Copy Details" }
                        }
                    }
                }

                // Modal Footer
                div {
                    style: "
                        display: flex; justify-content: flex-end; gap: 10px;
                        padding: 14px 20px; border-top: 1px solid {theme.border};
                        background-color: {theme.surface};
                    ",
                    button {
                        onclick: move |_| {
                            is_open.set(false);
                            on_refresh_link.call(());
                        },
                        style: "
                            padding: 8px 14px; border-radius: 6px;
                            background-color: transparent; border: 1px solid {theme.border};
                            color: {theme.on_surface}; font-size: 13px; font-weight: 500;
                            cursor: pointer;
                        ",
                        "Refresh Link"
                    }
                    button {
                        onclick: move |_| {
                            is_open.set(false);
                            on_retry.call(());
                        },
                        style: "
                            padding: 8px 16px; border-radius: 6px;
                            background-color: {theme.primary}; border: none;
                            color: {theme.on_primary}; font-size: 13px; font-weight: 600;
                            cursor: pointer;
                        ",
                        "Retry Download"
                    }
                }
            }
        }
    }
}
