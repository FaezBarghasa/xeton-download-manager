use dioxus::prelude::*;
use crate::components::icons::IconXetonLogo;

#[component]
pub fn AboutModal(
    on_close: EventHandler<()>,
) -> Element {
    let mut active_tab = use_signal(|| "about");

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-surface border border-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col text-on-surface text-xs select-none",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-b border-border bg-surface-variant/30",
                    div {
                        class: "flex items-center gap-2",
                        IconXetonLogo { class: "w-5 h-5 text-primary" }
                        span { class: "font-semibold text-sm", "About Xeton Download Manager" }
                    }
                    button {
                        class: "p-1 rounded-md text-on-surface/50 hover:text-on-surface hover:bg-surface-variant/50 transition-colors",
                        onclick: move |_| on_close.call(()),
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

                // Sub-Tabs
                div {
                    class: "flex items-center border-b border-border px-5 bg-surface-variant/10 text-xs font-medium text-on-surface/60",
                    button {
                        class: format!(
                            "py-2.5 px-3 border-b-2 transition-colors {}",
                            if active_tab() == "about" { "border-primary text-primary" } else { "border-transparent hover:text-on-surface" }
                        ),
                        onclick: move |_| active_tab.set("about"),
                        "Overview"
                    }
                    button {
                        class: format!(
                            "py-2.5 px-3 border-b-2 transition-colors {}",
                            if active_tab() == "credits" { "border-primary text-primary" } else { "border-transparent hover:text-on-surface" }
                        ),
                        onclick: move |_| active_tab.set("credits"),
                        "Credits & Contributors"
                    }
                    button {
                        class: format!(
                            "py-2.5 px-3 border-b-2 transition-colors {}",
                            if active_tab() == "license" { "border-primary text-primary" } else { "border-transparent hover:text-on-surface" }
                        ),
                        onclick: move |_| active_tab.set("license"),
                        "Open Source Licenses"
                    }
                }

                // Content
                div {
                    class: "p-6 min-h-[260px] max-h-[360px] overflow-y-auto",
                    match active_tab() {
                        "about" => rsx! {
                            div {
                                class: "flex flex-col items-center text-center space-y-3",
                                IconXetonLogo { class: "w-16 h-16 drop-shadow-md" }
                                div {
                                    div { class: "text-base font-bold tracking-tight text-on-surface", "Xeton Download Manager" }
                                    div { class: "text-xs text-primary font-mono font-medium mt-0.5", "Version 0.9.0 (Desktop & Android)" }
                                    div { class: "text-[11px] text-on-surface/50 mt-1", "Next-Gen Ultra-Fast Multi-Threaded Download Accelerator" }
                                }
                                div {
                                    class: "text-xs text-on-surface/70 max-w-sm leading-relaxed pt-2",
                                    "Engineered in Rust (Tauri v2 + Dioxus v0.7) for maximum memory efficiency, zero CPU bloat, and supersonic chunk assembly."
                                }
                                div {
                                    class: "pt-4 flex items-center justify-center gap-4 text-xs font-medium text-primary",
                                    a {
                                        class: "hover:underline cursor-pointer",
                                        "GitHub Repository"
                                    }
                                    span { class: "text-on-surface/20", "•" }
                                    a {
                                        class: "hover:underline cursor-pointer",
                                        "Report Issue"
                                    }
                                    span { class: "text-on-surface/20", "•" }
                                    a {
                                        class: "hover:underline cursor-pointer",
                                        "Browser Extension"
                                    }
                                }
                            }
                        },
                        "credits" => rsx! {
                            div {
                                class: "space-y-4",
                                div {
                                    div { class: "font-semibold text-xs text-on-surface", "Engineering & Architecture" }
                                    div { class: "text-on-surface/70 mt-1", "Faez Barghasa (@FaezBarghasa) - Lead Systems Architect" }
                                }
                                div {
                                    div { class: "font-semibold text-xs text-on-surface", "UI & Design Direction" }
                                    div { class: "text-on-surface/70 mt-1", "Antigravity Agentic Studio & Xeton Project Contributors" }
                                }
                                div {
                                    div { class: "font-semibold text-xs text-on-surface", "Upstream Inspiration" }
                                    div { class: "text-on-surface/70 mt-1", "AB Download Manager (open source community)" }
                                }
                            }
                        },
                        _ => rsx! {
                            div {
                                class: "space-y-3 font-mono text-[11px] text-on-surface/70 leading-relaxed",
                                div { class: "font-semibold text-on-surface", "Apache License 2.0 / MIT Dual License" }
                                p {
                                    "Copyright (c) 2026 Faez Barghasa. All rights reserved."
                                }
                                p {
                                    "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files..."
                                }
                                div {
                                    class: "pt-2 font-medium text-on-surface/90",
                                    "Third-Party Ecosystem: Tauri v2, Dioxus v0.7, Tokio, Reqwest, OpenSSL, Wry, Tao."
                                }
                            }
                        }
                    }
                }

                // Footer
                div {
                    class: "flex items-center justify-between px-5 py-3.5 border-t border-border bg-surface-variant/20",
                    div {
                        class: "text-[11px] text-on-surface/40",
                        "© 2026 Xeton Project"
                    }
                    button {
                        class: "px-5 py-1.5 bg-primary hover:bg-primary-hover text-on-primary rounded-lg text-xs font-medium shadow-sm transition-colors",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
