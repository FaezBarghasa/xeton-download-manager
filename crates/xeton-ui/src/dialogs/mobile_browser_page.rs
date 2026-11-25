use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn MobileBrowserPage(theme: ThemePalette, on_back: EventHandler<()>) -> Element {
    let mut current_url = use_signal(|| String::new());
    let mut tab_count = use_signal(|| 1);

    rsx! {
        div {
            style: "
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background-color: {theme.background}; color: {theme.on_background};
                display: flex; flex-direction: column; justify-content: space-between; z-index: 2500;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            ",
            // Browser Address / Top Bar
            div {
                style: "
                    height: 56px; display: flex; align-items: center; gap: 10px;
                    padding: 0 16px; background-color: {theme.surface};
                    border-bottom: 1px solid {theme.border};
                ",
                button {
                    onclick: move |_| on_back.call(()),
                    style: "
                        width: 36px; height: 36px; border-radius: 50%; border: none;
                        background: transparent; color: {theme.on_surface};
                        display: flex; align-items: center; justify-content: center; cursor: pointer;
                    ",
                    svg {
                        view_box: "0 0 24 24", width: "22", height: "22", stroke: "currentColor",
                        fill: "none", stroke_width: "2.5", stroke_linecap: "round", stroke_linejoin: "round",
                        line { x1: "19", y1: "12", x2: "5", y2: "12" }
                        polyline { points: "12 19 5 12 12 5" }
                    }
                }
                div {
                    style: "
                        flex: 1; height: 38px; background-color: {theme.background};
                        border: 1px solid {theme.border}; border-radius: 20px;
                        display: flex; align-items: center; padding: 0 14px;
                    ",
                    input {
                        r#type: "text",
                        placeholder: "Search or type URL",
                        value: "{current_url}",
                        oninput: move |e| current_url.set(e.value()),
                        style: "
                            width: 100%; border: none; background: transparent;
                            color: {theme.on_surface}; font-size: 14px; outline: none;
                        "
                    }
                }
            }

            // Main Webview Placeholder / No Tabs State
            div {
                style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 20px;",
                p {
                    style: "font-size: 16px; color: {theme.on_surface_muted}; margin-bottom: 24px;",
                    "No tabs are open"
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 12px; width: 100%; max-width: 260px;",
                    button {
                        style: "
                            width: 100%; height: 46px; border-radius: 24px; border: 1px solid {theme.border};
                            background-color: {theme.surface}; color: {theme.on_surface};
                            font-size: 14px; font-weight: 600; cursor: pointer;
                        ",
                        "Paste And Go"
                    }
                    button {
                        style: "
                            width: 100%; height: 46px; border-radius: 24px; border: 1px solid {theme.border};
                            background-color: {theme.surface}; color: {theme.on_surface};
                            font-size: 14px; font-weight: 600; cursor: pointer;
                        ",
                        "Bookmarks"
                    }
                }
            }

            // Bottom Browser Controls
            div {
                style: "
                    height: 52px; display: flex; align-items: center; justify-content: space-around;
                    padding: 0 8px; background-color: {theme.surface}; border-top: 1px solid {theme.border};
                ",
                button {
                    style: "background: transparent; border: none; color: {theme.on_surface_muted}; cursor: pointer;",
                    svg {
                        view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", fill: "none", stroke_width: "2",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                button {
                    style: "background: transparent; border: none; color: {theme.on_surface_muted}; cursor: pointer;",
                    svg {
                        view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", fill: "none", stroke_width: "2",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
                button {
                    style: "background: transparent; border: none; color: {theme.on_surface}; cursor: pointer;",
                    svg {
                        view_box: "0 0 24 24", width: "20", height: "20", stroke: "currentColor", fill: "none", stroke_width: "2",
                        polyline { points: "23 4 23 10 17 10" }
                        path { d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                    }
                }
                // Tabs Count
                div {
                    style: "
                        width: 22px; height: 22px; border: 2px solid {theme.on_surface}; border-radius: 4px;
                        display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: 700;
                    ",
                    "{tab_count}"
                }
            }
        }
    }
}
