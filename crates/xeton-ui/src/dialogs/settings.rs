use crate::theme::{THEME_DARK, THEME_LIGHT, THEME_OBSIDIAN, ThemePalette};
use dioxus::prelude::*;

#[component]
pub fn SettingsModal(
    theme: ThemePalette,
    is_open: Signal<bool>,
    current_theme: Signal<ThemePalette>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    let mut active_tab = use_signal(|| "appearance");

    // General & Appearance
    let mut language = use_signal(|| "en".to_string());
    let mut auto_start = use_signal(|| true);
    let mut start_minimized = use_signal(|| false);
    let mut close_to_tray = use_signal(|| true);

    // Engine & Network
    let mut default_connections = use_signal(|| 8u32);
    let mut speed_limit_enabled = use_signal(|| false);
    let mut global_speed_limit_kb = use_signal(|| 0u64);
    let mut socket_timeout_secs = use_signal(|| 30u32);
    let mut retry_count = use_signal(|| 5u32);

    // Downloads & Storage
    let mut default_download_dir = use_signal(|| "~/Downloads".to_string());
    let mut auto_categorize = use_signal(|| true);
    let mut create_subfolders = use_signal(|| true);
    let mut delete_temp_on_fail = use_signal(|| false);

    // Browser Integration
    let mut browser_integration_port = use_signal(|| 18512u16);
    let mut capture_downloads = use_signal(|| true);

    // System & Power
    let mut prevent_sleep = use_signal(|| true);
    let mut notification_sound = use_signal(|| true);

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| *is_open.write() = false,

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-3xl overflow-hidden flex flex-col text-on-surface text-xs select-none",
                onclick: move |e| e.stop_propagation(),

                // Dialog Header
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-b border-border bg-surface-variant/30",
                    div {
                        class: "flex items-center gap-2.5",
                        svg {
                            class: "w-5 h-5 text-primary",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                            }
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                            }
                        }
                        span { class: "font-semibold text-sm", "Preferences & Settings" }
                    }
                    button {
                        class: "p-1 rounded-md text-on-surface/50 hover:text-on-surface hover:bg-surface-variant/50 transition-colors",
                        onclick: move |_| *is_open.write() = false,
                        svg {
                            class: "w-4 h-4",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M6 18L18 6M6 6l12 12"
                            }
                        }
                    }
                }

                // Main Layout: Sidebar Tabs & Tab Content
                div {
                    class: "grid grid-cols-12 min-h-[420px] max-h-[500px] divide-x divide-border",

                    // Left Nav Tabs
                    div {
                        class: "col-span-4 p-3 bg-surface-variant/10 space-y-1",
                        button {
                            class: format!(
                                "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left transition-colors {}",
                                if active_tab() == "appearance" { "bg-primary text-on-primary font-medium shadow-sm" } else { "text-on-surface/80 hover:bg-surface-variant/50" }
                            ),
                            onclick: move |_| active_tab.set("appearance"),
                            span { "🎨" }
                            span { "Appearance & UI" }
                        }
                        button {
                            class: format!(
                                "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left transition-colors {}",
                                if active_tab() == "engine" { "bg-primary text-on-primary font-medium shadow-sm" } else { "text-on-surface/80 hover:bg-surface-variant/50" }
                            ),
                            onclick: move |_| active_tab.set("engine"),
                            span { "⚡" }
                            span { "Engine & Network" }
                        }
                        button {
                            class: format!(
                                "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left transition-colors {}",
                                if active_tab() == "storage" { "bg-primary text-on-primary font-medium shadow-sm" } else { "text-on-surface/80 hover:bg-surface-variant/50" }
                            ),
                            onclick: move |_| active_tab.set("storage"),
                            span { "📁" }
                            span { "Downloads & Storage" }
                        }
                        button {
                            class: format!(
                                "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left transition-colors {}",
                                if active_tab() == "integration" { "bg-primary text-on-primary font-medium shadow-sm" } else { "text-on-surface/80 hover:bg-surface-variant/50" }
                            ),
                            onclick: move |_| active_tab.set("integration"),
                            span { "🌐" }
                            span { "Browser Integration" }
                        }
                        button {
                            class: format!(
                                "w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left transition-colors {}",
                                if active_tab() == "power" { "bg-primary text-on-primary font-medium shadow-sm" } else { "text-on-surface/80 hover:bg-surface-variant/50" }
                            ),
                            onclick: move |_| active_tab.set("power"),
                            span { "🔌" }
                            span { "System & Power" }
                        }
                    }

                    // Tab Content Body
                    div {
                        class: "col-span-8 p-6 overflow-y-auto space-y-5",

                        match active_tab() {
                            "appearance" => rsx! {
                                div {
                                    class: "space-y-4",
                                    div {
                                        label { class: "block font-medium text-on-surface mb-2", "Theme Selection" }
                                        div {
                                            class: "grid grid-cols-3 gap-3",
                                            button {
                                                class: format!(
                                                    "p-3 rounded-lg border text-center font-medium transition-all {}",
                                                    if theme.id == "dark" { "border-primary bg-primary/10 text-primary ring-1 ring-primary" } else { "border-border bg-surface-variant/30 text-on-surface hover:bg-surface-variant/50" }
                                                ),
                                                onclick: move |_| *current_theme.write() = THEME_DARK,
                                                "Dark (Default)"
                                            }
                                            button {
                                                class: format!(
                                                    "p-3 rounded-lg border text-center font-medium transition-all {}",
                                                    if theme.id == "light" { "border-primary bg-primary/10 text-primary ring-1 ring-primary" } else { "border-border bg-surface-variant/30 text-on-surface hover:bg-surface-variant/50" }
                                                ),
                                                onclick: move |_| *current_theme.write() = THEME_LIGHT,
                                                "Light"
                                            }
                                            button {
                                                class: format!(
                                                    "p-3 rounded-lg border text-center font-medium transition-all {}",
                                                    if theme.id == "obsidian" { "border-primary bg-primary/10 text-primary ring-1 ring-primary" } else { "border-border bg-surface-variant/30 text-on-surface hover:bg-surface-variant/50" }
                                                ),
                                                onclick: move |_| *current_theme.write() = THEME_OBSIDIAN,
                                                "Obsidian"
                                            }
                                        }
                                    }

                                    div {
                                        label { class: "block font-medium text-on-surface mb-1.5", "Interface Language" }
                                        select {
                                            class: "w-full bg-background border border-border rounded-lg px-3 py-2 text-xs text-on-surface focus:outline-none focus:border-primary",
                                            value: "{language}",
                                            onchange: move |e| language.set(e.value()),
                                            option { value: "en", "English (US)" }
                                            option { value: "fa", "Persian (فارسی)" }
                                            option { value: "es", "Spanish (Español)" }
                                            option { value: "de", "German (Deutsch)" }
                                            option { value: "zh", "Chinese (简体中文)" }
                                            option { value: "ru", "Russian (Русский)" }
                                        }
                                    }

                                    div {
                                        class: "space-y-2.5 pt-2 border-t border-border",
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Launch Xeton on system startup" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: auto_start(),
                                                onchange: move |e| auto_start.set(e.checked()),
                                            }
                                        }
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Start minimized to system tray" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: start_minimized(),
                                                onchange: move |e| start_minimized.set(e.checked()),
                                            }
                                        }
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Close window to system tray instead of exiting" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: close_to_tray(),
                                                onchange: move |e| close_to_tray.set(e.checked()),
                                            }
                                        }
                                    }
                                }
                            },
                            "engine" => rsx! {
                                div {
                                    class: "space-y-4",
                                    div {
                                        label { class: "block font-medium text-on-surface mb-1.5", "Default Multi-Segment Threads per Download" }
                                        input {
                                            r#type: "number",
                                            min: "1",
                                            max: "32",
                                            class: "w-28 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                            value: "{default_connections}",
                                            oninput: move |e| {
                                                if let Ok(val) = e.value().parse::<u32>() {
                                                    default_connections.set(val.clamp(1, 32));
                                                }
                                            }
                                        }
                                        div { class: "text-[11px] text-on-surface/50 mt-1", "Supersonic multi-part chunking. Up to 32 threads supported." }
                                    }

                                    div {
                                        class: "border border-border rounded-xl p-4 bg-surface-variant/20 space-y-3",
                                        div {
                                            class: "flex items-center justify-between",
                                            span { class: "font-medium text-on-surface", "Global Speed Limiter" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: speed_limit_enabled(),
                                                onchange: move |e| speed_limit_enabled.set(e.checked()),
                                            }
                                        }
                                        if speed_limit_enabled() {
                                            div {
                                                class: "flex items-center gap-3 pt-2",
                                                input {
                                                    r#type: "number",
                                                    min: "0",
                                                    step: "128",
                                                    placeholder: "e.g. 2048",
                                                    class: "w-36 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                                    value: "{global_speed_limit_kb}",
                                                    oninput: move |e| {
                                                        if let Ok(val) = e.value().parse::<u64>() {
                                                            global_speed_limit_kb.set(val);
                                                        }
                                                    }
                                                }
                                                span { class: "text-on-surface/60", "KB/s (0 = unlimited)" }
                                            }
                                        }
                                    }

                                    div {
                                        class: "grid grid-cols-2 gap-4",
                                        div {
                                            label { class: "block font-medium text-on-surface mb-1.5", "Connection Timeout (seconds)" }
                                            input {
                                                r#type: "number",
                                                min: "5",
                                                max: "120",
                                                class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                                value: "{socket_timeout_secs}",
                                                oninput: move |e| {
                                                    if let Ok(val) = e.value().parse::<u32>() {
                                                        socket_timeout_secs.set(val);
                                                    }
                                                }
                                            }
                                        }
                                        div {
                                            label { class: "block font-medium text-on-surface mb-1.5", "Max Network Retries" }
                                            input {
                                                r#type: "number",
                                                min: "0",
                                                max: "20",
                                                class: "w-full bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                                value: "{retry_count}",
                                                oninput: move |e| {
                                                    if let Ok(val) = e.value().parse::<u32>() {
                                                        retry_count.set(val);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "storage" => rsx! {
                                div {
                                    class: "space-y-4",
                                    div {
                                        label { class: "block font-medium text-on-surface mb-1.5", "Default Download Directory" }
                                        div {
                                            class: "flex gap-2",
                                            input {
                                                class: "flex-1 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface focus:outline-none focus:border-primary",
                                                value: "{default_download_dir}",
                                                oninput: move |e| default_download_dir.set(e.value()),
                                            }
                                            button {
                                                class: "px-3 py-1.5 bg-surface-variant text-on-surface hover:bg-surface-variant/80 rounded-lg text-xs font-medium border border-border",
                                                "Browse"
                                            }
                                        }
                                    }

                                    div {
                                        class: "space-y-2.5 pt-2 border-t border-border",
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Automatically assign category based on file extension" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: auto_categorize(),
                                                onchange: move |e| auto_categorize.set(e.checked()),
                                            }
                                        }
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Create category subfolders inside download directory" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: create_subfolders(),
                                                onchange: move |e| create_subfolders.set(e.checked()),
                                            }
                                        }
                                        label {
                                            class: "flex items-center justify-between cursor-pointer py-1",
                                            span { class: "text-on-surface", "Purge incomplete part files on permanent download cancellation" }
                                            input {
                                                r#type: "checkbox",
                                                class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                                checked: delete_temp_on_fail(),
                                                onchange: move |e| delete_temp_on_fail.set(e.checked()),
                                            }
                                        }
                                    }
                                }
                            },
                            "integration" => rsx! {
                                div {
                                    class: "space-y-4",
                                    div {
                                        class: "p-3 bg-primary/10 border border-primary/20 rounded-lg text-primary text-[11px] leading-relaxed",
                                        "Xeton includes a lightweight local HTTP/WebSocket IPC server for Chrome, Firefox, Edge, and Brave extensions."
                                    }
                                    div {
                                        label { class: "block font-medium text-on-surface mb-1.5", "Browser Bridge Local Port" }
                                        input {
                                            r#type: "number",
                                            class: "w-28 bg-background border border-border rounded-lg px-3 py-1.5 text-xs text-on-surface font-mono focus:outline-none focus:border-primary",
                                            value: "{browser_integration_port}",
                                            oninput: move |e| {
                                                if let Ok(val) = e.value().parse::<u16>() {
                                                    browser_integration_port.set(val);
                                                }
                                            }
                                        }
                                    }
                                    label {
                                        class: "flex items-center justify-between cursor-pointer py-1",
                                        span { class: "text-on-surface", "Automatically intercept browser download triggers" }
                                        input {
                                            r#type: "checkbox",
                                            class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                            checked: capture_downloads(),
                                            onchange: move |e| capture_downloads.set(e.checked()),
                                        }
                                    }
                                }
                            },
                            _ => rsx! {
                                div {
                                    class: "space-y-4",
                                    label {
                                        class: "flex items-center justify-between cursor-pointer py-1",
                                        span { class: "text-on-surface", "Prevent computer from sleeping while downloads are running" }
                                        input {
                                            r#type: "checkbox",
                                            class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                            checked: prevent_sleep(),
                                            onchange: move |e| prevent_sleep.set(e.checked()),
                                        }
                                    }
                                    label {
                                        class: "flex items-center justify-between cursor-pointer py-1",
                                        span { class: "text-on-surface", "Play notification sound upon download completion" }
                                        input {
                                            r#type: "checkbox",
                                            class: "rounded border-border text-primary focus:ring-0 cursor-pointer w-4 h-4",
                                            checked: notification_sound(),
                                            onchange: move |e| notification_sound.set(e.checked()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Dialog Footer
                div {
                    class: "flex items-center justify-end gap-2.5 px-6 py-3.5 border-t border-border bg-surface-variant/20",
                    button {
                        class: "px-4 py-1.5 bg-surface-variant hover:bg-surface-variant/80 text-on-surface rounded-lg text-xs font-medium transition-colors",
                        onclick: move |_| *is_open.write() = false,
                        "Cancel"
                    }
                    button {
                        class: "px-5 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors",
                        onclick: move |_| {
                            *is_open.write() = false;
                        },
                        "Save Preferences"
                    }
                }
            }
        }
    }
}
