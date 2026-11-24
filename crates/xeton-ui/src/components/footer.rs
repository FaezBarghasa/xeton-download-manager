use dioxus::prelude::*;
use crate::theme::ThemePalette;

#[component]
pub fn Footer(
    theme: ThemePalette,
    active_count: usize,
    total_speed: u64,
) -> Element {
    let speed_formatted = if total_speed == 0 {
        "0 B/s".into()
    } else if total_speed < 1024 {
        format!("{} B/s", total_speed)
    } else if total_speed < 1024 * 1024 {
        format!("{:.1} KB/s", total_speed as f64 / 1024.0)
    } else {
        format!("{:.2} MB/s", total_speed as f64 / (1024.0 * 1024.0))
    };

    rsx! {
        div {
            class: "footer",
            style: "
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 6px 16px;
                background-color: {theme.surface};
                border-top: 1px solid {theme.border};
                font-size: 12px;
                color: {theme.on_surface};
                height: 32px;
                user-select: none;
            ",
            // Left: Speed Limiter toggle preset
            div {
                style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                span { style: "color: {theme.primary}; font-weight: 600;", "⚡ Speed Limiter:" }
                span { style: "padding: 2px 6px; border-radius: 4px; background: {theme.background}; border: 1px solid {theme.border}; font-size: 11px;", "Unlimited" }
            }
            // Right: Active & Speed indicators
            div {
                style: "display: flex; align-items: center; gap: 16px;",
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "opacity: 0.5;", "Active:" }
                    span { style: "font-weight: 600;", "{active_count}" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "opacity: 0.5;", "Speed:" }
                    span { style: "font-weight: 600; color: {theme.primary};", "{speed_formatted}" }
                }
            }
        }
    }
}
