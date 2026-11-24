use dioxus::prelude::*;
use crate::theme::{ThemePalette, THEME_DARK, THEME_LIGHT, THEME_OBSIDIAN};

#[component]
pub fn SettingsModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    current_theme: Signal<ThemePalette>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut auto_start = use_signal(|| true);
    let mut speed_limit_enabled = use_signal(|| false);
    let mut default_connections = use_signal(|| 8);

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
                    width: 540px;
                    background-color: {theme.surface};
                    border: 1px solid {theme.border};
                    border-radius: 8px;
                    padding: 24px;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.5);
                    display: flex; flex-direction: column; gap: 20px;
                ",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span { style: "font-size: 18px; font-weight: 700; color: {theme.on_surface};", "Application Settings" }
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "background: transparent; border: none; color: {theme.on_surface}; cursor: pointer; font-size: 16px;",
                        "✕"
                    }
                }
                // Theme Selection
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    label { style: "font-size: 13px; font-weight: 600; color: {theme.on_surface};", "Appearance & Theme" }
                    div {
                        style: "display: flex; gap: 10px;",
                        {
                            let is_dark = theme.id == "dark";
                            let border_color = if is_dark { theme.primary } else { theme.border };
                            rsx! {
                                button {
                                    onclick: move |_| *current_theme.write() = THEME_DARK,
                                    style: "
                                        flex: 1; padding: 10px; border-radius: 6px;
                                        border: 1px solid {border_color};
                                        background-color: #1E1F22; color: #D6D6D6; font-size: 13px; font-weight: 600; cursor: pointer;
                                    ",
                                    "Dark"
                                }
                            }
                        }
                        {
                            let is_light = theme.id == "light";
                            let border_color = if is_light { theme.primary } else { theme.border };
                            rsx! {
                                button {
                                    onclick: move |_| *current_theme.write() = THEME_LIGHT,
                                    style: "
                                        flex: 1; padding: 10px; border-radius: 6px;
                                        border: 1px solid {border_color};
                                        background-color: #FFFFFF; color: #232323; font-size: 13px; font-weight: 600; cursor: pointer;
                                    ",
                                    "Light"
                                }
                            }
                        }
                        {
                            let is_obsidian = theme.id == "obsidian";
                            let border_color = if is_obsidian { theme.primary } else { theme.border };
                            rsx! {
                                button {
                                    onclick: move |_| *current_theme.write() = THEME_OBSIDIAN,
                                    style: "
                                        flex: 1; padding: 10px; border-radius: 6px;
                                        border: 1px solid {border_color};
                                        background-color: #0F1012; color: #ECECEC; font-size: 13px; font-weight: 600; cursor: pointer;
                                    ",
                                    "Obsidian"
                                }
                            }
                        }
                    }
                }
                // Network & Connections
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    label { style: "font-size: 13px; font-weight: 600; color: {theme.on_surface};", "Default Connections per Download" }
                    input {
                        r#type: "number",
                        min: "1",
                        max: "32",
                        value: "{default_connections}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                *default_connections.write() = val;
                            }
                        },
                        style: "
                            padding: 8px 12px; border-radius: 4px;
                            background-color: {theme.background};
                            border: 1px solid {theme.border};
                            color: {theme.on_background}; font-size: 13px; width: 80px;
                        "
                    }
                }
                // Autostart toggle
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 13px; color: {theme.on_surface};", "Launch on system startup" }
                    input {
                        r#type: "checkbox",
                        checked: *auto_start.read(),
                        onchange: move |e| *auto_start.write() = e.checked(),
                    }
                }
                // Speed limiter toggle
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 13px; color: {theme.on_surface};", "Enable Global Speed Limiter" }
                    input {
                        r#type: "checkbox",
                        checked: *speed_limit_enabled.read(),
                        onchange: move |e| *speed_limit_enabled.write() = e.checked(),
                    }
                }
                // Footer buttons
                div {
                    style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 10px;",
                    button {
                        onclick: move |_| *is_open.write() = false,
                        style: "
                            padding: 8px 18px; border-radius: 4px;
                            background-color: {theme.primary}; border: none;
                            color: {theme.on_primary}; font-weight: 600; cursor: pointer;
                        ",
                        "Done"
                    }
                }
            }
        }
    }
}
