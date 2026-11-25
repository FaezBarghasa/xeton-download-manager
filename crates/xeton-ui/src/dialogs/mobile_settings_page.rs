use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[component]
pub fn MobileSettingsPage(theme: ThemePalette, on_back: EventHandler<()>) -> Element {
    let mut show_average_speed = use_signal(|| true);
    let mut notification_enabled = use_signal(|| true);
    let mut speed_limiter_enabled = use_signal(|| false);

    rsx! {
        div {
            style: "
                position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
                background-color: {theme.background}; color: {theme.on_background};
                display: flex; flex-direction: column; z-index: 2500;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            ",
            // Navigation Top Bar
            div {
                style: "
                    height: 56px; display: flex; align-items: center; gap: 16px;
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
                span {
                    style: "font-size: 18px; font-weight: 700; color: {theme.on_surface};",
                    "Settings"
                }
            }

            // Scrollable Content
            div {
                style: "flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px;",
                // Theme Card
                div {
                    style: "background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 16px;",
                    div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface}; margin-bottom: 4px;", "Theme" }
                    div { style: "font-size: 13px; color: {theme.on_surface_muted}; margin-bottom: 12px;", "Dark (Default)" }
                    div {
                        style: "display: flex; gap: 8px;",
                        div { style: "width: 24px; height: 24px; border-radius: 50%; background-color: #1E1F22; border: 2px solid {theme.primary};" }
                        div { style: "width: 24px; height: 24px; border-radius: 50%; background-color: #F8F9FA; border: 1px solid {theme.border};" }
                    }
                }

                // Average Speed Toggle Card
                div {
                    style: "background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 16px; display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface};", "Average Speed" }
                        div { style: "font-size: 13px; color: {theme.on_surface_muted};", "Calculate speed as a rolling average" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: *show_average_speed.read(),
                        onchange: move |e| show_average_speed.set(e.checked()),
                        style: "width: 20px; height: 20px; cursor: pointer; accent-color: {theme.primary};"
                    }
                }

                // Notifications Toggle Card
                div {
                    style: "background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 16px; display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface};", "Show Notifications" }
                        div { style: "font-size: 13px; color: {theme.on_surface_muted};", "Notify on completed and failed downloads" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: *notification_enabled.read(),
                        onchange: move |e| notification_enabled.set(e.checked()),
                        style: "width: 20px; height: 20px; cursor: pointer; accent-color: {theme.primary};"
                    }
                }

                // Speed Limiter Card
                div {
                    style: "background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 16px; display: flex; justify-content: space-between; align-items: center;",
                    div {
                        div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface};", "Speed Limiter" }
                        div { style: "font-size: 13px; color: {theme.on_surface_muted};", "Throttle download bandwidth" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: *speed_limiter_enabled.read(),
                        onchange: move |e| speed_limiter_enabled.set(e.checked()),
                        style: "width: 20px; height: 20px; cursor: pointer; accent-color: {theme.primary};"
                    }
                }

                // Storage Folder Card
                div {
                    style: "background-color: {theme.surface}; border: 1px solid {theme.border}; border-radius: 12px; padding: 16px;",
                    div { style: "font-size: 15px; font-weight: 600; color: {theme.on_surface}; margin-bottom: 4px;", "Default Download Folder" }
                    div { style: "font-size: 13px; color: {theme.on_surface_muted}; word-break: break-all;", "/sdcard/Download" }
                }
            }
        }
    }
}
