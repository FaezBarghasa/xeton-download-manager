use dioxus::prelude::*;
use crate::theme::ThemePalette;

#[component]
pub fn QRCodeModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    title: String,
    data: String,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut copied = use_signal(|| false);

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
                    width: 400px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    display: flex; flex-direction: column;
                    box-shadow: 0 12px 32px rgba(0,0,0,0.5);
                    overflow: hidden; text-align: center;
                ",
                // Header
                div {
                    style: "
                        display: flex; align-items: center; justify-content: space-between;
                        padding: 14px 20px; border-bottom: 1px solid {theme.border};
                    ",
                    span {
                        style: "font-weight: 600; font-size: 15px; color: {theme.on_surface};",
                        "{title}"
                    }
                    button {
                        onclick: move |_| is_open.set(false),
                        style: "background: transparent; border: none; color: {theme.on_surface_muted}; cursor: pointer; font-size: 16px;",
                        "✕"
                    }
                }

                // Body with QR Code Placeholder / SVG Frame
                div {
                    style: "padding: 24px; display: flex; flex-direction: column; align-items: center; gap: 16px;",
                    div {
                        style: "
                            width: 200px; height: 200px; background-color: #FFFFFF;
                            border-radius: 8px; padding: 12px; box-sizing: border-box;
                            display: flex; align-items: center; justify-content: center;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                        ",
                        // Procedural Clean QR Mockup SVG
                        svg {
                            view_box: "0 0 100 100", width: "176", height: "176",
                            rect { x: "0", y: "0", width: "30", height: "30", fill: "#1E1F22" }
                            rect { x: "6", y: "6", width: "18", height: "18", fill: "#FFFFFF" }
                            rect { x: "10", y: "10", width: "10", height: "10", fill: "#1E1F22" }

                            rect { x: "70", y: "0", width: "30", height: "30", fill: "#1E1F22" }
                            rect { x: "76", y: "6", width: "18", height: "18", fill: "#FFFFFF" }
                            rect { x: "80", y: "10", width: "10", height: "10", fill: "#1E1F22" }

                            rect { x: "0", y: "70", width: "30", height: "30", fill: "#1E1F22" }
                            rect { x: "6", y: "76", width: "18", height: "18", fill: "#FFFFFF" }
                            rect { x: "10", y: "80", width: "10", height: "10", fill: "#1E1F22" }

                            rect { x: "36", y: "10", width: "8", height: "8", fill: "#1E1F22" }
                            rect { x: "50", y: "10", width: "12", height: "8", fill: "#1E1F22" }
                            rect { x: "36", y: "36", width: "28", height: "28", fill: "#4791BF" }
                            rect { x: "72", y: "40", width: "16", height: "8", fill: "#1E1F22" }
                            rect { x: "10", y: "40", width: "16", height: "8", fill: "#1E1F22" }
                            rect { x: "40", y: "76", width: "24", height: "12", fill: "#1E1F22" }
                            rect { x: "76", y: "76", width: "14", height: "14", fill: "#1E1F22" }
                        }
                    }

                    div {
                        style: "font-size: 13px; color: {theme.on_surface_muted}; max-width: 300px; word-break: break-all;",
                        "{data}"
                    }

                    button {
                        onclick: move |_| {
                            copied.set(true);
                        },
                        style: "
                            padding: 8px 16px; border-radius: 6px;
                            background-color: transparent; border: 1px solid {theme.border};
                            color: {theme.on_surface}; font-size: 13px; cursor: pointer;
                        ",
                        if *copied.read() { "Link Copied ✓" } else { "Copy Link Address" }
                    }
                }

                // Footer
                div {
                    style: "
                        display: flex; justify-content: center;
                        padding: 12px; border-top: 1px solid {theme.border};
                        background-color: {theme.surface};
                    ",
                    button {
                        onclick: move |_| is_open.set(false),
                        style: "
                            padding: 6px 20px; border-radius: 6px;
                            background-color: {theme.primary}; border: none;
                            color: {theme.on_primary}; font-size: 13px; font-weight: 600;
                            cursor: pointer;
                        ",
                        "Done"
                    }
                }
            }
        }
    }
}
