use dioxus::prelude::*;
use crate::theme::ThemePalette;

#[component]
pub fn MobileAddSheet(
    theme: ThemePalette,
    on_close: EventHandler<()>,
    on_select_option: EventHandler<String>, // "clipboard", "new", "batch"
) -> Element {
    rsx! {
        div {
            style: "
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background-color: rgba(0, 0, 0, 0.6); display: flex; align-items: flex-end;
                justify-content: center; z-index: 2000; animation: fadeIn 0.2s ease-out;
            ",
            onclick: move |_| on_close.call(()),
            div {
                style: "
                    width: 100%; max-width: 480px; background-color: {theme.surface};
                    border-top-left-radius: 20px; border-top-right-radius: 20px;
                    border: 1px solid {theme.border}; border-bottom: none;
                    padding: 16px 20px 28px 20px; box-sizing: border-box;
                ",
                onclick: move |e| e.stop_propagation(),
                // Handle bar
                div {
                    style: "
                        width: 40px; height: 4px; border-radius: 2px; background-color: {theme.border};
                        margin: 0 auto 16px auto;
                    "
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    // Import From Clipboard
                    button {
                        onclick: move |_| on_select_option.call("clipboard".to_string()),
                        style: "
                            display: flex; align-items: center; gap: 14px; width: 100%;
                            padding: 14px 16px; border-radius: 12px; border: none;
                            background-color: transparent; color: {theme.on_surface};
                            font-size: 15px; font-weight: 500; cursor: pointer; text-align: left;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor",
                            fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" }
                            rect { x: "8", y: "2", width: "8", height: "4", rx: "1", ry: "1" }
                        }
                        span { "Import From Clipboard" }
                    }
                    // New Download
                    button {
                        onclick: move |_| on_select_option.call("new".to_string()),
                        style: "
                            display: flex; align-items: center; gap: 14px; width: 100%;
                            padding: 14px 16px; border-radius: 12px; border: none;
                            background-color: transparent; color: {theme.on_surface};
                            font-size: 15px; font-weight: 500; cursor: pointer; text-align: left;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor",
                            fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                        span { "New Download" }
                    }
                    // Batch Download
                    button {
                        onclick: move |_| on_select_option.call("batch".to_string()),
                        style: "
                            display: flex; align-items: center; gap: 14px; width: 100%;
                            padding: 14px 16px; border-radius: 12px; border: none;
                            background-color: transparent; color: {theme.on_surface};
                            font-size: 15px; font-weight: 500; cursor: pointer; text-align: left;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor",
                            fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            polygon { points: "13 2 3 14 12 14 11 22 21 10 12 10 13 2" }
                        }
                        span { "Batch Download" }
                    }
                }
            }
        }
    }
}
