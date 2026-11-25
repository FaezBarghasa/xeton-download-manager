use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn MobileAddModal(
    theme: ThemePalette,
    on_close: EventHandler<()>,
    on_start: EventHandler<(String, String)>,
) -> Element {
    let mut url = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());

    rsx! {
        div {
            style: "
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background-color: rgba(0, 0, 0, 0.65); display: flex; align-items: flex-end;
                justify-content: center; z-index: 2500; animation: fadeIn 0.2s ease-out;
            ",
            onclick: move |_| on_close.call(()),
            div {
                style: "
                    width: 100%; max-width: 480px; background-color: {theme.surface};
                    border-top-left-radius: 20px; border-top-right-radius: 20px;
                    border: 1px solid {theme.border}; border-bottom: none;
                    padding: 20px 20px 32px 20px; box-sizing: border-box;
                ",
                onclick: move |e| e.stop_propagation(),
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                    span {
                        style: "font-size: 18px; font-weight: 700; color: {theme.on_surface};",
                        "New Download"
                    }
                    div {
                        style: "
                            padding: 4px 10px; border-radius: 12px; background-color: {theme.background};
                            border: 1px solid {theme.border}; font-size: 12px; font-weight: 600; color: {theme.primary};
                        ",
                        "Auto"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 14px; margin-bottom: 24px;",
                    div {
                        style: "
                            display: flex; align-items: center; gap: 10px; padding: 12px 14px;
                            background-color: {theme.background}; border: 1px solid {theme.border}; border-radius: 10px;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "18", height: "18", stroke: "{theme.on_surface_muted}",
                            fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Download link",
                            value: "{url}",
                            oninput: move |e| url.set(e.value()),
                            style: "width: 100%; border: none; background: transparent; color: {theme.on_surface}; outline: none; font-size: 14px;"
                        }
                    }
                    div {
                        style: "
                            display: flex; align-items: center; gap: 10px; padding: 12px 14px;
                            background-color: {theme.background}; border: 1px solid {theme.border}; border-radius: 10px;
                        ",
                        svg {
                            view_box: "0 0 24 24", width: "18", height: "18", stroke: "{theme.on_surface_muted}",
                            fill: "none", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                        }
                        input {
                            r#type: "text",
                            placeholder: "File name (optional)",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                            style: "width: 100%; border: none; background: transparent; color: {theme.on_surface}; outline: none; font-size: 14px;"
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 12px;",
                    button {
                        onclick: move |_| on_close.call(()),
                        style: "
                            flex: 1; height: 46px; border-radius: 12px; border: 1px solid {theme.border};
                            background-color: transparent; color: {theme.on_surface}; font-weight: 600; font-size: 14px;
                            cursor: pointer;
                        ",
                        "Cancel"
                    }
                    button {
                        onclick: move |_| {
                            let u = url.peek().clone();
                            let n = name.peek().clone();
                            if !u.is_empty() {
                                on_start.call((u, n));
                            }
                        },
                        style: "
                            flex: 1; height: 46px; border-radius: 12px; border: none;
                            background-color: {theme.primary}; color: {theme.on_primary}; font-weight: 600; font-size: 14px;
                            cursor: pointer;
                        ",
                        "OK"
                    }
                }
            }
        }
    }
}
