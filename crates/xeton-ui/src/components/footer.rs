use dioxus::prelude::*;
use crate::components::icons::IconZap;
use crate::theme::ThemePalette;

#[component]
pub fn Footer(
    theme: ThemePalette,
    active_count: usize,
    total_speed: u64,
    #[props(default = "normal".to_string())] density: String,
    on_cycle_density: Option<EventHandler<()>>,
) -> Element {
    let mut speed_preset_idx = use_signal(|| 0usize);
    let presets = ["Unlimited", "10 MB/s", "5 MB/s", "1 MB/s"];
    let current_preset = presets[*speed_preset_idx.read() % presets.len()];

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
                padding: 4px 16px;
                background-color: {theme.surface};
                border-top: 1px solid {theme.border};
                font-size: 12px;
                color: {theme.on_surface};
                height: 34px;
                user-select: none;
            ",
            // Left: Speed Limiter & Density Switcher
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                button {
                    class: "xeton-btn",
                    style: "padding: 2px 8px; font-size: 11px; height: 24px; border-radius: 5px;",
                    onclick: move |_| {
                        let next = (*speed_preset_idx.peek() + 1) % presets.len();
                        speed_preset_idx.set(next);
                    },
                    IconZap { size: 12, color: Some(theme.primary.to_string()) }
                    span { style: "color: {theme.on_surface_muted};", "Limiter:" }
                    span { class: "tabular-nums font-semibold", "{current_preset}" }
                }
                if let Some(handler) = on_cycle_density {
                    button {
                        class: "xeton-btn",
                        style: "padding: 2px 8px; font-size: 11px; height: 24px; border-radius: 5px;",
                        onclick: move |_| handler.call(()),
                        span { style: "color: {theme.on_surface_muted};", "Density:" }
                        span { style: "text-transform: capitalize; font-weight: 600;", "{density}" }
                    }
                }
            }
            // Right: Active & Speed indicators
            div {
                style: "display: flex; align-items: center; gap: 16px;",
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "color: {theme.on_surface_muted};", "Active:" }
                    span { class: "tabular-nums font-semibold", "{active_count}" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "color: {theme.on_surface_muted};", "Speed:" }
                    span { class: "tabular-nums font-semibold", style: "color: {theme.primary};", "{speed_formatted}" }
                }
            }
        }
    }
}
