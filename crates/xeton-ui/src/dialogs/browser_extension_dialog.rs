use crate::theme::ThemePalette;
use dioxus::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ExtensionInfo {
    pub success: bool,
    pub extension_path: String,
    pub chrome_host_manifest: Option<String>,
    pub firefox_host_manifest: Option<String>,
    pub message: String,
}

#[component]
pub fn BrowserExtensionDialog(theme: ThemePalette, on_close: EventHandler<()>) -> Element {
    let mut status_message = use_signal(|| "Loading extension details...".to_string());
    let mut extension_path = use_signal(|| "".to_string());
    let mut is_installed = use_signal(|| false);
    let mut is_working = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            if let Ok(info) =
                crate::tauri_bindings::invoke_no_args::<ExtensionInfo>("get_extension_info").await
            {
                extension_path.set(info.extension_path);
                status_message.set("Ready to install or reload into your browser.".to_string());
            } else {
                status_message.set("Extension folder ready.".to_string());
            }
        });
    });

    let on_install = move |_| {
        is_working.set(true);
        spawn(async move {
            match crate::tauri_bindings::invoke_no_args::<ExtensionInfo>(
                "install_browser_extension",
            )
            .await
            {
                Ok(res) => {
                    is_installed.set(true);
                    status_message.set(
                        "Native messaging manifests installed for Chrome & Firefox!".to_string(),
                    );
                }
                Err(e) => {
                    status_message.set(format!("Installation failed: {}", e));
                }
            }
            is_working.set(false);
        });
    };

    let on_open_browser = move |browser: &'static str| {
        spawn(async move {
            let _ = crate::tauri_bindings::invoke::<(), _>(
                "open_extension_in_browser",
                &serde_json::json!({ "browser": browser }),
            )
            .await;
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-in fade-in duration-150",
            onclick: move |_| on_close.call(()),
            div {
                class: "w-[560px] max-w-[94vw] bg-surface rounded-xl shadow-2xl border border-white/10 overflow-hidden flex flex-col animate-in zoom-in-95 duration-150",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex items-center justify-between px-6 py-4 border-b border-white/10 bg-surface-variant/40",
                    div {
                        class: "flex items-center gap-3",
                        div { class: "w-2.5 h-2.5 rounded-full bg-primary shadow-[0_0_10px_var(--primary)]" }
                        h3 { class: "font-bold text-base text-on-surface", "Browser Extension Integration" }
                    }
                    button {
                        class: "text-on-surface/50 hover:text-on-surface transition-colors p-1 rounded-md text-lg",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Content
                div {
                    class: "p-6 space-y-5 text-sm text-on-surface/80",
                    div {
                        class: "p-3.5 bg-primary/10 border border-primary/20 rounded-lg flex items-center gap-3",
                        div { class: "text-primary text-xl font-bold", "⚡" }
                        div {
                            div { class: "font-semibold text-primary text-xs uppercase tracking-wider", "One-Click Interception & Sniffer" }
                            div { class: "text-xs text-on-surface/70 mt-0.5", "Automatically catches downloads, injects the Video Sniffer pill, and provides page batch scanning." }
                        }
                    }

                    div {
                        class: "space-y-1.5",
                        label { class: "text-xs font-semibold text-on-surface/60 uppercase tracking-wider", "Extension Source Directory" }
                        div {
                            class: "font-mono text-xs bg-black/30 p-2.5 rounded border border-white/5 break-all select-all text-on-surface/90",
                            "{extension_path}"
                        }
                    }

                    div {
                        class: "space-y-2.5",
                        div { class: "text-xs font-semibold text-on-surface/60 uppercase tracking-wider", "Launch & Auto-Load into Browsers" }
                        div {
                            class: "grid grid-cols-2 gap-3",
                            button {
                                class: "xeton-btn flex items-center justify-center gap-2 py-2.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 transition-colors font-medium text-xs",
                                onclick: move |_| on_open_browser("chrome"),
                                "🌐 Google Chrome"
                            }
                            button {
                                class: "xeton-btn flex items-center justify-center gap-2 py-2.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 transition-colors font-medium text-xs",
                                onclick: move |_| on_open_browser("brave"),
                                "🦁 Brave Browser"
                            }
                            button {
                                class: "xeton-btn flex items-center justify-center gap-2 py-2.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 transition-colors font-medium text-xs",
                                onclick: move |_| on_open_browser("chromium"),
                                "🔵 Chromium"
                            }
                            button {
                                class: "xeton-btn flex items-center justify-center gap-2 py-2.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 transition-colors font-medium text-xs",
                                onclick: move |_| on_open_browser("firefox"),
                                "🦊 Mozilla Firefox"
                            }
                        }
                    }

                    div {
                        class: "text-xs text-on-surface/60 font-mono bg-white/[0.02] p-2.5 rounded border border-white/5 flex items-center gap-2",
                        span { class: "inline-block w-2 h-2 rounded-full", style: if *is_installed.read() { "background: #10B981;" } else { "background: #F59E0B;" } }
                        "{status_message}"
                    }
                }

                // Footer
                div {
                    class: "flex items-center justify-between px-6 py-3.5 border-t border-white/10 bg-surface-variant/20",
                    button {
                        class: "text-xs text-on-surface/60 hover:text-on-surface underline transition-colors",
                        onclick: move |_| on_open_browser("folder"),
                        "Open Extension Folder in File Manager"
                    }
                    div {
                        class: "flex items-center gap-3",
                        button {
                            class: "px-4 py-2 text-xs font-semibold rounded-lg bg-primary hover:bg-primary/90 text-white shadow-lg transition-all",
                            disabled: *is_working.read(),
                            onclick: on_install,
                            if *is_working.read() { "Configuring..." } else { "Install Native Host Manifests" }
                        }
                    }
                }
            }
        }
    }
}
